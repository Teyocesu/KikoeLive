//! Model catalog, provenance and integrity checks.
//!
//! Every downloadable file is pinned to an exact repository revision and SHA-256
//! taken from the Hugging Face LFS record. A file is only "installed" after its
//! hash matches; partial downloads live under a `.part` name and are never used.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::lang::TargetLanguage;
use crate::translate::PromptStyle;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelKind {
    Asr,
    Vad,
    Translator,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFormat {
    /// whisper.cpp legacy container (magic "lmgg").
    Ggml,
    Gguf,
}

#[derive(Clone, Debug, Serialize)]
pub struct ModelSpec {
    pub id: &'static str,
    pub kind: ModelKind,
    pub display_name: &'static str,
    pub file_name: &'static str,
    pub repo: &'static str,
    pub revision: &'static str,
    pub sha256: &'static str,
    pub size_bytes: u64,
    pub license: &'static str,
    pub format: ModelFormat,
    /// Target languages this translator is approved for (empty for ASR/VAD).
    pub serves: &'static [TargetLanguage],
    pub prompt_style: Option<PromptStyle>,
    pub notes: &'static str,
}

impl ModelSpec {
    pub fn url(&self) -> String {
        format!("https://huggingface.co/{}/resolve/{}/{}", self.repo, self.revision, self.file_name)
    }
}

pub const CATALOG: &[ModelSpec] = &[
    ModelSpec {
        id: "whisper-large-v3-turbo-q5_0",
        kind: ModelKind::Asr,
        display_name: "Whisper large-v3-turbo (q5_0)",
        file_name: "ggml-large-v3-turbo-q5_0.bin",
        repo: "ggerganov/whisper.cpp",
        revision: "5359861c739e955e79d9a303bcbc70fb988958b1",
        sha256: "394221709cd5ad1f40c46e6031ca61bce88931e6e088c188294c6d5a55ffa7e2",
        size_bytes: 574_041_195,
        license: "MIT (OpenAI Whisper weights; ggml conversion by whisper.cpp)",
        format: ModelFormat::Ggml,
        serves: &[],
        prompt_style: None,
        notes: "Japanese speech recognition. Default.",
    },
    ModelSpec {
        id: "whisper-small-q5_1",
        kind: ModelKind::Asr,
        display_name: "Whisper small (q5_1)",
        file_name: "ggml-small-q5_1.bin",
        repo: "ggerganov/whisper.cpp",
        revision: "5359861c739e955e79d9a303bcbc70fb988958b1",
        sha256: "ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb",
        size_bytes: 190_085_487,
        license: "MIT (OpenAI Whisper weights; ggml conversion by whisper.cpp)",
        format: ModelFormat::Ggml,
        serves: &[],
        prompt_style: None,
        notes: "Smaller, less accurate ASR for low-memory machines.",
    },
    ModelSpec {
        id: "silero-vad-v6.2.0",
        kind: ModelKind::Vad,
        display_name: "Silero VAD v6.2.0",
        file_name: "ggml-silero-v6.2.0.bin",
        repo: "ggml-org/whisper-vad",
        revision: "9ffd54a1e1ee413ddf265af9913beaf518d1639b",
        sha256: "2aa269b785eeb53a82983a20501ddf7c1d9c48e33ab63a41391ac6c9f7fb6987",
        size_bytes: 885_098,
        license: "MIT (Silero VAD)",
        format: ModelFormat::Ggml,
        serves: &[],
        prompt_style: None,
        notes: "Voice activity detection.",
    },
    ModelSpec {
        id: "qwen3-4b-q4_k_m",
        kind: ModelKind::Translator,
        display_name: "Qwen3 4B (Q4_K_M)",
        file_name: "Qwen3-4B-Q4_K_M.gguf",
        repo: "Qwen/Qwen3-4B-GGUF",
        revision: "bc640142c66e1fdd12af0bd68f40445458f3869b",
        sha256: "7485fe6f11af29433bc51cab58009521f205840f5b4ae3a32fa7f92e8534fdf5",
        size_bytes: 2_497_280_256,
        license: "Apache-2.0",
        format: ModelFormat::Gguf,
        serves: &[TargetLanguage::En, TargetLanguage::Es],
        prompt_style: Some(PromptStyle::ChatMlNoThink),
        notes: "Evaluated candidate (see EVIDENCE.md).",
    },
    ModelSpec {
        id: "qwen3.5-4b-q4_k_m",
        kind: ModelKind::Translator,
        display_name: "Qwen3.5 4B (Q4_K_M)",
        file_name: "Qwen3.5-4B-Q4_K_M.gguf",
        repo: "ggml-org/Qwen3.5-4B-GGUF",
        revision: "8d43d974efc48399cb58957ad49c8b26bf5a109c",
        sha256: "217002fa8008ef4fc6d548353a3aebef055a96e6ff8e0e2c0953ef8254a89c14",
        size_bytes: 3_032_091_584,
        license: "Apache-2.0",
        format: ModelFormat::Gguf,
        serves: &[TargetLanguage::En, TargetLanguage::Es],
        prompt_style: Some(PromptStyle::ChatMlNoThink),
        notes: "Evaluated candidate (see EVIDENCE.md).",
    },
];

pub fn spec(id: &str) -> Option<&'static ModelSpec> {
    CATALOG.iter().find(|m| m.id == id)
}

