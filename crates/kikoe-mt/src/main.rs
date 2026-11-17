//! kikoe-mt: KikoeLive's translation worker.
//!
//! Protocol: one JSON object per line on stdin (requests) and stdout (responses),
//! defined in `kikoe_core::protocol`. stdout carries nothing else; llama.cpp logs
//! go to stderr. The worker exits on `shutdown` or when stdin closes (parent gone).
//!
//! Prompt caching: the language-constant prompt prefix (system + examples) is
//! evaluated once and its sequence state snapshotted; each line restores that
//! snapshot and evaluates only the short suffix. This works for both plain
//! transformer models and hybrid/recurrent ones, unlike partial KV removal.

use std::io::{BufRead, Write};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use std::sync::mpsc::{Receiver, channel};

use anyhow::{Context, Result, anyhow};
use kikoe_core::protocol::{PROTOCOL_VERSION, WorkerRequest, WorkerResponse, decode_request, encode_line};
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::context::session::{LlamaStateSeqFlags, SeqState};
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;

const N_BATCH: u32 = 512;
/// Prompt prefixes kept as sequence-state snapshots (e.g. English, Spanish,
/// English-assisted Spanish). Least recently used is evicted.
const PREFIX_CACHE: usize = 4;

fn backend_name() -> &'static str {
    if cfg!(feature = "cuda") {
        "cuda"
    } else if cfg!(feature = "metal") {
        "metal"
    } else {
        "cpu"
    }
}

fn send(msg: &WorkerResponse) {
    let mut out = std::io::stdout().lock();
    let _ = out.write_all(encode_line(msg).as_bytes());
    let _ = out.flush();
}

fn main() {
    let cancel = Arc::new(AtomicU64::new(0));
    let rx = spawn_reader(cancel.clone());
    send(&WorkerResponse::Hello { protocol: PROTOCOL_VERSION, pid: std::process::id(), backend: backend_name().into() });
    let backend = match LlamaBackend::init() {
        Ok(b) => b,
        Err(e) => {
            send(&WorkerResponse::Error { id: None, message: format!("backend init failed: {e}") });
            return;
        }
    };

    let mut pending: Option<WorkerRequest> = None;
    loop {
        let req = match pending.take() {
            Some(r) => r,
            None => match rx.recv() {
                Ok(r) => r,
                Err(_) => return,
            },
        };
        match req {
            WorkerRequest::Shutdown => return,
            WorkerRequest::Cancel { .. } => continue,
            WorkerRequest::Translate { id, .. } => {
                send(&WorkerResponse::Error { id: Some(id), message: "no model loaded".into() });
            }
            WorkerRequest::Load { model_path, gpu_layers, n_ctx, threads } => {
                match serve_model(&backend, &rx, &cancel, &model_path, gpu_layers, n_ctx, threads) {
                    Ok(Next::Load(r)) => pending = Some(r),
                    Ok(Next::Exit) => return,
                    Err(e) => send(&WorkerResponse::Error { id: None, message: format!("{e:#}") }),
                }
            }
        }
    }
}

enum Next {
    Load(WorkerRequest),
    Exit,
}

fn spawn_reader(cancel: Arc<AtomicU64>) -> Receiver<WorkerRequest> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let Ok(line) = line else { break };
            if line.trim().is_empty() {
                continue;
            }
            match decode_request(&line) {
                Ok(WorkerRequest::Cancel { id }) => cancel.store(id, Ordering::Release),
                Ok(r) => {
                    if tx.send(r).is_err() {
                        return;
                    }
                }
                Err(e) => send(&WorkerResponse::Error { id: None, message: e }),
            }
        }
        // stdin closed: the app is gone
        let _ = tx.send(WorkerRequest::Shutdown);
    });
    rx
}

struct PrefixCache {
    text: String,
    tokens: usize,
    state: SeqState,
}

#[allow(clippy::too_many_arguments)]
fn serve_model(
    backend: &LlamaBackend,
    rx: &Receiver<WorkerRequest>,
    cancel: &AtomicU64,
    model_path: &str,
    gpu_layers: u32,
    n_ctx: u32,
    threads: i32,
) -> Result<Next> {
    let t0 = Instant::now();
    let threads = if threads > 0 {
        threads
    } else {
        std::thread::available_parallelism().map(|n| (n.get() / 2).max(1) as i32).unwrap_or(4)
    };
    let cparams = LlamaContextParams::default()
        .with_n_ctx(NonZeroU32::new(n_ctx.max(512)))
        .with_n_batch(N_BATCH)
        .with_n_threads(threads)
        .with_n_threads_batch(threads);
    // The GPU is shared with whatever else the user runs (games, editors), so free
    // VRAM varies. If the model or its context cannot be placed on the GPU, fall
    // back to the CPU once and say so, instead of failing the session.
    let mut layers = gpu_layers;
    let model = loop {
        let params = LlamaModelParams::default().with_n_gpu_layers(layers);
        match LlamaModel::load_from_file(backend, model_path, &params) {
            Ok(m) => {
                if layers == 0 || m.new_context(backend, cparams.clone()).is_ok() {
                    break m;
                }
                eprintln!("kikoe-mt: not enough GPU memory for the context; retrying on CPU");
            }
            Err(e) if layers > 0 => eprintln!("kikoe-mt: GPU model load failed ({e}); retrying on CPU"),
            Err(e) => return Err(anyhow!("loading {model_path}: {e}")),
        }
        layers = 0;
    };
    let mut ctx = model.new_context(backend, cparams).context("creating context")?;
    let mut description = model.meta_val_str("general.name").unwrap_or_else(|_| "unknown".into());
    if gpu_layers > 0 && layers == 0 {
        description.push_str(" (CPU fallback: GPU memory unavailable)");
    }
    send(&WorkerResponse::Loaded {
        model_path: model_path.into(),
        load_ms: t0.elapsed().as_millis() as u64,
        n_ctx: ctx.n_ctx(),
        gpu_layers: layers,
        description,
        size_bytes: model.size(),
    });

    let mut cache: Vec<PrefixCache> = Vec::new();
    loop {
        let req = match rx.recv() {
            Ok(r) => r,
            Err(_) => return Ok(Next::Exit),
        };
        match req {
            WorkerRequest::Shutdown => return Ok(Next::Exit),
            WorkerRequest::Load { .. } => return Ok(Next::Load(req)),
            WorkerRequest::Cancel { .. } => {}
            WorkerRequest::Translate { id, prefix, suffix, max_tokens, stop } => {
                match generate(&model, &mut ctx, &mut cache, cancel, id, &prefix, &suffix, max_tokens, &stop) {
                    Ok(Some(resp)) => send(&resp),
                    Ok(None) => send(&WorkerResponse::Cancelled { id }),
                    Err(e) => {
                        cache.clear();
                        ctx.clear_kv_cache();
                        send(&WorkerResponse::Error { id: Some(id), message: format!("{e:#}") });
                    }
                }
            }
        }
    }
}

