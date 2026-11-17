//! kikoe-eval: measurements with the same engines and pipeline as the app.
//!
//!   mt        translate a JSONL corpus (correct Japanese in) -> per-language results
//!   asr       transcribe a JSONL manifest of WAV files -> CER per utterance
//!   pipeline  run the full live pipeline on a WAV (real-time paced) with optional
//!             language switches, aligned against a truth file
//!   live      run the full pipeline on real system audio (WASAPI / SCK) for N seconds
//!
//! Results are JSON (machine-readable) plus Markdown (for review in EVIDENCE.md).

mod metrics;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow, bail};
use kikoe_asr::{SileroVad, WhisperAsr, WhisperConfig};
use kikoe_core::audio::AudioFormat;
use kikoe_core::clock::PipelineClock;
use kikoe_core::lang::{LanguageControl, TargetLanguage};
use kikoe_core::pipeline::{AsrEngine, CancelFlags, Engines, PipelineConfig, PipelineEvent, PipelineHandle, TranslationEngine};
use kikoe_core::segmenter::{EnergyVad, Vad};
use kikoe_core::translate::{PromptStyle, Route, sanitize_output};
use kikoe_core::worker::{WorkerConfig, WorkerModel, WorkerTranslator};
use serde::{Deserialize, Serialize};
use serde_json::json;

struct Args {
    cmd: String,
    kv: HashMap<String, Vec<String>>,
    flags: Vec<String>,
}

impl Args {
    fn parse() -> Self {
        let mut it = std::env::args().skip(1);
        let cmd = it.next().unwrap_or_default();
        let mut kv: HashMap<String, Vec<String>> = HashMap::new();
        let mut flags = Vec::new();
        let rest: Vec<String> = it.collect();
        let mut i = 0;
        while i < rest.len() {
            let a = &rest[i];
            if let Some(k) = a.strip_prefix("--") {
                if i + 1 < rest.len() && !rest[i + 1].starts_with("--") {
                    kv.entry(k.to_string()).or_default().push(rest[i + 1].clone());
                    i += 2;
                    continue;
                }
                flags.push(k.to_string());
            }
            i += 1;
        }
        Self { cmd, kv, flags }
    }
    fn get(&self, k: &str) -> Option<&str> {
        self.kv.get(k).and_then(|v| v.first()).map(String::as_str)
    }
    fn req(&self, k: &str) -> Result<&str> {
        self.get(k).ok_or_else(|| anyhow!("missing --{k}"))
    }
    fn all(&self, k: &str) -> Vec<String> {
        self.kv.get(k).cloned().unwrap_or_default()
    }
    fn flag(&self, k: &str) -> bool {
        self.flags.iter().any(|f| f == k)
    }
    fn num<T: std::str::FromStr>(&self, k: &str, default: T) -> T {
        self.get(k).and_then(|v| v.parse().ok()).unwrap_or(default)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Item {
    id: String,
    ja: String,
    #[serde(default)]
    en: String,
    #[serde(default)]
    es: String,
    #[serde(default)]
    context: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    wav: Option<String>,
    #[serde(default)]
    start_s: Option<f64>,
    #[serde(default)]
    end_s: Option<f64>,
}

fn read_jsonl(path: &str) -> Result<Vec<Item>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    text.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(|e| anyhow!("{path}: {e}"))).collect()
}

fn langs(a: &Args) -> Vec<TargetLanguage> {
    match a.get("lang").unwrap_or("both") {
        "en" => vec![TargetLanguage::En],
        "es" => vec![TargetLanguage::Es],
        _ => vec![TargetLanguage::En, TargetLanguage::Es],
    }
}

