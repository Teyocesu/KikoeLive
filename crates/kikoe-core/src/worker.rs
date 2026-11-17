//! Client side of the `kikoe-mt` translation worker process.
//!
//! The worker is started from a fixed executable path chosen by the app (never
//! from user input), with stdin/stdout pipes only. If it hangs past the request
//! timeout or exits, it is killed and the next `prepare` starts a fresh one.

use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, RecvTimeoutError, unbounded};

use crate::lang::TargetLanguage;
use crate::pipeline::{CancelFlags, TranslateError, TranslationEngine, TranslationOutput};
use crate::protocol::{WorkerRequest, WorkerResponse, decode_response, encode_line};
use crate::translate::{PromptStyle, RenderedPrompt};

#[derive(Clone, Debug)]
pub struct WorkerModel {
    pub path: PathBuf,
    pub style: PromptStyle,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub exe: PathBuf,
    pub models: HashMap<TargetLanguage, WorkerModel>,
    pub gpu_layers: u32,
    pub n_ctx: u32,
    pub threads: i32,
    pub request_timeout: Duration,
    pub load_timeout: Duration,
}

struct Proc {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<Result<WorkerResponse, String>>,
    backend: String,
}

pub struct WorkerTranslator {
    cfg: WorkerConfig,
    proc: Option<Proc>,
    loaded: Option<PathBuf>,
    next_id: u64,
    stderr_tail: Arc<Mutex<VecDeque<String>>>,
    spawns: u32,
    /// What the worker reported for the loaded model (name, GPU layers actually used).
    loaded_info: Option<(String, u32)>,
}

impl WorkerTranslator {
    pub fn new(cfg: WorkerConfig) -> Self {
        Self {
            cfg,
            proc: None,
            loaded: None,
            next_id: 1,
            stderr_tail: Arc::new(Mutex::new(VecDeque::new())),
            spawns: 0,
            loaded_info: None,
        }
    }

    pub fn backend(&self) -> Option<&str> {
        self.proc.as_ref().map(|p| p.backend.as_str())
    }

    pub fn spawn_count(&self) -> u32 {
        self.spawns
    }

    /// (model description, GPU layers actually used) of the loaded model.
    pub fn loaded_info(&self) -> Option<&(String, u32)> {
        self.loaded_info.as_ref()
    }

    pub fn pid(&self) -> Option<u32> {
        self.proc.as_ref().map(|p| p.child.id())
    }

    /// Last lines the worker wrote to stderr (llama.cpp logs), for error reports.
    pub fn stderr_tail(&self) -> Vec<String> {
        self.stderr_tail.lock().unwrap().iter().cloned().collect()
    }

