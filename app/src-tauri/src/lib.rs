//! KikoeLive desktop backend (Tauri 2).
//!
//! The frontend can only call the commands registered below; each validates its
//! input (language codes, catalog model ids, existing file paths) and none of them
//! runs shell commands or opens network listeners. The only process KikoeLive
//! starts is its own bundled translation worker (`kikoe-mt`), from a fixed path.

mod models;
mod settings;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kikoe_asr::{SileroVad, WhisperAsr, WhisperConfig};
use kikoe_core::audio::{CaptureError, SourceInfo};
use kikoe_core::clock::PipelineClock;
use kikoe_core::lang::{LanguageControl, TargetLanguage};
use kikoe_core::models::{DEFAULT_VAD_ID, ModelKind, default_translator};
use kikoe_core::pipeline::{Engines, EventSink, PipelineConfig, PipelineEvent, PipelineHandle};
use kikoe_core::translate::PromptStyle;
use kikoe_core::worker::{WorkerConfig, WorkerModel, WorkerTranslator};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};

use crate::models::ModelState;
use crate::settings::{OverlaySettings, Settings};

const EVT_PIPELINE: &str = "kikoe://pipeline";
const EVT_APP: &str = "kikoe://app";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Idle,
    Loading,
    WaitingPermission,
    Running,
    Stopping,
    Error,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AppEvent {
    Phase { phase: Phase, detail: Option<String> },
    Download { id: String, state: String, downloaded: u64, total: u64, bytes_per_sec: f64, error: Option<String> },
    Models { models: Vec<ModelState> },
    Settings { settings: Settings },
    Language { lang: TargetLanguage, epoch: u64 },
}

struct Inner {
    settings: Settings,
    phase: Phase,
    detail: Option<String>,
    pipeline: Option<PipelineHandle>,
    downloads: HashMap<String, Arc<AtomicBool>>,
    verifying: HashMap<String, Arc<AtomicBool>>,
}

pub struct AppState {
    inner: Mutex<Inner>,
    lang: Arc<LanguageControl>,
}

fn emit_app(app: &AppHandle, e: AppEvent) {
    let _ = app.emit(EVT_APP, e);
}

fn set_phase(app: &AppHandle, phase: Phase, detail: Option<String>) {
    let st = app.state::<AppState>();
    {
        let mut g = st.inner.lock().unwrap();
        g.phase = phase;
        g.detail = detail.clone();
    }
    emit_app(app, AppEvent::Phase { phase, detail });
}

fn emit_models(app: &AppHandle) {
    let st = app.state::<AppState>();
    let models = models::inventory(&st.inner.lock().unwrap().settings);
    emit_app(app, AppEvent::Models { models });
}

#[derive(Serialize)]
struct Snapshot {
    version: &'static str,
    phase: Phase,
    detail: Option<String>,
    settings: Settings,
    models: Vec<ModelState>,
    lang: TargetLanguage,
    epoch: u64,
    gpu_backend: &'static str,
    worker_found: bool,
    models_dir: String,
    /// Catalog id of the translator the backend uses for each language.
    translator_for: HashMap<TargetLanguage, &'static str>,
}

fn worker_exe() -> PathBuf {
    let name = if cfg!(windows) { "kikoe-mt.exe" } else { "kikoe-mt" };
    std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join(name))).unwrap_or_else(|| PathBuf::from(name))
}

#[tauri::command]
fn get_snapshot(state: State<'_, AppState>) -> Snapshot {
    let g = state.inner.lock().unwrap();
    let (lang, epoch) = state.lang.current();
    Snapshot {
        version: env!("CARGO_PKG_VERSION"),
        phase: g.phase,
        detail: g.detail.clone(),
        settings: g.settings.clone(),
        models: models::inventory(&g.settings),
        lang,
        epoch,
        gpu_backend: kikoe_asr::gpu_backend_compiled(),
        worker_found: worker_exe().exists(),
        models_dir: models::models_dir().to_string_lossy().into_owned(),
        translator_for: TargetLanguage::ALL.iter().map(|l| (*l, default_translator(*l).id)).collect(),
    }
}

/// Runs on a worker thread: device enumeration must not depend on (or change)
/// the COM apartment of Tauri's UI thread.
#[tauri::command]
async fn list_sources() -> Result<Vec<SourceInfo>, String> {
    std::thread::spawn(|| kikoe_capture::list_sources().map_err(|e| e.to_string()))
        .join()
        .map_err(|_| "device enumeration failed".to_string())?
}