/// The translator the product uses for each target language. Chosen from the
/// measured evaluation in EVIDENCE.md; a single model serves both languages so
/// switching never loads a second large model.
pub fn default_translator(lang: TargetLanguage) -> &'static ModelSpec {
    let _ = lang;
    spec(DEFAULT_TRANSLATOR_ID).expect("default translator in catalog")
}

pub const DEFAULT_TRANSLATOR_ID: &str = "qwen3.5-4b-q4_k_m";
pub const DEFAULT_ASR_ID: &str = "whisper-large-v3-turbo-q5_0";
pub const DEFAULT_VAD_ID: &str = "silero-vad-v6.2.0";

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum VerifyError {
    #[error("file not found")]
    Missing,
    #[error("size mismatch: expected {expected} bytes, found {found}")]
    Size { expected: u64, found: u64 },
    #[error("not a {0:?} model file")]
    Format(ModelFormat),
    #[error("SHA-256 mismatch")]
    Hash,
    #[error("verification cancelled")]
    Cancelled,
    #[error("I/O error: {0}")]
    Io(String),
}

fn magic_ok(path: &Path, format: ModelFormat) -> io::Result<bool> {
    let mut f = File::open(path)?;
    let mut m = [0u8; 4];
    if f.read(&mut m)? < 4 {
        return Ok(false);
    }
    Ok(match format {
        ModelFormat::Gguf => &m == b"GGUF",
        ModelFormat::Ggml => &m == b"lmgg",
    })
}

/// Cheap check used at startup: exists, exact size, right container magic.
pub fn quick_check(path: &Path, spec: &ModelSpec) -> Result<(), VerifyError> {
    let meta = std::fs::metadata(path).map_err(|_| VerifyError::Missing)?;
    if meta.len() != spec.size_bytes {
        return Err(VerifyError::Size { expected: spec.size_bytes, found: meta.len() });
    }
    match magic_ok(path, spec.format) {
        Ok(true) => Ok(()),
        Ok(false) => Err(VerifyError::Format(spec.format)),
        Err(e) => Err(VerifyError::Io(e.to_string())),
    }
}