    fn spawn(&mut self) -> Result<(), String> {
        let mut cmd = Command::new(&self.cfg.exe);
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        let mut child = cmd.spawn().map_err(|e| format!("cannot start translation worker {:?}: {e}", self.cfg.exe))?;
        self.spawns += 1;
        let stdin = child.stdin.take().ok_or("worker stdin unavailable")?;
        let stdout = child.stdout.take().ok_or("worker stdout unavailable")?;
        let stderr = child.stderr.take().ok_or("worker stderr unavailable")?;
        let (tx, rx) = unbounded();
        std::thread::Builder::new()
            .name("kikoe-mt-reader".into())
            .spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    match line {
                        Ok(l) if l.trim().is_empty() => continue,
                        Ok(l) => {
                            if tx.send(decode_response(&l)).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        let tail = self.stderr_tail.clone();
        std::thread::Builder::new()
            .name("kikoe-mt-stderr".into())
            .spawn(move || {
                for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                    let mut t = tail.lock().unwrap();
                    t.push_back(line);
                    while t.len() > 40 {
                        t.pop_front();
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        let mut p = Proc { child, stdin, rx, backend: String::new() };
        match p.rx.recv_timeout(Duration::from_secs(20)) {
            Ok(Ok(WorkerResponse::Hello { backend, protocol, .. })) => {
                if protocol != crate::protocol::PROTOCOL_VERSION {
                    let _ = p.child.kill();
                    return Err(format!("worker protocol {protocol} does not match app"));
                }
                p.backend = backend;
            }
            other => {
                let _ = p.child.kill();
                let _ = p.child.wait();
                return Err(format!("worker did not start correctly: {other:?}; {}", self.stderr_tail().join(" | ")));
            }
        }
        self.proc = Some(p);
        self.loaded = None;
        Ok(())
    }

    fn send(&mut self, req: &WorkerRequest) -> Result<(), String> {
        let p = self.proc.as_mut().ok_or("worker not running")?;
        p.stdin.write_all(encode_line(req).as_bytes()).and_then(|_| p.stdin.flush()).map_err(|e| format!("worker pipe closed: {e}"))
    }

    fn kill(&mut self) {
        if let Some(mut p) = self.proc.take() {
            let _ = p.child.kill();
            let _ = p.child.wait();
        }
        self.loaded = None;
    }

    fn model_for(&self, lang: TargetLanguage) -> Result<&WorkerModel, String> {
        self.cfg.models.get(&lang).ok_or_else(|| format!("no translation model installed for {}", lang.native_name()))
    }
}

impl TranslationEngine for WorkerTranslator {
    fn prepare(&mut self, lang: TargetLanguage) -> Result<Option<u64>, String> {
        let model = self.model_for(lang)?.clone();
        let alive = self.proc.as_mut().is_some_and(|p| matches!(p.child.try_wait(), Ok(None)));
        if !alive {
            self.kill();
            self.spawn()?;
        }
        if self.loaded.as_ref() == Some(&model.path) {
            return Ok(None);
        }
        let t0 = Instant::now();
        self.send(&WorkerRequest::Load {
            model_path: model.path.to_string_lossy().into_owned(),
            gpu_layers: self.cfg.gpu_layers,
            n_ctx: self.cfg.n_ctx,
            threads: self.cfg.threads,
        })?;
        let deadline = t0 + self.cfg.load_timeout;
        loop {
            let rx = self.proc.as_ref().ok_or("worker not running")?.rx.clone();
            match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
                Ok(Ok(WorkerResponse::Loaded { description, gpu_layers, .. })) => {
                    self.loaded = Some(model.path.clone());
                    self.loaded_info = Some((description, gpu_layers));
                    return Ok(Some(t0.elapsed().as_millis() as u64));
                }
                Ok(Ok(WorkerResponse::Error { message, .. })) => {
                    self.kill();
                    return Err(format!("could not load {}: {message}", model.label));
                }
                Ok(Ok(_)) => continue,
                Ok(Err(e)) => {
                    self.kill();
                    return Err(e);
                }
                Err(_) => {
                    let tail = self.stderr_tail().join(" | ");
                    self.kill();
                    return Err(format!("loading {} timed out or the worker exited: {tail}", model.label));
                }
            }
        }
    }

    fn prompt_style(&self, lang: TargetLanguage) -> PromptStyle {
        self.cfg.models.get(&lang).map(|m| m.style).unwrap_or(PromptStyle::ChatMlNoThink)
    }

    fn translate(&mut self, prompt: &RenderedPrompt, stop: &[String], cancel: &CancelFlags<'_>) -> Result<TranslationOutput, TranslateError> {
        if self.proc.is_none() || self.loaded.is_none() {
            return Err(TranslateError::WorkerDied("worker not ready".into()));
        }
        let id = self.next_id;
        self.next_id += 1;
        self.send(&WorkerRequest::Translate {
            id,
            prefix: prompt.prefix.clone(),
            suffix: prompt.suffix.clone(),
            max_tokens: prompt.max_tokens,
            stop: stop.to_vec(),
        })
        .map_err(|e| {
            self.kill();
            TranslateError::WorkerDied(e)
        })?;
        let rx = self.proc.as_ref().unwrap().rx.clone();
        let started = Instant::now();
        let mut cancel_sent = false;
        loop {
            match rx.recv_timeout(Duration::from_millis(15)) {
                Ok(Ok(WorkerResponse::Translated { id: rid, text, prompt_tokens, cached_tokens, generated_tokens, prompt_ms, generate_ms, .. }))
                    if rid == id =>
                {
                    if cancel_sent {
                        return Err(TranslateError::Cancelled);
                    }
                    return Ok(TranslationOutput { raw: text, prompt_tokens, cached_tokens, generated_tokens, prompt_ms, generate_ms });
                }
                Ok(Ok(WorkerResponse::Cancelled { id: rid })) if rid == id => return Err(TranslateError::Cancelled),
                Ok(Ok(WorkerResponse::Error { id: Some(rid), message })) if rid == id => return Err(TranslateError::Failed(message)),
                Ok(Ok(_)) => continue, // late reply to an older, cancelled request
                Ok(Err(e)) => {
                    self.kill();
                    return Err(TranslateError::WorkerDied(e));
                }
                Err(RecvTimeoutError::Timeout) => {
                    if cancel.is_set() && !cancel_sent {
                        cancel_sent = true;
                        if self.send(&WorkerRequest::Cancel { id }).is_err() {
                            self.kill();
                            return Err(TranslateError::WorkerDied("pipe closed".into()));
                        }
                    }
                    if cancel_sent && started.elapsed() > Duration::from_secs(2) + self.cfg.request_timeout / 4 {
                        // worker ignored the cancel: treat as hung
                        self.kill();
                        return Err(TranslateError::WorkerDied("worker did not acknowledge cancel".into()));
                    }
                    if started.elapsed() > self.cfg.request_timeout {
                        self.kill();
                        return Err(TranslateError::WorkerDied(format!("no reply within {:?}", self.cfg.request_timeout)));
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    let tail = self.stderr_tail().join(" | ");
                    self.kill();
                    return Err(TranslateError::WorkerDied(format!("worker exited: {tail}")));
                }
            }
        }
    }

    fn shutdown(&mut self) {
        if self.proc.is_some() {
            let _ = self.send(&WorkerRequest::Shutdown);
            let t0 = Instant::now();
            while t0.elapsed() < Duration::from_millis(300) {
                if let Some(p) = self.proc.as_mut() {
                    if matches!(p.child.try_wait(), Ok(Some(_))) {
                        break;
                    }
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        self.kill();
    }

    fn describe(&self) -> String {
        let labels: Vec<String> = TargetLanguage::ALL
            .iter()
            .filter_map(|l| self.cfg.models.get(l).map(|m| format!("{}: {}", l.code(), m.label)))
            .collect();
        let mut d = labels.join(", ");
        if let Some((desc, layers)) = &self.loaded_info {
            d.push_str(&format!(" [{}{}]", if *layers > 0 { "GPU" } else { "CPU" }, if desc.contains("fallback") { ", fallback" } else { "" }));
        }
        d
    }
}

impl Drop for WorkerTranslator {
    fn drop(&mut self) {
        self.kill();
    }
}
