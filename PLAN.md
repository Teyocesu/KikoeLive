# PLAN.md — mutable implementation plan

Updated 2026-11-17 (Windows implementation round, branch `impl/integrated-app`).

## Done in this round
- [x] Shared pipeline crate with unit + simulation tests (`crates/kikoe-core`).
- [x] WASAPI loopback adapter (validated); ScreenCaptureKit adapter written (uncompiled).
- [x] whisper.cpp ASR + Silero VAD adapters; llama.cpp translation worker process.
- [x] Evaluation harness (text MT, ASR, full pipeline from file, live loopback).
- [x] Tauri app: control window, floating overlay, model manager, live switching.
- [x] Windows toolchain, CUDA build, portable test artifact, CI workflow (unrun).
- [x] EN and ES evaluated separately; translator chosen by measurement (EVIDENCE.md).

## Next (in priority order)
1. **macOS round on the M1:** compile `kikoe-capture` (ScreenCaptureKit) and the
   app with `--features metal`, run `kikoe-eval` (mt, asr, pipeline, live),
   measure memory with Qwen3.5-4B on 8 GiB. Only if it does not fit, evaluate a
   smaller translator for both languages again before switching.
2. **Real-stream evaluation:** a few minutes of authorised stream audio with a
   human-checked Japanese transcript and independent EN/ES references; human
   review of the error classification in EVIDENCE.md.
3. **Spanish quality:** the main gap (11/88 vs 4/88). Candidates: larger model
   where memory allows (GPU-only), targeted prompt work validated on held-out
   data, or a dedicated ja→es model with a product-compatible licence.
4. **CPU-only Whisper performance** (≈230 s/utterance in this build): profile the
   whisper-rs-sys CPU path; until fixed, the CPU fallback is not usable.
5. Reconcile with the Phase 0/1 history from the Mac; port any Phase 1 regression
   cases not already covered.
6. Signed installers (owner decision; needs credentials). NSIS bundling exists in
   `tauri.conf.json` but was not run.

## Open questions for the owner
- Accept Qwen3.5-4B (3.0 GB) as the single translator for both languages?
- Ship the CUDA runtime (≈0.77 GB) in the Windows package, or CPU/GPU variants?
