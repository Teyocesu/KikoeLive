//! Model store: inventory, verification, pinned downloads.
//!
//! * Files live in the app's own data folder (`<local data>/KikoeLive/models`) or
//!   at a user-chosen existing path (no duplicate copy); either way a file is used
//!   only after its SHA-256 matches the catalog.
//! * Downloads come only from the pinned catalog URL, go to `<name>.part`, resume
//!   with an HTTP Range request, can be cancelled, and are renamed into place only
//!   after verification. Nothing downloads unless the user presses Download.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use kikoe_core::lang::TargetLanguage;
use kikoe_core::models::{CATALOG, ModelKind, ModelSpec, VerifiedRecord, full_verify, quick_check};
use serde::Serialize;

use crate::settings::Settings;

pub fn models_dir() -> PathBuf {
    dirs::data_local_dir().unwrap_or_else(std::env::temp_dir).join("KikoeLive").join("models")
}

pub fn app_path(spec: &ModelSpec) -> PathBuf {
    models_dir().join(spec.file_name)
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelState {
    pub id: &'static str,
    pub display_name: &'static str,
    pub kind: ModelKind,
    pub size_bytes: u64,
    pub license: &'static str,
    pub source_repo: &'static str,
    pub revision: &'static str,
    pub serves: Vec<TargetLanguage>,
    /// installed | missing | needs_verification | invalid
    pub status: &'static str,
    pub path: Option<String>,
    pub external: bool,
    pub detail: Option<String>,
}

/// Path of a verified, usable copy of `spec`, if any.
pub fn resolve(spec: &ModelSpec, settings: &Settings) -> Option<PathBuf> {
    let candidates: Vec<PathBuf> = settings
        .model_paths
        .get(spec.id)
        .map(PathBuf::from)
        .into_iter()
        .chain(std::iter::once(app_path(spec)))
        .collect();
    for c in candidates {
        let ok = settings.verified.iter().any(|r| Path::new(&r.path) == c && r.still_valid(spec));
        if ok {
            return Some(c);
        }
    }
    None
}

pub fn inventory(settings: &Settings) -> Vec<ModelState> {
    CATALOG
        .iter()
        .map(|spec| {
            let external = settings.model_paths.contains_key(spec.id);
            let (status, path, detail) = match resolve(spec, settings) {
                Some(p) => ("installed", Some(p.to_string_lossy().into_owned()), None),
                None => {
                    let p = settings.model_paths.get(spec.id).map(PathBuf::from).unwrap_or_else(|| app_path(spec));
                    match quick_check(&p, spec) {
                        Ok(()) => ("needs_verification", Some(p.to_string_lossy().into_owned()), None),
                        Err(kikoe_core::models::VerifyError::Missing) => ("missing", None, None),
                        Err(e) => ("invalid", Some(p.to_string_lossy().into_owned()), Some(e.to_string())),
                    }
                }
            };
            ModelState {
                id: spec.id,
                display_name: spec.display_name,
                kind: spec.kind,
                size_bytes: spec.size_bytes,
                license: spec.license,
                source_repo: spec.repo,
                revision: spec.revision,
                serves: spec.serves.to_vec(),
                status,
                path,
                external,
                detail,
            }
        })
        .collect()
}

/// Full SHA-256 verification of a candidate file; returns the record to store.
pub fn verify(spec: &ModelSpec, path: &Path, cancel: &AtomicBool, mut progress: impl FnMut(u64)) -> Result<VerifiedRecord, String> {
    full_verify(path, spec, Some(cancel), &mut progress).map_err(|e| e.to_string())
}

pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub bytes_per_sec: f64,
}

/// Download `spec` into the app models dir. Resumes a previous `.part`.
pub fn download(spec: &ModelSpec, cancel: Arc<AtomicBool>, mut progress: impl FnMut(DownloadProgress)) -> Result<PathBuf, String> {
    let dir = models_dir();
    std::fs::create_dir_all(&dir).map_err(|e| format!("cannot create {dir:?}: {e}"))?;
    let final_path = app_path(spec);
    let part = final_path.with_extension(format!(
        "{}.part",
        final_path.extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default()
    ));
    let mut have = std::fs::metadata(&part).map(|m| m.len()).unwrap_or(0);
    if have > spec.size_bytes {
        let _ = std::fs::remove_file(&part);
        have = 0;
    }
    if have < spec.size_bytes {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!("KikoeLive/", env!("CARGO_PKG_VERSION"), " (model download)"))
            .connect_timeout(Duration::from_secs(20))
            .timeout(None)
            .build()
            .map_err(|e| e.to_string())?;
        let mut req = client.get(spec.url());
        if have > 0 {
            req = req.header(reqwest::header::RANGE, format!("bytes={have}-"));
        }
        let mut resp = req.send().map_err(|e| format!("download failed: {e}"))?;
        let status = resp.status();
        if have > 0 && status == reqwest::StatusCode::OK {
            have = 0; // server ignored Range: start over
        } else if !(status.is_success()) {
            return Err(format!("download failed: HTTP {status}"));
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(have > 0)
            .truncate(have == 0)
            .open(&part)
            .map_err(|e| e.to_string())?;
        let mut buf = vec![0u8; 1 << 16];
        let (mut last, start, base) = (Instant::now(), Instant::now(), have);
        loop {
            if cancel.load(Ordering::Acquire) {
                return Err("cancelled".into());
            }
            let n = resp.read(&mut buf).map_err(|e| format!("download interrupted: {e}"))?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
            have += n as u64;
            if have > spec.size_bytes {
                return Err("server sent more data than expected".into());
            }
            if last.elapsed() >= Duration::from_millis(250) {
                last = Instant::now();
                let secs = start.elapsed().as_secs_f64().max(0.001);
                progress(DownloadProgress { downloaded: have, total: spec.size_bytes, bytes_per_sec: (have - base) as f64 / secs });
            }
        }
        file.flush().map_err(|e| e.to_string())?;
    }
    if have != spec.size_bytes {
        return Err(format!("incomplete download ({have} of {} bytes); press Download to resume", spec.size_bytes));
    }
    // verify the .part before it can ever be used
    let hash_ok = kikoe_core::models::sha256_file(&part, Some(&cancel), |_| {}).map_err(|e| e.to_string())? == spec.sha256;
    if !hash_ok {
        let _ = std::fs::remove_file(&part);
        return Err("downloaded file failed SHA-256 verification and was deleted".into());
    }
    std::fs::rename(&part, &final_path).map_err(|e| e.to_string())?;
    Ok(final_path)
}

pub fn spec_for(id: &str) -> Result<&'static ModelSpec, String> {
    kikoe_core::models::spec(id).ok_or_else(|| format!("unknown model id {id:?}"))
}
