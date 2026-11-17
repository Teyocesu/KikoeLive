# EVIDENCE.md — what has actually been measured

Evidence labels: **[test]** automated test · **[run]** real execution with real
engines · **[visual]** manual visual check (screenshot) · **[static]** code
inspection · **[sim]** simulation with stand-in engines · **[n/t]** not tested.

## 0. Environment of this round (2026-11-17)
* Machine: the **Windows target** — AMD Ryzen 7 7735HS, 31.2 GiB RAM, NVIDIA
  GeForce RTX 4050 Laptop (6 GiB, driver 617.42), Windows 11 Home 10.0.26300.
  The M1 MacBook Air was **not available**; no macOS result in this file.
* Phase 0/1 repository: **not available** (GitHub `Teyocesu/KikoeLive` was empty;
  no local copy). Phase 0/1 figures quoted in the brief (0.95 s / 1.29 s median
  post-speech latency, 322 MiB RSS, 73 ms Stop, OPUS-MT quality FAIL) could not be
  re-verified and are treated as historical claims only.
* Toolchain installed with the owner's approval: rustup + Rust 1.100.0 (per-user),
  VS 2022 Build Tools 17.14 (MSVC 14.44, CMake 3.31, Clang/libclang component),
  CUDA Toolkit 12.9.86 (compiler, cudart, cuBLAS, CCCL, VS integration only — the
  display driver was not touched and is still 617.42). Node 24.15 was present.
  Disk: C: free 150 → 129.9 GB after installs; D: (project) 428 GB free.
* Model files (all SHA-256 verified against the Hugging Face LFS records):
  * reused without copying from another local app (ClassScribe):
    `ggml-large-v3-turbo-q5_0.bin` (394221709c…), `ggml-small-q5_1.bin`
    (ae85e4a935…), `ggml-silero-v6.2.0.bin` (2aa269b785…);
  * downloaded (owner-approved, ~5.5 GB) into `%LOCALAPPDATA%\KikoeLive\models`:
    `Qwen3-4B-Q4_K_M.gguf` (7485fe6f11…, official Qwen repo),
    `Qwen3.5-4B-Q4_K_M.gguf` (217002fa80…, ggml-org conversion of Qwen/Qwen3.5-4B).
  * Both GGUF chat templates disable reasoning with an empty
    `<think>\n\n</think>\n\n` block, which is what the prompt renderer emits
    [static: GGUF metadata read].

## 1. Japanese ASR (Whisper via whisper.cpp, CUDA) — [run]
`kikoe-eval asr`, each WAV transcribed as one segment, CER after removing
punctuation/spaces (full tables: `eval/results/asr-*.md`).

| set | model / decoding | CER | median ASR time | p95 |
|---|---|---|---|---|
| FLEURS ja dev, 40 real read-speech recordings (8–17 s) | large-v3-turbo q5_0, greedy | **0.041** | 429 ms | 572 ms |
| same | large-v3-turbo q5_0, beam 5 | 0.038 | 867 ms | 1319 ms |
| same | small q5_1, greedy | 0.134 | 499 ms | 773 ms |
| 48 synthetic conversational lines (Windows TTS, 3 voices, 1.1–1.35× rate) | large-v3-turbo q5_0, greedy | 0.032 | 211 ms | 244 ms |
| FLEURS, first 2 recordings, **CPU only**, 8 threads | large-v3-turbo q5_0 | 0.061 | **≈230 s** per utterance | — |

* Decision: large-v3-turbo greedy (beam 5 halves throughput for −0.3 CER points;
  small is 3× worse).
* **CPU-only ASR is unusable in this build** (≈230 s per 10–15 s utterance; a
  correctly vectorised whisper.cpp should be one to two orders of magnitude
  faster). AVX2 is enabled in the ggml-cpu project; the root cause was not found
  this round. CPU fallback for ASR: **FAIL**.
* Bug found by the real-engine run and fixed: whisper-rs 0.16
  `set_abort_callback_safe` reinterprets a boxed `dyn FnMut` as the concrete
  closure (undefined behaviour) and aborted every encode (whisper error -6). The
  adapter now uses the raw hook with a typed trampoline (`crates/kikoe-asr`).
  The pipeline simulation tests could not catch this (stand-in engines).

## 2. Translation on correct Japanese text — [run] + manual review
`kikoe-eval mt` with the app's exact prompts (`crates/kikoe-core/src/translate.rs`),
greedy decoding, worker on CUDA. chrF is automatic; **error counts are a manual
review by the implementer, not by a human translator.** A "major"
error changes meaning: inverted/missing negation (NEG), wrong subject/person
(SUBJ), wrong action/order (ACT), wrong entity/name (ENT), invented content
(INV), context leaked into the line (CTX), echo/omission (OMIT).