fn worker(a: &Args, mt_model: &str) -> Result<WorkerTranslator> {
    let exe = PathBuf::from(a.get("worker").unwrap_or("target/release/kikoe-mt.exe"));
    if !exe.exists() {
        bail!("worker executable {exe:?} not found");
    }
    let label = Path::new(mt_model).file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
    let model = WorkerModel { path: PathBuf::from(mt_model), style: PromptStyle::ChatMlNoThink, label };
    let mut models = HashMap::new();
    models.insert(TargetLanguage::En, model.clone());
    models.insert(TargetLanguage::Es, model);
    Ok(WorkerTranslator::new(WorkerConfig {
        exe,
        models,
        gpu_layers: a.num("gpu-layers", 999),
        n_ctx: 2048,
        threads: a.num("threads", 0),
        request_timeout: Duration::from_secs(30),
        load_timeout: Duration::from_secs(180),
    }))
}

fn route(a: &Args) -> Route {
    match a.get("route") {
        Some("assisted") => Route::EnglishAssisted,
        Some("direct") => Route::Direct,
        _ => kikoe_core::models::DEFAULT_SPANISH_ROUTE,
    }
}

fn write_out(prefix: &str, json: &serde_json::Value, md: &str) -> Result<()> {
    if let Some(dir) = Path::new(prefix).parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(format!("{prefix}.json"), serde_json::to_string_pretty(json)?)?;
    std::fs::write(format!("{prefix}.md"), md)?;
    eprintln!("wrote {prefix}.json and {prefix}.md");
    Ok(())
}

fn md_escape(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

// --------------------------------------------------------------------------- mt

fn cmd_mt(a: &Args) -> Result<()> {
    let items = read_jsonl(a.req("corpus")?)?;
    let model = a.req("model")?;
    let mut w = worker(a, model)?;
    let never = AtomicBool::new(false);
    let use_context = !a.flag("no-context");
    let mut results = Vec::new();
    let mut md = format!(
        "# MT evaluation\n\nmodel: `{model}`  \ncorpus: `{}`  \ncontext: {use_context}  \nspanish route: {:?}\n\n",
        a.req("corpus")?,
        route(a)
    );
    for lang in langs(a) {
        let t_load = Instant::now();
        let load = w.prepare(lang).map_err(|e| anyhow!(e))?;
        let load_ms = load.unwrap_or(0).max(0) as f64;
        eprintln!("[{}] model ready ({} ms, worker backend {:?})", lang.code(), load_ms, w.backend());
        md.push_str(&format!("## ja-{}\n\n| id | Japanese | reference | output | chrF | ms |\n|---|---|---|---|---|---|\n", lang.code()));
        let mut chrf_sum = 0.0;
        let mut times = Vec::new();
        for it in &items {
            let ctx: Vec<String> = if use_context { it.context.clone() } else { vec![] };
            let t0 = Instant::now();
            let flags = CancelFlags { stop: &never, switch: &never };
            let out = kikoe_core::pipeline::translate_line(&mut w, PromptStyle::ChatMlNoThink, lang, route(a), &ctx, &it.ja, &flags)
                .map_err(|e| anyhow!("{e:?}"))?;
            let ms = t0.elapsed().as_secs_f64() * 1000.0;
            times.push(ms);
            let (text, issue) = match sanitize_output(&out.raw).and_then(|t| kikoe_core::translate::check_plausible(&it.ja, &t).map(|_| t)) {
                Ok(t) => (t, None),
                Err(i) => (String::new(), Some(format!("{i:?}"))),
            };
            let reference = if lang == TargetLanguage::En { &it.en } else { &it.es };
            let chrf = metrics::chrf(&text, reference);
            chrf_sum += chrf;
            md.push_str(&format!(
                "| {} | {} | {} | {} | {:.1} | {:.0} |\n",
                it.id,
                md_escape(&it.ja),
                md_escape(reference),
                md_escape(if text.is_empty() { issue.as_deref().unwrap_or("") } else { &text }),
                chrf,
                ms
            ));
            results.push(json!({
                "lang": lang.code(), "id": it.id, "ja": it.ja, "context": ctx, "reference": reference,
                "raw": out.raw, "output": text, "sanitize_issue": issue, "chrf": chrf, "ms": ms,
                "prompt_tokens": out.prompt_tokens, "cached_tokens": out.cached_tokens,
                "generated_tokens": out.generated_tokens, "prompt_ms": out.prompt_ms, "generate_ms": out.generate_ms,
                "tags": it.tags,
            }));
        }
        let s = kikoe_core::metrics::summarize(times.clone());
        let mean_chrf = chrf_sum / items.len().max(1) as f64;
        md.push_str(&format!(
            "\nmean chrF {:.1}; latency ms median {:.0} / p95 {:.0} / max {:.0}; load {:.0} ms (prepare {} ms wall)\n\n",
            mean_chrf,
            s.median,
            s.p95,
            s.max,
            load_ms,
            t_load.elapsed().as_millis()
        ));
    }
    w.shutdown();
    write_out(a.req("out")?, &json!({ "model": model, "results": results }), &md)
}

// -------------------------------------------------------------------------- asr

fn read_wav(path: &str) -> Result<(Vec<f32>, AudioFormat)> {
    let mut r = hound::WavReader::open(path).with_context(|| format!("opening {path}"))?;
    let spec = r.spec();
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => r.samples::<f32>().collect::<Result<_, _>>()?,
        hound::SampleFormat::Int => {
            let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
            r.samples::<i32>().map(|s| s.map(|v| v as f32 / max)).collect::<Result<_, _>>()?
        }
    };
    Ok((samples, AudioFormat { sample_rate: spec.sample_rate, channels: spec.channels }))
}

