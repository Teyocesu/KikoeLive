# ADR 0001 — Runtime architecture for the integrated app

Date: 2026-11-17 · Status: accepted

## Context
The brief prefers Tauri 2 + React/TS + Rust with native capture, whisper.cpp and
a local multilingual translator. Phase 0/1 used a Python/Tk harness with
CTranslate2 OPUS-MT; translation fidelity was the main failure. The Phase 0/1
repository was not available on the implementation machine (the GitHub remote is
empty), so this implementation starts a new history on Windows; Phase 1 lessons
are re-implemented from the brief (see HANDOFF.md).

## Decision
* **Shell:** Tauri 2 (Rust backend, React + TypeScript UI, two native windows:
  controls and a transparent always-on-top overlay). No Python at runtime.
* **Shared pipeline:** `crates/kikoe-core` — platform-independent Rust: PCM
  contract, resampler, segmenter, filters, language control with epochs,
  prompts/sanitising, coordinator, bounded queues, metrics, model catalog.
* **Capture adapters:** `crates/kikoe-capture` — WASAPI loopback (Windows) and
  ScreenCaptureKit audio (macOS) behind one `CaptureStream` trait.
* **ASR + VAD:** whisper.cpp through `whisper-rs`, in-process (CUDA on Windows,
  Metal on macOS, CPU fallback via a setting). Silero VAD through whisper.cpp's
  ggml port, so no second inference runtime is added for VAD.
* **Translation:** a quantised instruction LLM (Qwen family, Apache-2.0, GGUF)
  run by llama.cpp (`llama-cpp-2`) in a **separate worker process** (`kikoe-mt`)
  speaking JSON lines over stdin/stdout.
  * whisper.cpp and llama.cpp each embed their own copy of ggml; linking both
    into one binary risks duplicate-symbol/ABI conflicts. A process boundary
    removes that risk.
  * A hung or crashed model is killed and restarted without losing the app or
    capture; memory is returned to the OS on Stop.
  * No sockets or listeners: only the worker's own pipes.
* **One model for both languages:** the same translator serves `ja-en` and
  `ja-es` with language-specific prompts, so a language switch costs only a
  prompt-prefix rebuild — never a second multi-GB model in memory (important for
  the 8 GiB M1).
* **Prompt caching:** the worker snapshots the sequence state after the
  language-constant prompt prefix and restores it per line (works for transformer
  and hybrid/recurrent models alike).

## Consequences
* Two native inference libraries are compiled (whisper.cpp, llama.cpp); both are
  ggml-based, so GPU support and quantisation formats stay consistent.
* Windows CUDA builds need the CUDA runtime DLLs next to the executables
  (redistributable under the CUDA EULA, ≈0.77 GB for cudart + cuBLAS + cuBLASLt 12.9); a CPU-only build avoids that.
* The worker is a Tauri `externalBin` sidecar; the app spawns it only from its own
  install directory.