### 2.1 Candidates (prompt v1) — chrF
| set | Qwen3-4B Q4_K_M | Qwen3.5-4B Q4_K_M |
|---|---|---|
| conversational 48 → EN | 59.6 | **70.3** |
| conversational 48 → ES | 48.8 | **56.9** |
| FLEURS 40 → EN | 50.6 | **53.3** |
| FLEURS 40 → ES | 40.4 | **42.8** |

Major errors, conversational, v1: Qwen3-4B EN ≈6 (incl. lost negation c35,
お疲れ misread c28, context leak p1b), ES ≈10; Qwen3.5-4B EN 3, ES 7.
**Qwen3.5-4B selected.** (Qwen3-4B is the measured baseline; the Phase 1 OPUS-MT
baseline could not be re-run: its code/models were not available here.)

### 2.2 Final configuration (Qwen3.5-4B, prompt v2, direct, plausibility guard)
| | conversational (48) | FLEURS (40, independent refs) | total major | median / p95 MT time (conv) | (FLEURS) |
|---|---|---|---|---|---|
| **ja→en** | chrF 70.9 · 2 major | chrF 53.0 · 2 major | **4 / 88 (4.5 %)** | 258 / 372 ms | 638 / 1181 ms |
| **ja→es** | chrF 57.2 · 6 major | chrF 43.1 · 5 major | **11 / 88 (12.5 %)** | 297 / 455 ms | 756 / 2108 ms |

English major errors: c05 怖くないわけじゃない → "It's not scary" (NEG, double
negation lost); c34 嘘でしょ、また落ちた → "That's a lie, you fell again" (SUBJ +
idiom); f1510 国連では…考えています → "We would like…" (SUBJ); f1563 ひな (chick)
→ "Hinano" (ENT).

Spanish major errors: c04 全然怖くないし → "¡No tiene miedo en absoluto!" (SUBJ:
3rd person); c13 鍵を取ってから戻ろう → "Primero me quito la llave…" (ACT);
c26 先輩に怒られちゃった → "El senpai me enojó" (ACT: "made me angry" vs "scolded
me"); c28 お疲れぺこ → "¡Gracias, Pekora!" (SUBJ: addresses the speaker);
c34 → "¡Miente!" (idiom → "he/she lies"); c35 声が聞こえないって → "¿No se oye tu
voz?" (SUBJ: your vs my); f1510 → "Quiero…" (SUBJ); f1513 ポロ → "fútbol" (ENT);
f1552 プロット分析 → "análisis del prototipo" (ENT); f1563 → "hinano" (ENT);
f1566 中米 → "América Latina y el Caribe" (ENT/INV).

Correct where it matters (examples): negation c01/c06/c11/c12/c20/c21/c22/c30/
c39/p2b/p3b/p4b in both languages; c05 double negation is **correct in Spanish**
("No es que no tenga miedo, pero estoy bien") although wrong in English;
self-reference "Pekora isn't playing a horror game today" (c01); names
Captain Marine / hololive / superchat; context pairs p1b/p4b translated without
repeating the context.

Comparison table with all references and outputs: `eval/results/mt2-conversational-direct.md`,
`eval/results/mt2-fleurs-direct.md`, `eval/results/mt3-*-es-direct-guard.md`.

### 2.3 Measured and rejected: English-assisted Spanish (pivot hint)
Japanese line (source of truth) + the model's own English draft → Spanish.
| | direct ES | assisted ES |
|---|---|---|
| major errors conv / FLEURS | 6 / 5 (guarded) | 6 / 4 |
| median MT time conv / FLEURS | 289 / 777 ms | 557 / 1519 ms |
It fixed some Spanish-only errors (polo, c13, c28) but inherited English ones
(c05 → "No es tan aterrador") and added an inversion (c27 → "No les gusto mucho").
Gain within noise at 2× latency → **not adopted**; kept only as a retry when a
direct Spanish output fails the plausibility guard.

### 2.4 Guard added because of a measured failure
Direct Spanish rendered the 84-character f1583 as "¡Uy!" (echo of a few-shot
answer). `check_plausible` now rejects example echoes on lines ≥ 8 characters and
outputs with < 0.5 letters per source character on lines ≥ 16 characters; Spanish
retries once via the assisted route (f1583 then correct, 2.5 s), otherwise the
line is shown in Japanese only. Unit-tested [test]; re-run [run] changed no other
conversational output.

## 3. Full pipeline from a file, real time — [run]
`kikoe-eval pipeline`: the 48 conversational lines synthesised with three
Windows Japanese voices (1.1× and 1.35× rate) and concatenated with 0.35–1.5 s
gaps into a 125.7 s, 48 kHz stereo stream (`clean`), and the same under
synthetic game-style BGM + SFX at 10 dB SNR (`bgm`). The file source releases
10 ms packets on the wall clock, exactly like a device; everything after it is
the app's pipeline (resampler, Silero, segmenter, filters, Whisper, worker,
coordinator). Latencies use the **true** speech boundaries from the truth file.
**Synthetic speech is far cleaner than a real VTuber stream; these numbers are
an upper bound on quality.**