fn to_16k_mono(samples: &[f32], f: AudioFormat) -> Vec<f32> {
    let mut mono = Vec::new();
    kikoe_core::audio::downmix_to_mono(samples, f.channels, &mut mono);
    let mut r = kikoe_core::resample::Resampler::new(f.sample_rate, 16_000);
    let mut out = Vec::new();
    r.process(&mono, &mut out);
    r.finish(&mut out);
    out
}

fn whisper(a: &Args) -> Result<WhisperAsr> {
    WhisperAsr::load(WhisperConfig {
        model_path: PathBuf::from(a.req("asr")?),
        use_gpu: !a.flag("cpu"),
        threads: a.num("asr-threads", 4),
        beam_size: a.num("beam", 1),
        initial_prompt: a.get("asr-prompt").map(str::to_owned),
    })
    .map_err(|e| anyhow!(e))
}

fn cmd_asr(a: &Args) -> Result<()> {
    let mut items = read_jsonl(a.req("manifest")?)?;
    if let Some(n) = a.get("limit").and_then(|v| v.parse::<usize>().ok()) {
        items.truncate(n);
    }
    let t_load = Instant::now();
    let mut asr = whisper(a)?;
    let load_ms = t_load.elapsed().as_millis();
    let abort = Arc::new(AtomicBool::new(false));
    let mut md = format!(
        "# ASR evaluation\n\nmodel: {} (load {load_ms} ms)\n\n| id | reference | hypothesis | CER | audio s | ms |\n|---|---|---|---|---|---|\n",
        asr.describe()
    );
    let mut res = Vec::new();
    let (mut edits, mut ref_len) = (0usize, 0usize);
    let mut times = Vec::new();
    for it in &items {
        let wav = it.wav.as_deref().ok_or_else(|| anyhow!("{} has no wav", it.id))?;
        let (s, f) = read_wav(wav)?;
        let pcm = to_16k_mono(&s, f);
        let t0 = Instant::now();
        let out = asr.transcribe(&pcm, &abort).map_err(|e| anyhow!(e))?;
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        times.push(ms);
        let (e, n) = metrics::cer_counts(&out.text, &it.ja);
        edits += e;
        ref_len += n;
        let cer = e as f64 / n.max(1) as f64;
        md.push_str(&format!(
            "| {} | {} | {} | {:.3} | {:.1} | {:.0} |\n",
            it.id,
            md_escape(&it.ja),
            md_escape(&out.text),
            cer,
            pcm.len() as f64 / 16_000.0,
            ms
        ));
        res.push(json!({"id": it.id, "reference": it.ja, "hypothesis": out.text, "cer": cer, "ms": ms,
            "no_speech_prob": out.no_speech_prob, "avg_token_prob": out.avg_token_prob, "audio_s": pcm.len() as f64 / 16000.0}));
    }
    let s = kikoe_core::metrics::summarize(times);
    md.push_str(&format!(
        "\ncorpus CER {:.3} ({edits}/{ref_len}); ASR ms median {:.0} / p95 {:.0} / max {:.0}\n",
        edits as f64 / ref_len.max(1) as f64,
        s.median,
        s.p95,
        s.max
    ));
    write_out(
        a.req("out")?,
        &json!({"model": asr.describe(), "corpus_cer": edits as f64 / ref_len.max(1) as f64, "results": res}),
        &md,
    )
}

