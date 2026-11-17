# HANDOFF.md — current state (2026-11-17)

Branch `impl/integrated-app` (new history; the GitHub remote was empty and the
Phase 0/1 repository was not on this machine). Nothing pushed.

## What exists and works (Windows 11, Ryzen 7 7735HS, RTX 4050 — the target PC)
* **KikoeLive desktop app** (Tauri 2 + React + Rust): main window + floating
  overlay; Start/Stop; English ↔ Español while running; optional Japanese line;
  model manager (pinned downloads, SHA-256, reuse existing files); diagnostics.
  Portable test build: `dist\windows\KikoeLive\KikoeLive.exe` (unsigned).
* **Pipeline** (`crates/kikoe-core`): WASAPI loopback → resample 16 kHz →
  Silero VAD → segmenter → Whisper large-v3-turbo (CUDA) → filters → Qwen3.5-4B
  translator in a separate worker process (CUDA) → coordinator → UI.
* Measured on this PC (EVIDENCE.md): stream CER 0.032 on synthetic speech (also
  under BGM); detected speech end → subtitle ≈1.0–1.3 s median with an idle-ish
  GPU; 0 stale-language subtitles across switches; Stop 9–771 ms; worker crash
  recovered in ≈7 s.

## Quality status (honest)
* ja→en: 4/88 lines with a major meaning error on correct Japanese text.
* ja→es: 11/88 — clearly weaker (person/subject and idiom errors).
* Reviewed by the implementer, not a human translator; no real VTuber
  audio was available, so real-stream quality is **unknown**.

## Known problems / blockers
1. **macOS untested**: ScreenCaptureKit adapter written against
   `screencapturekit` 11 but never compiled; Metal build never run; M1 memory
   with the 3 GB translator unknown.
2. **CPU-only Whisper is unusably slow** in this build (≈230 s per utterance) —
   root cause not found. GPU is required on Windows today.
3. The RTX 4050 here is shared with other heavy GPU apps; with them active the
   translator slowed 4–5× and the overload policy skipped 1 line (EVIDENCE §4).
4. Translator private memory ≈4.8 GB on Windows/CUDA (not investigated).
5. whisper-rs 0.16 `set_abort_callback_safe` is unsound (worked around).

## How to run
```powershell
. .\scripts\dev-env.ps1
powershell -ExecutionPolicy Bypass -File scripts\package-windows.ps1
dist\windows\KikoeLive\KikoeLive.exe
```
Models: the app lists what is missing; Download or "Use existing file…".
Evaluation: `scripts\run-eval.ps1` (see header for data preparation).

## Physical Windows QA checklist (RTX 4050 machine) — short
1. Launch `KikoeLive.exe`; all four required models show "✓ Ready".
2. Play a Japanese YouTube stream; choose the output device you hear it on; Start.
3. Within ~2 s of each phrase the overlay shows Japanese + English.
4. Click Español mid-stream: next lines Spanish, no English line appears after.
5. Drag the overlay, resize from its corner, change opacity/size, hide with ×,
   restore with "Reset overlay position".
6. Kill `kikoe-mt.exe` in Task Manager: "Recovering", then subtitles resume.
7. Stop; `kikoe-mt.exe` disappears. Close the window; no KikoeLive process remains.
8. Untick "Use GPU acceleration" only if you want to see the CPU fallback (slow).

## Most important next step
Run the same build + evaluation on the **M1 Mac** (compile the ScreenCaptureKit
adapter, Metal, measure memory with Qwen3.5-4B) and evaluate on a few minutes of
**real, authorised stream audio with a human-checked transcript and EN/ES
references** — that is what decides whether quality is acceptable for release.