| run | stream CER | segs / merged / missed / outside speech | speech start → subtitle median / p95 / max | speech end → subtitle median / p95 / max | MT median | Stop |
|---|---|---|---|---|---|---|
| clean, EN | 0.032 | 49 / 3 / 0 / 0 | 3.02 / 5.80 / 9.64 s | 1.46 / 4.09 / 7.47 s | 639 ms ¹ | 48 ms |
| clean, ES | 0.032 | 49 / 3 / 0 / 0 | 2.71 / 3.76 / 4.24 s | 1.22 / 1.47 / 2.43 s | 305 ms | 76 ms |
| BGM, EN | 0.032 | 45 / 6 / 0 / 0 | 2.98 / 3.99 / 6.10 s | 1.28 / 1.63 / 3.68 s | — | 9 ms |
| BGM, ES | 0.032 | 45 / 6 / 0 / 0 | 2.92 / 3.90 / 6.08 s | 1.31 / 1.66 / 3.66 s | 287 ms | 56 ms |
| clean, EN→ES at 40 s→EN at 85 s | 0.032 | 49 / 3 / 0 / 0 | EN 2.90 / 3.80 · ES 2.45 / 3.73 s | EN 1.13 / 1.63 · ES 1.13 / 1.38 s | 269 / 250 ms | 44 ms |
| BGM, EN, **energy VAD** (Phase 1-style baseline) | **0.153** | 54 / 1 / **3** / 0 (12 blips, 9 splits) | 2.12 / 3.06 / 4.99 s | — | — | 36 ms |

¹ first run after the build; later runs ≈ 270–305 ms. The GPU is shared with the
owner's other applications (see §4), which varies run to run.

* Stream CER = edit distance of all transcripts vs all reference lines in order
  (robust to merged/split segments). Per-utterance CER in the `.md` files is
  inflated when two lines share one segment (the merged text is correct).
* Speech end → Japanese line shown: median 0.69–0.81 s, p95 0.83–1.18 s.
* ASR queue wait p95 < 0.05 ms; no drops, no translation failures, coordinator
  rejections 0 in all file runs. Silero produced **no segment outside speech**,
  including under BGM.
* **Language switching** [run]: 0 publications in the old language after each
  switch; the same model serves both languages, so a switch costs no model load
  (`ModelStatus ready, load_ms: null`) — only a cached prompt-prefix change.
* chrF of live subtitles against references (clean): EN 62.5–64.9, ES 48.9–53.3
  (lower than text-only because ASR/merging changes the input).

## 4. Real system-audio capture (WASAPI loopback) — [run]
`kikoe-eval live --play`: the clean stream was played through the default output
(Windows default render device on this PC: "Voicemeeter AUX Input (VB-Audio
Voicemeeter VAIO)") while KikoeLive captured it via WASAPI loopback, 140 s, with
a switch EN→ES at 60 s. Playback start was aligned automatically (offset 2.26 s).

* Capture: 14,094 packets, 6.77 M frames, **0 dropped**, 1 OS discontinuity,
  0 device re-opens; float32 48 kHz stereo negotiated via AUTOCONVERTPCM.
* All 49 segments transcribed (stream behaviour identical to the file run; CER
  per utterance 0.123 before merge correction); ASR median 272 ms.
* Translation ran **4–5× slower than in the file runs** (MT median 1.27 s EN /
  1.44 s ES, max 10.2 s) because the RTX 4050 was concurrently used by three Unity
  editors, Substance 3D Painter, Wallpaper Engine and the Oculus runtime (GPU
  49 % busy and P5 power state before KikoeLive started; laptop on AC). The
  translator stayed on the GPU (`[GPU]`, no fallback).
* Under that load the bounded design worked as intended: MT queue wait p95
  5–6 s, **1 line shown in Japanese only** (`mt-queue` eviction) instead of
  accumulating delay; speech end → subtitle median 2.7 s EN / 3.1 s ES, p95 10.0 /
  6.5 s, max 11.7 s. 0 stale-language publications after the switch.
* Stop with the worker loaded: 771 ms (includes worker shutdown).