// --------------------------------------------------------------------- pipeline

#[derive(Clone, Debug, Serialize)]
struct Logged {
    t_ms: f64,
    event: PipelineEvent,
}

fn build_engines(a: &Args) -> Result<Engines> {
    let vad: Box<dyn Vad> = match a.get("vad-kind").unwrap_or("silero") {
        "energy" => Box::new(EnergyVad::new()),
        _ => Box::new(SileroVad::load(Path::new(a.req("vad")?), a.num("vad-context", 16)).map_err(|e| anyhow!(e))?),
    };
    let asr = whisper(a)?;
    let mt = worker(a, a.req("mt")?)?;
    Ok(Engines { vad, asr: Box::new(asr), translator: Box::new(mt) })
}

fn parse_switches(a: &Args) -> Result<Vec<(f64, TargetLanguage)>> {
    a.all("switch")
        .iter()
        .map(|s| {
            let (t, l) = s.split_once(':').ok_or_else(|| anyhow!("--switch expects SECONDS:LANG"))?;
            Ok((t.parse()?, TargetLanguage::parse(l).ok_or_else(|| anyhow!("bad language {l}"))?))
        })
        .collect()
}

fn run_pipeline(
    a: &Args,
    capture: Box<dyn kikoe_core::audio::CaptureStream>,
    clock: PipelineClock,
    max_secs: f64,
    drain: bool,
) -> Result<(Vec<Logged>, kikoe_core::pipeline::StopReport, kikoe_core::pipeline::PipelineStats)> {
    let initial = TargetLanguage::parse(a.get("lang").unwrap_or("en")).unwrap_or(TargetLanguage::En);
    let lang = LanguageControl::new(initial);
    let log: Arc<Mutex<Vec<Logged>>> = Arc::new(Mutex::new(Vec::new()));
    let log2 = log.clone();
    let t0 = Instant::now();
    let verbose = a.flag("verbose");
    let sink: kikoe_core::pipeline::EventSink = Arc::new(move |e: PipelineEvent| {
        if verbose && !matches!(e, PipelineEvent::Level { .. } | PipelineEvent::Stats { .. }) {
            eprintln!("{:>8.0} {}", t0.elapsed().as_secs_f64() * 1000.0, serde_json::to_string(&e).unwrap_or_default());
        }
        if !matches!(e, PipelineEvent::Level { .. }) {
            log2.lock().unwrap().push(Logged { t_ms: t0.elapsed().as_secs_f64() * 1000.0, event: e });
        }
    });
    let mut cfg = PipelineConfig { spanish_route: route(a), ..PipelineConfig::default() };
    if a.flag("no-drop") {
        cfg.drop_stale = false;
        cfg.asr_queue = 1024;
        cfg.mt_queue = 1024;
    }
    let engines = build_engines(a)?;
    let handle = PipelineHandle::start(cfg, clock, capture, engines, lang.clone(), sink);
    let switches = parse_switches(a)?;
    let mut next_switch = 0;
    let start = Instant::now();
    loop {
        let el = start.elapsed().as_secs_f64();
        while next_switch < switches.len() && el >= switches[next_switch].0 {
            let (_, l) = switches[next_switch];
            let e = lang.set(l);
            eprintln!("switch to {} at {:.1}s (epoch {:?})", l.code(), el, e);
            log.lock().unwrap().push(Logged {
                t_ms: t0.elapsed().as_secs_f64() * 1000.0,
                event: PipelineEvent::State { state: kikoe_core::pipeline::PipelineState::Processing, detail: Some(format!("LANGUAGE_SWITCH {}", l.code())) },
            });
            next_switch += 1;
        }
        if (drain && handle.is_drained()) || el > max_secs {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let stats = handle.stats();
    let report = handle.stop();
    let events = log.lock().unwrap().clone();
    Ok((events, report, stats))
}

fn cmd_pipeline(a: &Args) -> Result<()> {
    let wav = a.req("wav")?;
    let (samples, fmt) = read_wav(wav)?;
    let dur = samples.len() as f64 / fmt.channels as f64 / fmt.sample_rate as f64;
    let truth = a.get("truth").map(read_jsonl).transpose()?.unwrap_or_default();
    let clock = PipelineClock::new();
    let realtime = !a.flag("fast");
    let cap = kikoe_capture::file::FileCapture::new(wav, samples, fmt, clock, realtime, 10);
    let (events, report, stats) = run_pipeline(a, Box::new(cap), clock, dur * 3.0 + 60.0, true)?;
    let analysis = metrics::align(&events.iter().map(|l| l.event.clone()).collect::<Vec<_>>(), &truth, 0);
    let md = metrics::pipeline_markdown(wav, &analysis, &stats, &report);
    write_out(
        a.req("out")?,
        &json!({"wav": wav, "duration_s": dur, "realtime": realtime, "stop": report, "stats": stats, "analysis": analysis, "events": events}),
        &md,
    )
}

fn cmd_live(a: &Args) -> Result<()> {
    let secs: f64 = a.num("seconds", 60.0);
    let truth = a.get("truth").map(read_jsonl).transpose()?.unwrap_or_default();
    let clock = PipelineClock::new();
    let cap = kikoe_capture::open_system_capture(a.get("source"), clock).map_err(|e| anyhow!("{e}"))?;
    eprintln!("capturing {:?}", cap.source());
    // Optional: play a test WAV through the default output (what a viewer would hear).
    if let Some(play) = a.get("play") {
        let play = std::fs::canonicalize(play).with_context(|| format!("--play {play}"))?;
        // canonicalize() yields a \\?\ verbatim path; SoundPlayer wants a plain one.
        let path = play.to_string_lossy().trim_start_matches(r"\\?\").replace('\'', "''");
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            let _ = std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", &format!("(New-Object Media.SoundPlayer '{path}').PlaySync()")])
                .status();
        });
    }
    let (events, report, stats) = run_pipeline(a, cap, clock, secs, false)?;
    let evs: Vec<PipelineEvent> = events.iter().map(|l| l.event.clone()).collect();
    let offset = if truth.is_empty() { 0 } else { metrics::estimate_offset_ms(&evs, &truth) };
    let analysis = metrics::align(&evs, &truth, offset);
    let md = format!("estimated playback offset: {offset} ms\n\n{}", metrics::pipeline_markdown("live system audio", &analysis, &stats, &report));
    write_out(a.req("out")?, &json!({"live_seconds": secs, "offset_ms": offset, "stop": report, "stats": stats, "analysis": analysis, "events": events}), &md)
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    kikoe_asr::install_logging();
    let a = Args::parse();
    match a.cmd.as_str() {
        "mt" => cmd_mt(&a),
        "asr" => cmd_asr(&a),
        "pipeline" => cmd_pipeline(&a),
        "live" => cmd_live(&a),
        _ => {
            eprintln!("usage: kikoe-eval <mt|asr|pipeline|live> [options]  (see crates/kikoe-eval/src/main.rs)");
            Ok(())
        }
    }
}
