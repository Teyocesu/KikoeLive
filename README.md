# KikoeLive

Live English or Spanish subtitles for Japanese audio playing on your computer
(VTuber streams, YouTube, Twitch…), computed entirely on-device.

* System audio only (WASAPI loopback on Windows, ScreenCaptureKit on macOS) —
  never the microphone.
* Whisper (whisper.cpp) for Japanese speech recognition, Silero VAD for speech
  detection, a local Qwen LLM (llama.cpp) for Japanese → English / Español.
* Floating, resizable, always-on-top subtitle overlay; switch English ↔ Español
  while it runs.
* No accounts, telemetry or cloud; no audio or transcript is saved.

Status and measured quality: see [HANDOFF.md](HANDOFF.md) and [EVIDENCE.md](EVIDENCE.md).
This is an unsigned development build, not a release.

## Layout
| path | what |
|---|---|
| `crates/kikoe-core` | shared, platform-independent pipeline (+ unit tests) |
| `crates/kikoe-capture` | WASAPI / ScreenCaptureKit adapters, file source for evaluation |
| `crates/kikoe-asr` | whisper.cpp ASR + Silero VAD adapters |
| `crates/kikoe-mt` | translation worker process (llama.cpp) |
| `crates/kikoe-eval` | evaluation harness (same engines and pipeline as the app) |
| `app/` | Tauri 2 app: React UI (`src/`), Rust backend (`src-tauri/`) |
| `eval/corpus` | evaluation text corpora (no audio) |
| `scripts/` | dev environment, packaging, evaluation data preparation |

## Build (Windows x64)
Prerequisites: Rust (rustup, MSVC), Node 22+, Visual Studio 2022 Build Tools with
"Desktop development with C++" and "C++ Clang Compiler for Windows", optionally
CUDA Toolkit 12.x for GPU.

```powershell
. .\scripts\dev-env.ps1                  # PATH, CMake, libclang, CUDA for this shell only
cargo test -p kikoe-core -p kikoe-eval   # unit tests
powershell -ExecutionPolicy Bypass -File scripts\package-windows.ps1   # -> dist\windows\KikoeLive
```
Add `-Cpu` for a CPU-only build, `-Bundle` for an NSIS installer.

## Build (macOS, Apple Silicon)
```sh
cargo build --release -p kikoe-mt --features metal
mkdir -p app/src-tauri/binaries && cp target/release/kikoe-mt app/src-tauri/binaries/kikoe-mt-aarch64-apple-darwin
cd app && npm ci && npx tauri build --features metal
```
The macOS path has not been compiled or run in this round (see HANDOFF.md).

## First run
1. Start KikoeLive. Under *Language packs & models* press **Download** for each
   required model (sizes are shown first), or **Use existing file…** to point at a
   copy you already have (it is verified, not copied).
2. Play a Japanese stream, choose **English** or **Español**, press **Start**.
3. Drag the overlay where you like; its corner resizes it. Use the main window to
   hide/restore it, change opacity/text size, or reset its position.
