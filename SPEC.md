# KikoeLive — product specification

Status: approved scope for release 0.x (revised 2026-11-17 to make English and
Spanish both mandatory release targets).

## Product
A free, local-first desktop application that shows English **or** Spanish
subtitles for Japanese audio playing on the user's computer (primary use case:
Japanese VTuber livestreams on YouTube/Twitch), in a floating overlay.

## Users can
1. Pick a system-audio source (default output, or a specific output device).
2. Choose the subtitle language — **English** or **Español** — and change it while
   running, without restarting capture or speech recognition.
3. Press Start / Stop. Start never requires running scripts or editing files.
4. Optionally show the original Japanese line above the translation.
5. Move, resize, hide/restore the overlay and adjust its opacity and text size
   without interrupting playback.
6. See clearly whether KikoeLive is Idle, Loading, Waiting for permission,
   Listening, Processing, Falling behind, Recovering, Stopped or in Error.
7. Install missing models deliberately (size shown first), cancel/resume
   downloads, or point KikoeLive at a verified copy already on disk.

## Languages
* Source: Japanese only.
* Targets: `ja-en` and `ja-es`, both first-class. One target is shown at a time.
* Translation is direct Japanese → target (no pivot through another language).
* Each target is evaluated independently; a pass in one never implies the other.

## Invariants
* **Meaning first.** Negation, subject, action, names must be preserved; nothing
  may be invented; only the current line is translated; no notes, reasoning or
  annotations in subtitles.
* **No stale or mislabelled output.** After a language switch, no translation
  produced for the previous language is published. Subtitles appear in order and
  never twice.
* **Bounded.** Queues, buffers and statistics are bounded; when processing falls
  behind, old speech is dropped (and reported) so subtitles return to live.
* **Responsive.** Stop terminates capture and pending work promptly; the UI never
  blocks on inference.
* **Local and private.** No cloud inference, accounts, telemetry, network
  listeners, microphone capture, or persistence of audio/transcripts. Network use
  only for explicit model downloads from pinned, hash-verified sources.
* **Honest status.** The UI never shows success that has not happened.
* **Cross-platform parity.** One application for macOS (Apple Silicon) and Windows
  x64 with the same features and languages; platform code is isolated adapters.

## Platforms
* macOS 13+ on Apple Silicon (development reference: MacBook Air M1, 8 GiB) —
  ScreenCaptureKit audio, Metal.
* Windows 11 x64 (reference: Ryzen 7 7735HS, 32 GiB, RTX 4050 Laptop) — WASAPI
  loopback, CUDA, CPU fallback.

## Out of scope for this release
Simultaneous dual-language output, other source languages, re-translating past
subtitles after a switch, signed production installers, cloud features.