#[tauri::command]
fn set_language(app: AppHandle, state: State<'_, AppState>, lang: String) -> Result<u64, String> {
    let lang = TargetLanguage::parse(&lang).ok_or_else(|| format!("unsupported language {lang:?}"))?;
    state.lang.set(lang);
    let (_, epoch) = state.lang.current();
    let settings = {
        let mut g = state.inner.lock().unwrap();
        g.settings.target_language = lang;
        let _ = g.settings.save();
        g.settings.clone()
    };
    emit_app(&app, AppEvent::Language { lang, epoch });
    emit_app(&app, AppEvent::Settings { settings });
    Ok(epoch)
}

#[derive(Deserialize)]
struct SettingsPatch {
    show_japanese: Option<bool>,
    source_id: Option<Option<String>>,
    use_gpu: Option<bool>,
    asr_model: Option<String>,
    overlay: Option<OverlaySettings>,
}

#[tauri::command]
fn update_settings(app: AppHandle, state: State<'_, AppState>, patch: SettingsPatch) -> Result<Settings, String> {
    let settings = {
        let mut g = state.inner.lock().unwrap();
        if let Some(v) = patch.show_japanese {
            g.settings.show_japanese = v;
        }
        if let Some(v) = patch.source_id {
            g.settings.source_id = v.filter(|s| !s.trim().is_empty());
        }
        if let Some(v) = patch.use_gpu {
            g.settings.use_gpu = v;
        }
        if let Some(id) = patch.asr_model {
            let spec = models::spec_for(&id)?;
            if spec.kind != ModelKind::Asr {
                return Err("not a speech recognition model".into());
            }
            g.settings.asr_model = id;
        }
        if let Some(mut o) = patch.overlay {
            o.opacity = o.opacity.clamp(0.0, 1.0);
            o.font_size = o.font_size.clamp(14, 72);
            if let Some(w) = app.get_webview_window("overlay") {
                let _ = if o.visible { w.show() } else { w.hide() };
            }
            g.settings.overlay = o;
        }
        g.settings.save()?;
        g.settings.clone()
    };
    emit_app(&app, AppEvent::Settings { settings: settings.clone() });
    Ok(settings)
}

#[tauri::command]
fn reset_overlay_position(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let w = app.get_webview_window("overlay").ok_or("overlay window missing")?;
    // Reset is also the recovery path for a hidden or lost overlay: make it visible in settings too.
    let settings = {
        let mut g = state.inner.lock().unwrap();
        g.settings.overlay.visible = true;
        let _ = g.settings.save();
        g.settings.clone()
    };
    emit_app(&app, AppEvent::Settings { settings });
    let _ = w.set_size(tauri::LogicalSize::new(900.0, 150.0));
    if let Ok(Some(m)) = w.primary_monitor() {
        let scale = m.scale_factor();
        let size = m.size().to_logical::<f64>(scale);
        let pos = m.position().to_logical::<f64>(scale);
        let _ = w.set_position(tauri::LogicalPosition::new(pos.x + (size.width - 900.0) / 2.0, pos.y + size.height - 230.0));
    }
    w.show().map_err(|e| e.to_string())?;
    w.set_always_on_top(true).map_err(|e| e.to_string())
}

#[tauri::command]
fn subtitle_rendered(state: State<'_, AppState>, seg_id: u64, lang: String) {
    if let Some(l) = TargetLanguage::parse(&lang) {
        if let Some(p) = state.inner.lock().unwrap().pipeline.as_ref() {
            p.record_ui_ack(seg_id, l);
        }
    }
}

// ------------------------------------------------------------------ models

#[tauri::command]
fn download_model(app: AppHandle, state: State<'_, AppState>, id: String) -> Result<(), String> {
    let spec = models::spec_for(&id)?;
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut g = state.inner.lock().unwrap();
        if g.downloads.contains_key(&id) {
            return Err("already downloading".into());
        }
        g.downloads.insert(id.clone(), cancel.clone());
    }
    let app2 = app.clone();
    std::thread::spawn(move || {
        let prog = |p: models::DownloadProgress| {
            emit_app(
                &app2,
                AppEvent::Download { id: spec.id.into(), state: "running".into(), downloaded: p.downloaded, total: p.total, bytes_per_sec: p.bytes_per_sec, error: None },
            )
        };
        let result = models::download(spec, cancel.clone(), prog);
        let st = app2.state::<AppState>();
        let (state_s, error) = match result {
            Ok(path) => match kikoe_core::models::VerifiedRecord::capture(spec.id, &path, spec.sha256.to_string()) {
                Ok(rec) => {
                    let mut g = st.inner.lock().unwrap();
                    g.settings.verified.retain(|r| r.model_id != spec.id || Path::new(&r.path) != path);
                    g.settings.verified.push(rec);
                    g.settings.model_paths.remove(spec.id);
                    let _ = g.settings.save();
                    ("done", None)
                }
                Err(e) => ("failed", Some(e.to_string())),
            },
            Err(e) if e == "cancelled" => ("cancelled", None),
            Err(e) => ("failed", Some(e)),
        };
        st.inner.lock().unwrap().downloads.remove(spec.id);
        emit_app(&app2, AppEvent::Download { id: spec.id.into(), state: state_s.into(), downloaded: 0, total: spec.size_bytes, bytes_per_sec: 0.0, error });
        emit_models(&app2);
    });
    Ok(())
}