fn decode_tokens(ctx: &mut LlamaContext<'_>, batch: &mut LlamaBatch, tokens: &[LlamaToken], start_pos: usize) -> Result<()> {
    for (ci, chunk) in tokens.chunks(N_BATCH as usize).enumerate() {
        batch.clear();
        let last_chunk = (ci + 1) * N_BATCH as usize >= tokens.len();
        for (i, t) in chunk.iter().enumerate() {
            let pos = (start_pos + ci * N_BATCH as usize + i) as i32;
            batch.add(*t, pos, &[0], last_chunk && i == chunk.len() - 1)?;
        }
        ctx.decode(batch).map_err(|e| anyhow!("decode failed: {e}"))?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn generate(
    model: &LlamaModel,
    ctx: &mut LlamaContext<'_>,
    cache: &mut Vec<PrefixCache>,
    cancel: &AtomicU64,
    id: u64,
    prefix: &str,
    suffix: &str,
    max_tokens: u32,
    stop: &[String],
) -> Result<Option<WorkerResponse>> {
    let vocab = model.vocab();
    let t_prompt = Instant::now();
    let suffix_tokens = vocab.tokenize(suffix.as_bytes(), false, true);
    let mut batch = LlamaBatch::new(N_BATCH as usize, 1);

    // 1. restore or build the cached prefix
    let mut cached_tokens = 0u32;
    let prefix_len = if let Some(i) = cache.iter().position(|c| c.text == prefix) {
        let c = cache.remove(i);
        ctx.state_seq_set(&c.state, 0).map_err(|e| anyhow!("restoring prefix state: {e:?}"))?;
        cached_tokens = c.tokens as u32;
        let n = c.tokens;
        cache.push(c); // most recently used at the end
        n
    } else {
        ctx.clear_kv_cache();
        let prefix_tokens = vocab.tokenize(prefix.as_bytes(), true, true);
        decode_tokens(ctx, &mut batch, &prefix_tokens, 0)?;
        let state = ctx.state_seq_get(0, LlamaStateSeqFlags::empty()).map_err(|e| anyhow!("saving prefix state: {e:?}"))?;
        if cache.len() >= PREFIX_CACHE {
            cache.remove(0);
        }
        cache.push(PrefixCache { text: prefix.to_string(), tokens: prefix_tokens.len(), state });
        prefix_tokens.len()
    };
    let total_prompt = prefix_len + suffix_tokens.len();
    if total_prompt + max_tokens as usize + 8 > ctx.n_ctx() as usize {
        return Err(anyhow!("prompt of {total_prompt} tokens does not fit the context"));
    }
    if cancel.load(Ordering::Acquire) == id {
        return Ok(None);
    }
    // 2. evaluate the per-line suffix
    decode_tokens(ctx, &mut batch, &suffix_tokens, prefix_len)?;
    let prompt_ms = t_prompt.elapsed().as_secs_f64() * 1000.0;

    // 3. greedy decoding (deterministic, no sampling randomness in subtitles)
    let t_gen = Instant::now();
    let mut sampler = LlamaSampler::greedy();
    let mut out_bytes: Vec<u8> = Vec::new();
    let mut pos = total_prompt;
    let mut generated = 0u32;
    let mut finish = "max_tokens";
    let mut logits_idx = batch.n_tokens() - 1;
    while generated < max_tokens {
        if cancel.load(Ordering::Acquire) == id {
            return Ok(None);
        }
        let token = sampler.sample(ctx, logits_idx);
        sampler.accept(token);
        generated += 1;
        if vocab.is_eog(token) {
            finish = "eog";
            break;
        }
        out_bytes.extend(vocab.token_to_piece(token, true, None));
        let text = String::from_utf8_lossy(&out_bytes);
        if let Some(cut) = stop.iter().filter_map(|s| text.find(s.as_str())).min() {
            out_bytes.truncate(cut);
            finish = "stop_string";
            break;
        }
        batch.clear();
        batch.add(token, pos as i32, &[0], true)?;
        ctx.decode(&mut batch).map_err(|e| anyhow!("decode failed: {e}"))?;
        logits_idx = 0;
        pos += 1;
    }
    let text = String::from_utf8_lossy(&out_bytes).into_owned();
    Ok(Some(WorkerResponse::Translated {
        id,
        text,
        prompt_tokens: total_prompt as u32,
        cached_tokens,
        generated_tokens: generated,
        prompt_ms,
        generate_ms: t_gen.elapsed().as_secs_f64() * 1000.0,
        finish: finish.into(),
    }))
}
