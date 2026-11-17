# PLAN.md — mutable implementation plan

Updated 2026-11-17 (Windows implementation round, branch `impl/integrated-app`).

## Done in this round
- [x] Shared pipeline crate with unit + simulation tests (`crates/kikoe-core`).
- [x] WASAPI loopback adapter; ScreenCaptureKit adapter written (uncompiled).
- [x] whisper.cpp ASR + Silero VAD adapters; llama.cpp translation worker process.
- [x] Evaluation harness (text MT, ASR, full pipeline from file, live loopback).
- [x] Tauri app: control window, floating overlay, model manager, live switching.
- [x] Windows toolchain, CUDA build, portable test artifact script, CI workflow.
- [x] Measured EN and ES quality / latency (see EVIDENCE.md).

## Next (in priority order)
1. **macOS round on the M1:** compile `kikoe-capture` (ScreenCaptureKit), build
   with `--features metal`, run the same `kikoe-eval` commands, measure memory
   with the 4B translator on 8 GiB; fall back to a smaller translator only if
   measurements require it (re-run the EN/ES evaluation for that model).
2. Independent human review of the EN and ES references and of the error
   classification in EVIDENCE.md; add real (authorised) VTuber stream clips with
   human transcripts to the evaluation set.
3. Reconcile this history with the Phase 0/1 repository from the Mac (it was not
   available here): port any Phase 1 regression cases not covered by the tests.
4. Signed installers for both platforms (owner decision; needs credentials).

## Open questions for the owner
- Accept Qwen3.5-4B (3.0 GB) as the single translator for both languages, or
  ship a smaller default for 8 GiB machines?
- Bundle the CUDA runtime (~0.5 GB) in the Windows installer, or ship CPU-only
  plus an optional GPU pack?