#[tauri::command]
fn cancel_download(state: State<'_, AppState>, id: String) {
    if let Some(c) = state.inner.lock().unwrap().downloads.get(&id) {
        c.store(true, Ordering::Release);
    }
}

/// Use a model file that already exists elsewhere on disk (no copy is made).
#[tauri::command]
fn use_existing_model(app: AppHandle, state: State<'_, AppState>, id: String, path: String) -> Result<(), String> {
    let spec = models::spec_for(&id)?;
    let p = PathBuf::from(path.trim().trim_matches('"'));
    if !p.is_absolute() || !p.is_file() {
        return Err("choose an existing model file (absolute path)".into());
    }
    kikoe_core::models::quick_check(&p, spec).map_err(|e| format!("{}: {e}", spec.display_name))?;
    start_verification(app, &state, spec, p);
    Ok(())
}

fn start_verification(app: AppHandle, state: &AppState, spec: &'static kikoe_core::models::ModelSpec, path: PathBuf) {
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut g = state.inner.lock().unwrap();
        if g.verifying.contains_key(spec.id) {
            return;
        }
        g.verifying.insert(spec.id.into(), cancel.clone());
    }
    std::thread::spawn(move || {
        let total = spec.size_bytes;
        let mut last = std::time::Instant::now();
        let res = models::verify(spec, &path, &cancel, |done| {
            if last.elapsed() > Duration::from_millis(250) {
                last = std::time::Instant::now();
                emit_app(&app, AppEvent::Download { id: spec.id.into(), state: "verifying".into(), downloaded: done, total, bytes_per_sec: 0.0, error: None });
            }
        });
        let st = app.state::<AppState>();
        let (s, err) = match res {
            Ok(rec) => {
                let mut g = st.inner.lock().unwrap();
                g.settings.verified.retain(|r| !(r.model_id == spec.id && Path::new(&r.path) == path));
                g.settings.verified.push(rec);
                if path != models::app_path(spec) {
                    g.settings.model_paths.insert(spec.id.into(), path.to_string_lossy().into_owned());
                }
                let _ = g.settings.save();
                ("done", None)
            }
            Err(e) => ("failed", Some(format!("{}: {e}", spec.display_name))),
        };
        st.inner.lock().unwrap().verifying.remove(spec.id);
        emit_app(&app, AppEvent::Download { id: spec.id.into(), state: s.into(), downloaded: total, total, bytes_per_sec: 0.0, error: err });
        emit_models(&app);
    });
}

// ------------------------------------------------------------------ session

