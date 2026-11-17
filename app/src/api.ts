// Typed bridge to the Rust backend. Every call here maps 1:1 to a command in
// app/src-tauri/src/lib.rs; the backend validates all arguments again.
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type Lang = "en" | "es";
export type Phase = "idle" | "loading" | "waiting_permission" | "running" | "stopping" | "error";
export type PipelineState = "starting" | "listening" | "processing" | "falling_behind" | "recovering" | "stopped" | "failed";

export interface OverlaySettings { visible: boolean; opacity: number; font_size: number }
export interface Settings {
  target_language: Lang;
  show_japanese: boolean;
  source_id: string | null;
  asr_model: string;
  use_gpu: boolean;
  overlay: OverlaySettings;
}
export interface ModelState {
  id: string;
  display_name: string;
  kind: "asr" | "vad" | "translator";
  size_bytes: number;
  license: string;
  source_repo: string;
  revision: string;
  serves: Lang[];
  status: "installed" | "missing" | "needs_verification" | "invalid";
  path: string | null;
  external: boolean;
  detail: string | null;
}
export interface Snapshot {
  version: string;
  phase: Phase;
  detail: string | null;
  settings: Settings;
  models: ModelState[];
  lang: Lang;
  epoch: number;
  gpu_backend: string;
  worker_found: boolean;
  models_dir: string;
  translator_for: Record<Lang, string>;
}
export interface SourceInfo { id: string; label: string; kind: string; is_default: boolean }

export interface Summary { n: number; median: number; p95: number; max: number; mean: number }
export interface LangLatency {
  speech_start_to_subtitle_ms: Summary;
  speech_end_to_subtitle_ms: Summary;
  mt_ms: Summary;
  mt_queue_ms: Summary;
  ui_ms: Summary;
}
export interface PipelineStats {
  asr_queue_len: number;
  mt_queue_len: number;
  dropped_asr_queue: number;
  dropped_asr_stale: number;
  dropped_mt_queue: number;
  skipped_mt_stale: number;
  filtered: number;
  translation_failures: number;
  translator_restarts: number;
  capture: { packets: number; dropped_packets: number; reopen_count: number };
  latency: { asr_ms: Summary; asr_queue_ms: Summary; speech_end_to_transcript_ms: Summary; en: LangLatency; es: LangLatency };
}

export type PipelineEvent =
  | { kind: "state"; state: PipelineState; detail: string | null }
  | { kind: "level"; speech_prob: number; rms: number; in_speech: boolean }
  | { kind: "transcript"; seg_id: number; ja: string; start_ms: number; end_ms: number }
  | { kind: "translation"; seg_id: number; lang: Lang; epoch: number; ja: string; text: string }
  | { kind: "translation_skipped"; seg_id: number; lang: Lang; reason: string }
  | { kind: "dropped"; seg_id: number; stage: string; reason: string }
  | { kind: "model_status"; lang: Lang; model: string; status: { status: "loading" } | { status: "ready"; load_ms: number | null } | { status: "failed"; message: string } }
  | { kind: "stats"; stats: PipelineStats }
  | { kind: "error"; stage: string; message: string; recoverable: boolean };

export type AppEvent =
  | { kind: "phase"; phase: Phase; detail: string | null }
  | { kind: "download"; id: string; state: "running" | "verifying" | "done" | "failed" | "cancelled"; downloaded: number; total: number; bytes_per_sec: number; error: string | null }
  | { kind: "models"; models: ModelState[] }
  | { kind: "settings"; settings: Settings }
  | { kind: "language"; lang: Lang; epoch: number };

export const api = {
  snapshot: () => invoke<Snapshot>("get_snapshot"),
  sources: () => invoke<SourceInfo[]>("list_sources"),
  setLanguage: (lang: Lang) => invoke<number>("set_language", { lang }),
  updateSettings: (patch: Partial<{ show_japanese: boolean; source_id: string | null; use_gpu: boolean; asr_model: string; overlay: OverlaySettings }>) =>
    invoke<Settings>("update_settings", { patch }),
  resetOverlay: () => invoke<void>("reset_overlay_position"),
  rendered: (segId: number, lang: Lang) => invoke<void>("subtitle_rendered", { segId, lang }),
  download: (id: string) => invoke<void>("download_model", { id }),
  cancelDownload: (id: string) => invoke<void>("cancel_download", { id }),
  useExisting: (id: string, path: string) => invoke<void>("use_existing_model", { id, path }),
  start: () => invoke<void>("start"),
  stop: () => invoke<void>("stop"),
};

export const onPipeline = (f: (e: PipelineEvent) => void): Promise<UnlistenFn> =>
  listen<PipelineEvent>("kikoe://pipeline", (e) => f(e.payload));
export const onApp = (f: (e: AppEvent) => void): Promise<UnlistenFn> => listen<AppEvent>("kikoe://app", (e) => f(e.payload));

export const LANG_LABEL: Record<Lang, string> = { en: "English", es: "Español" };

export function formatBytes(n: number): string {
  if (n >= 1e9) return `${(n / 1e9).toFixed(1)} GB`;
  if (n >= 1e6) return `${Math.round(n / 1e6)} MB`;
  return `${Math.max(1, Math.round(n / 1e3))} KB`;
}

/** Signal to the backend once a subtitle has actually been painted. */
export function ackAfterPaint(segId: number, lang: Lang) {
  requestAnimationFrame(() => requestAnimationFrame(() => void api.rendered(segId, lang).catch(() => {})));
}