/// Streaming SHA-256. `progress` receives bytes hashed so far.
pub fn sha256_file(path: &Path, cancel: Option<&AtomicBool>, mut progress: impl FnMut(u64)) -> Result<String, VerifyError> {
    let mut f = File::open(path).map_err(|_| VerifyError::Missing)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    let mut done = 0u64;
    loop {
        if cancel.is_some_and(|c| c.load(Ordering::Relaxed)) {
            return Err(VerifyError::Cancelled);
        }
        let n = f.read(&mut buf).map_err(|e| VerifyError::Io(e.to_string()))?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
        done += n as u64;
        progress(done);
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

pub fn full_verify(
    path: &Path,
    spec: &ModelSpec,
    cancel: Option<&AtomicBool>,
    progress: impl FnMut(u64),
) -> Result<VerifiedRecord, VerifyError> {
    quick_check(path, spec)?;
    let hash = sha256_file(path, cancel, progress)?;
    if hash != spec.sha256 {
        return Err(VerifyError::Hash);
    }
    VerifiedRecord::capture(spec.id, path, hash)
}

/// Remembered result of a full verification, so a verified multi-GB file is not
/// re-hashed on every launch. Invalidated when size or mtime change.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifiedRecord {
    pub model_id: String,
    pub path: String,
    pub size: u64,
    pub modified_unix_ms: u64,
    pub sha256: String,
}

impl VerifiedRecord {
    pub fn capture(model_id: &str, path: &Path, sha256: String) -> Result<Self, VerifyError> {
        let meta = std::fs::metadata(path).map_err(|_| VerifyError::Missing)?;
        Ok(Self {
            model_id: model_id.to_string(),
            path: path.to_string_lossy().into_owned(),
            size: meta.len(),
            modified_unix_ms: mtime_ms(&meta),
            sha256,
        })
    }

    pub fn still_valid(&self, spec: &ModelSpec) -> bool {
        if self.model_id != spec.id || self.sha256 != spec.sha256 {
            return false;
        }
        let Ok(meta) = std::fs::metadata(&self.path) else { return false };
        meta.len() == self.size && mtime_ms(&meta) == self.modified_unix_ms && quick_check(Path::new(&self.path), spec).is_ok()
    }
}

fn mtime_ms(meta: &std::fs::Metadata) -> u64 {
    meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis() as u64).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn tmp(name: &str, bytes: &[u8]) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("kikoe-models-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(name);
        std::fs::File::create(&p).unwrap().write_all(bytes).unwrap();
        p
    }

    const fn fake_spec(size: u64, sha: &'static str) -> ModelSpec {
        ModelSpec {
            id: "fake",
            kind: ModelKind::Translator,
            display_name: "fake",
            file_name: "fake.gguf",
            repo: "x/y",
            revision: "r",
            sha256: sha,
            size_bytes: size,
            license: "test",
            format: ModelFormat::Gguf,
            serves: &[],
            prompt_style: None,
            notes: "",
        }
    }

    #[test]
    fn catalog_entries_are_pinned_and_unique() {
        let mut ids = std::collections::HashSet::new();
        for m in CATALOG {
            assert!(ids.insert(m.id), "duplicate id {}", m.id);
            assert_eq!(m.revision.len(), 40, "{} not pinned to a commit", m.id);
            assert_eq!(m.sha256.len(), 64);
            assert!(m.url().starts_with("https://huggingface.co/"));
        }
        for l in TargetLanguage::ALL {
            assert!(default_translator(l).serves.contains(&l));
        }
    }

    #[test]
    fn truncated_or_wrong_files_are_rejected() {
        // "GGUF" + 4 bytes; sha256 of these 8 bytes:
        let good = b"GGUF\x03\x00\x00\x00";
        let sha = sha256_file(&tmp("good.gguf", good), None, |_| {}).unwrap();
        let sha: &'static str = Box::leak(sha.into_boxed_str());
        let spec = fake_spec(8, sha);
        assert!(full_verify(&tmp("good2.gguf", good), &spec, None, |_| {}).is_ok());
        assert_eq!(
            quick_check(&tmp("short.gguf", b"GGUF"), &spec),
            Err(VerifyError::Size { expected: 8, found: 4 })
        );
        assert_eq!(quick_check(&tmp("html.gguf", b"<html>.."), &spec), Err(VerifyError::Format(ModelFormat::Gguf)));
        assert_eq!(full_verify(&tmp("flip.gguf", b"GGUF\x03\x00\x00\x01"), &spec, None, |_| {}), Err(VerifyError::Hash));
        assert_eq!(quick_check(Path::new("Z:/definitely/missing.gguf"), &spec), Err(VerifyError::Missing));
    }

    #[test]
    fn verified_record_invalidates_on_change() {
        let good = b"GGUF\x03\x00\x00\x00";
        let p = tmp("rec.gguf", good);
        let sha = sha256_file(&p, None, |_| {}).unwrap();
        let sha: &'static str = Box::leak(sha.into_boxed_str());
        let spec = fake_spec(8, sha);
        let rec = full_verify(&p, &spec, None, |_| {}).unwrap();
        assert!(rec.still_valid(&spec));
        std::fs::File::create(&p).unwrap().write_all(b"GGUF\x03\x00\x00\x00\x00").unwrap();
        assert!(!rec.still_valid(&spec));
    }
}