#[tauri::command]
fn start(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    {
        let mut g = state.inner.lock().unwrap();
        if g.pipeline.is_some() || matches!(g.phase, Phase::Loading | Phase::Stopping) {
            return Err("already running".into());
        }
        g.phase = Phase::Loading;
    }
    std::thread::Builder::new()
        .name("kikoe-start".into())
        .spawn(move || {
            if let Err((phase, msg)) = start_session(&app) {
                set_phase(&app, phase, Some(msg));
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn start_session(app: &AppHandle) -> Result<(), (Phase, String)> {
    let st = app.state::<AppState>();
    let settings = st.inner.lock().unwrap().settings.clone();
    let err = |m: String| (Phase::Error, m);
    set_phase(app, Phase::Loading, Some("Checking models…".into()));
    let asr_spec = models::spec_for(&settings.asr_model).map_err(err)?;
    let vad_spec = models::spec_for(DEFAULT_VAD_ID).map_err(err)?;
    let mut missing = Vec::new();
    let asr_path = models::resolve(asr_spec, &settings);
    let vad_path = models::resolve(vad_spec, &settings);
    if asr_path.is_none() {
        missing.push(asr_spec.display_name);
    }
    if vad_path.is_none() {
        missing.push(vad_spec.display_name);
    }
    let mut worker_models = HashMap::new();
    for l in TargetLanguage::ALL {
        let spec = default_translator(l);
        match models::resolve(spec, &settings) {
            Some(p) => {
                worker_models.insert(l, WorkerModel { path: p, style: spec.prompt_style.unwrap_or(PromptStyle::ChatMlNoThink), label: spec.display_name.into() });
            }
            None if l == st.lang.current().0 => missing.push(spec.display_name),
            None => {}
        }
    }
    if !missing.is_empty() {
        missing.dedup();
        return Err(err(format!("Install the required models first: {}", missing.join(", "))));
    }
    let exe = worker_exe();
    if !exe.exists() {
        return Err(err(format!("Translation worker not found next to the app ({exe:?}). Reinstall KikoeLive.")));
    }

    set_phase(app, Phase::Loading, Some("Loading speech recognition…".into()));
    let asr = WhisperAsr::load(WhisperConfig {
        model_path: asr_path.unwrap(),
        use_gpu: settings.use_gpu,
        threads: std::thread::available_parallelism().map(|n| (n.get() / 2).clamp(2, 8) as i32).unwrap_or(4),
        beam_size: 1,
        initial_prompt: None,
    })
    .map_err(err)?;
    let vad = SileroVad::load(&vad_path.unwrap(), 16).map_err(err)?;
    let translator = WorkerTranslator::new(WorkerConfig {
        exe,
        models: worker_models,
        gpu_layers: if settings.use_gpu { 999 } else { 0 },
        n_ctx: 2048,
        threads: 0,
        request_timeout: Duration::from_secs(20),
        load_timeout: Duration::from_secs(180),
    });

    set_phase(app, Phase::Loading, Some("Opening system audio…".into()));
    let clock = PipelineClock::new();
    let capture = match kikoe_capture::open_system_capture(settings.source_id.as_deref(), clock) {
        Ok(c) => c,
        Err(CaptureError::PermissionDenied(m)) => return Err((Phase::WaitingPermission, m)),
        Err(e) => return Err(err(e.to_string())),
    };
    let label = capture.source().label.clone();
    let app2 = app.clone();
    let sink: EventSink = Arc::new(move |e: PipelineEvent| {
        if let PipelineEvent::Error { stage, message, recoverable: false } = &e {
            log::error!("{stage}: {message}");
        }
        let _ = app2.emit(EVT_PIPELINE, &e);
    });
    let handle = PipelineHandle::start(
        PipelineConfig::default(),
        clock,
        capture,
        Engines { vad: Box::new(vad), asr: Box::new(asr), translator: Box::new(translator) },
        st.lang.clone(),
        sink,
    );
    st.inner.lock().unwrap().pipeline = Some(handle);
    set_phase(app, Phase::Running, Some(format!("Listening to {label}")));
    Ok(())
}

#[tauri::command]
fn stop(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let handle = state.inner.lock().unwrap().pipeline.take();
    let Some(handle) = handle else {
        set_phase(&app, Phase::Idle, None);
        return Ok(());
    };
    set_phase(&app, Phase::Stopping, None);
    std::thread::spawn(move || {
        let report = handle.stop();
        set_phase(&app, Phase::Idle, Some(format!("Stopped in {:.0} ms", report.stop_ms)));
    });
    Ok(())
}

fn stop_blocking(app: &AppHandle) {
    let handle = app.state::<AppState>().inner.lock().unwrap().pipeline.take();
    if let Some(h) = handle {
        let r = h.stop();
        log::info!("pipeline stopped on exit in {:.0} ms", r.stop_ms);
    }
}

fn verify_present_models(app: &AppHandle) {
    let st = app.state::<AppState>();
    let settings = st.inner.lock().unwrap().settings.clone();
    for m in models::inventory(&settings) {
        if m.status == "needs_verification" {
            if let (Ok(spec), Some(p)) = (models::spec_for(m.id), m.path.clone()) {
                start_verification(app.clone(), &st, spec, PathBuf::from(p));
            }
        }
    }
}

pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    kikoe_asr::install_logging();
    let settings = Settings::load();
    let lang = LanguageControl::new(settings.target_language);
    let overlay_visible = settings.overlay.visible;
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            inner: Mutex::new(Inner {
                settings,
                phase: Phase::Idle,
                detail: None,
                pipeline: None,
                downloads: HashMap::new(),
                verifying: HashMap::new(),
            }),
            lang,
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            list_sources,
            set_language,
            update_settings,
            reset_overlay_position,
            subtitle_rendered,
            download_model,
            cancel_download,
            use_existing_model,
            start,
            stop
        ])
        .setup(move |app| {
            if let Some(w) = app.get_webview_window("overlay") {
                let _ = w.set_always_on_top(true);
                if overlay_visible {
                    let _ = w.show();
                }
            }
            verify_present_models(app.handle());
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    let app = window.app_handle().clone();
                    stop_blocking(&app);
                    app.exit(0);
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building KikoeLive")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                stop_blocking(app);
            }
        });
}
