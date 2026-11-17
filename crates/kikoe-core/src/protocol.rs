//! JSON-lines protocol between the app and the `kikoe-mt` translation worker.
//!
//! The worker is a separate process so that (a) whisper.cpp and llama.cpp, which
//! each embed their own ggml, never share an address space, (b) a hung or
//! crashed model can be killed and restarted without taking the app down, and
//! (c) memory is returned to the OS when translation stops or the model changes.
//! The worker talks only over its own stdin/stdout: no sockets, no listeners.

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkerRequest {
    Load {
        model_path: String,
        /// Layers to offload to GPU (0 = CPU only, large value = all).
        gpu_layers: u32,
        n_ctx: u32,
        threads: i32,
    },
    Translate {
        id: u64,
        /// Constant, cacheable part of the prompt (system + examples).
        prefix: String,
        /// Per-line part of the prompt.
        suffix: String,
        max_tokens: u32,
        stop: Vec<String>,
    },
    /// Abort generation of request `id` if it is still running.
    Cancel { id: u64 },
    Shutdown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkerResponse {
    Hello {
        protocol: u32,
        pid: u32,
        /// Backends compiled into the worker, e.g. "cuda", "metal", "cpu".
        backend: String,
    },
    Loaded {
        model_path: String,
        load_ms: u64,
        n_ctx: u32,
        gpu_layers: u32,
        description: String,
        size_bytes: u64,
    },
    Translated {
        id: u64,
        text: String,
        prompt_tokens: u32,
        cached_tokens: u32,
        generated_tokens: u32,
        prompt_ms: f64,
        generate_ms: f64,
        /// "eog", "stop_string", "max_tokens".
        finish: String,
    },
    Cancelled { id: u64 },
    Error { id: Option<u64>, message: String },
}

pub fn encode_line<T: Serialize>(msg: &T) -> String {
    let mut s = serde_json::to_string(msg).expect("protocol messages always serialise");
    s.push('\n');
    s
}

pub fn decode_request(line: &str) -> Result<WorkerRequest, String> {
    serde_json::from_str(line.trim()).map_err(|e| format!("bad request: {e}"))
}

pub fn decode_response(line: &str) -> Result<WorkerResponse, String> {
    serde_json::from_str(line.trim()).map_err(|e| format!("bad response: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_translate_request() {
        let r = WorkerRequest::Translate { id: 7, prefix: "p".into(), suffix: "s\n改行".into(), max_tokens: 32, stop: vec![] };
        let line = encode_line(&r);
        assert!(line.ends_with('\n') && !line[..line.len() - 1].contains('\n'), "one message per line");
        match decode_request(&line).unwrap() {
            WorkerRequest::Translate { id, suffix, .. } => {
                assert_eq!(id, 7);
                assert_eq!(suffix, "s\n改行");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn rejects_garbage() {
        assert!(decode_response("{\"type\":\"nope\"}").is_err());
        assert!(decode_request("not json").is_err());
    }
}
