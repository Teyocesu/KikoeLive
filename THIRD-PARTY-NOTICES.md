# Third-party notices

KikoeLive's own code is MIT-licensed. It builds on the components below. Model
weights and datasets are **not** stored in this repository; models are downloaded
by the user from the pinned sources listed here and verified by SHA-256.

## Native inference libraries (compiled into the app / worker)
| component | licence | used for |
|---|---|---|
| whisper.cpp (ggml-org), via `whisper-rs` / `whisper-rs-sys` 0.16 / 0.15 | MIT (whisper.cpp), Unlicense (bindings) | Japanese ASR, Silero VAD runtime |
| llama.cpp (ggml-org), via `llama-cpp-2` / `llama-cpp-sys-2` 0.1.161 | MIT (llama.cpp), MIT OR Apache-2.0 (bindings) | translation worker |
| ggml | MIT | tensor runtime inside both |

## Application frameworks
Tauri 2.12 and `tauri-plugin-dialog` 2.8 (Apache-2.0 OR MIT), React 19 (MIT),
Vite 8 (MIT, build-time only), `windows` 0.62 (MIT OR Apache-2.0),
`screencapturekit` 11 (MIT OR Apache-2.0), reqwest / native-tls, serde, sha2,
crossbeam, hound (Apache-2.0), dirs, log, env_logger, anyhow, thiserror.

Dependency licence audit (2026-11-17, `cargo metadata` for x86_64-pc-windows-msvc,
307 crates): MIT/Apache-2.0 variants, Unicode-3.0, Unlicense, BSD-3-Clause, Zlib,
and MPL-2.0 (file-level copyleft, unmodified). npm production + dev packages: MIT,
Apache-2.0, ISC, BSD-3-Clause, MPL-2.0. No GPL/AGPL or non-commercial licences.

## NVIDIA CUDA runtime (Windows GPU build only)
`cudart64_12.dll`, `cublas64_12.dll`, `cublasLt64_12.dll` from CUDA Toolkit 12.9
are copied next to the executables in the portable test build. They are listed as
redistributable in the NVIDIA CUDA Toolkit EULA (Attachment A) and remain under
NVIDIA's licence. CPU-only builds do not include them.

## Models (downloaded by the user, never committed)
| id | source (pinned revision) | licence |
|---|---|---|
| whisper-large-v3-turbo-q5_0 | huggingface.co/ggerganov/whisper.cpp @ 5359861c739e955e79d9a303bcbc70fb988958b1 | MIT (OpenAI Whisper weights) |
| whisper-small-q5_1 | same | MIT |
| silero-vad-v6.2.0 | huggingface.co/ggml-org/whisper-vad @ 9ffd54a1e1ee413ddf265af9913beaf518d1639b | MIT (Silero VAD) |
| qwen3-4b-q4_k_m | huggingface.co/Qwen/Qwen3-4B-GGUF @ bc640142c66e1fdd12af0bd68f40445458f3869b | Apache-2.0 |
| qwen3.5-4b-q4_k_m | huggingface.co/ggml-org/Qwen3.5-4B-GGUF @ 8d43d974efc48399cb58957ad49c8b26bf5a109c (conversion of Qwen/Qwen3.5-4B) | Apache-2.0 |

SHA-256 values are in `crates/kikoe-core/src/models.rs`.

## Evaluation data
* `eval/corpus/fleurs_dev_subset.jsonl` contains 40 sentences from **FLEURS**
  (Google, CC-BY 4.0; Japanese transcriptions) whose English (en_us) and Spanish
  (es_419) rows are **FLoRes** translations (Meta, CC-BY-SA 4.0). This file is
  therefore distributed under CC-BY-SA 4.0 with attribution to both. The FLEURS
  audio is not redistributed.
* `eval/corpus/conversational.jsonl`: Japanese lines and EN/ES references written
  for this project (MIT, like the code). The references are the implementer's own
  and have **not** been checked by a professional translator.
* Synthetic test audio is generated locally with the Japanese voices bundled with
  Windows (Microsoft Ayumi / Haruka / Ichiro) by `scripts/make_tts_corpus.ps1`;
  it is used for local testing only and is not committed or redistributed.
