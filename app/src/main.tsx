import { StrictMode, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { open } from "@tauri-apps/plugin-dialog";
import {
  ackAfterPaint, api, formatBytes, LANG_LABEL, onApp, onPipeline,
  type AppEvent, type Lang, type ModelState, type Phase, type PipelineEvent, type PipelineState,
  type PipelineStats, type Settings, type Snapshot, type SourceInfo,
} from "./api";
import "./styles.css";

interface Line { seg: number; ja: string; text?: string; lang?: Lang; note?: string }
interface Dl { state: string; downloaded: number; total: number; bps: number; error: string | null }

function statusOf(phase: Phase, ps: PipelineState | null, detail: string | null): { label: string; tone: string; hint?: string } {
  switch (phase) {
    case "idle": return { label: "Idle", tone: "idle", hint: detail ?? undefined };
    case "loading": return { label: "Loading", tone: "busy", hint: detail ?? undefined };
    case "waiting_permission": return { label: "Waiting for permission", tone: "warn", hint: detail ?? undefined };
    case "stopping": return { label: "Stopping…", tone: "busy" };
    case "error": return { label: "Error", tone: "bad", hint: detail ?? undefined };
    case "running":
      switch (ps) {
        case "processing": return { label: "Processing", tone: "busy" };
        case "falling_behind": return { label: "Falling behind", tone: "warn", hint: "Skipping old speech to catch up with the stream." };
        case "recovering": return { label: "Recovering", tone: "warn", hint: "Restarting the translator…" };
        case "failed": return { label: "Capture stopped", tone: "bad", hint: "Press Stop, then Start again." };
        default: return { label: "Listening", tone: "ok" };
      }
  }
}

const KIND_LABEL: Record<ModelState["kind"], string> = { asr: "Speech recognition", vad: "Voice detection", translator: "Translation" };

function App() {
  const [snap, setSnap] = useState<Snapshot | null>(null);
  const [phase, setPhase] = useState<Phase>("idle");
  const [detail, setDetail] = useState<string | null>(null);
  const [ps, setPs] = useState<PipelineState | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [models, setModels] = useState<ModelState[]>([]);
  const [lang, setLang] = useState<Lang>("en");
  const [sources, setSources] = useState<SourceInfo[]>([]);
  const [lines, setLines] = useState<Line[]>([]);
  const [level, setLevel] = useState(0);
  const [stats, setStats] = useState<PipelineStats | null>(null);
  const [mt, setMt] = useState<string | null>(null);
  const [dls, setDls] = useState<Record<string, Dl>>({});
  const [notice, setNotice] = useState<string | null>(null);
  const langRef = useRef<Lang>("en");

  const refresh = useCallback(async () => {
    const s = await api.snapshot();
    setSnap(s); setPhase(s.phase); setDetail(s.detail); setSettings(s.settings); setModels(s.models);
    setLang(s.lang); langRef.current = s.lang;
    api.sources().then(setSources).catch((e) => setNotice(String(e)));
  }, []);

  useEffect(() => {
    void refresh();
    const subs = [
      onApp((e: AppEvent) => {
        if (e.kind === "phase") { setPhase(e.phase); setDetail(e.detail); if (e.phase !== "running") setPs(null); }
        else if (e.kind === "models") setModels(e.models);
        else if (e.kind === "settings") setSettings(e.settings);
        else if (e.kind === "language") { setLang(e.lang); langRef.current = e.lang; }
        else if (e.kind === "download") {
          setDls((d) => ({ ...d, [e.id]: { state: e.state, downloaded: e.downloaded, total: e.total, bps: e.bytes_per_sec, error: e.error } }));
        }
      }),
      onPipeline((e: PipelineEvent) => {
        switch (e.kind) {
          case "state": setPs(e.state); break;
          case "level": setLevel(e.in_speech ? Math.max(0.35, e.speech_prob) : e.speech_prob * 0.5); break;
          case "stats": setStats(e.stats); break;
          case "model_status":
            setMt(e.status.status === "loading" ? `Loading translator for ${LANG_LABEL[e.lang]}…`
              : e.status.status === "failed" ? `Translator failed: ${e.status.message}` : null);
            break;
          case "transcript":
            setLines((l) => [{ seg: e.seg_id, ja: e.ja }, ...l].slice(0, 5));
            break;
          case "translation":
            if (e.lang !== langRef.current) break; // never show a line under the wrong language
            setLines((l) => l.map((x) => (x.seg === e.seg_id ? { ...x, text: e.text, lang: e.lang, note: undefined } : x)));
            ackAfterPaint(e.seg_id, e.lang);
            break;
          case "translation_skipped":
            setLines((l) => l.map((x) => (x.seg === e.seg_id && !x.text ? { ...x, note: "not translated (catching up)" } : x)));
            break;
          case "error":
            if (!e.recoverable) setNotice(`${e.stage}: ${e.message}`);
            break;
        }
      }),
    ];
    return () => { subs.forEach((p) => p.then((u) => u())); };
  }, [refresh]);

  const running = phase === "running";
  const busy = phase === "loading" || phase === "stopping";
  const status = statusOf(phase, ps, detail);

  const required = useMemo(() => {
    if (!settings || !snap) return [];
    const translatorId = snap.translator_for[lang];
    return models.filter((m) => m.id === settings.asr_model || m.kind === "vad" || m.id === translatorId);
  }, [models, settings, lang, snap]);
  const missing = required.filter((m) => m.status !== "installed");

  const toggle = async () => {
    setNotice(null);
    try { running ? await api.stop() : await api.start(); } catch (e) { setNotice(String(e)); }
  };
  const chooseLang = async (l: Lang) => {
    if (l === lang) return;
    setLang(l); langRef.current = l;
    setLines((ls) => ls.map((x) => (x.text && x.lang !== l ? { ...x, note: `switched to ${LANG_LABEL[l]}` } : x)));
    try { await api.setLanguage(l); } catch (e) { setNotice(String(e)); }
  };
  const patch = async (p: Parameters<typeof api.updateSettings>[0]) => {
    try { setSettings(await api.updateSettings(p)); } catch (e) { setNotice(String(e)); }
  };
  const useExisting = async (m: ModelState) => {
    const picked = await open({ multiple: false, directory: false, title: `Choose ${m.display_name}`, filters: [{ name: "Model", extensions: [m.id.startsWith("qwen") ? "gguf" : "bin"] }] });
    if (typeof picked === "string") {
      try { await api.useExisting(m.id, picked); } catch (e) { setNotice(String(e)); }
    }
  };

  if (!snap || !settings) return <div className="app"><p className="muted">Starting…</p></div>;

  const translatorFor = (l: Lang) => models.find((m) => m.id === snap.translator_for[l]);

  return (
    <div className="app">
      <header className="top">
        <div className="brand"><span className="logo" aria-hidden>聞</span><div><h1>KikoeLive</h1><p className="muted">Japanese → {LANG_LABEL[lang]}</p></div></div>
        <div className={`pill ${status.tone}`} role="status"><span className="dot" />{status.label}</div>
      </header>

      {status.hint && <p className={`hint ${status.tone}`}>{status.hint}</p>}
      {mt && <p className="hint busy">{mt}</p>}
      {notice && <div className="banner bad" role="alert"><span>{notice}</span><button className="link" onClick={() => setNotice(null)}>Dismiss</button></div>}
      {!snap.worker_found && <div className="banner bad">The translation worker (kikoe-mt) is missing next to the app. Reinstall KikoeLive.</div>}

      <section className="card">
        <label className="label">Subtitles in</label>
        <div className="seg" role="radiogroup" aria-label="Subtitle language">
          {(["en", "es"] as Lang[]).map((l) => (
            <button key={l} role="radio" aria-checked={lang === l} className={lang === l ? "on" : ""} onClick={() => void chooseLang(l)}>
              {LANG_LABEL[l]}
            </button>
          ))}
        </div>

        <label className="label" htmlFor="src">Audio source</label>
        <div className="row">
          <select id="src" disabled={running || busy} value={settings.source_id ?? ""} onChange={(e) => void patch({ source_id: e.target.value || null })}>
            <option value="">System default output{sources.find((s) => s.is_default) ? ` (${sources.find((s) => s.is_default)!.label})` : ""}</option>
            {sources.map((s) => <option key={s.id} value={s.id}>{s.label}</option>)}
          </select>
          <button className="icon" title="Refresh devices" disabled={running} onClick={() => api.sources().then(setSources).catch((e) => setNotice(String(e)))}>↻</button>
        </div>
        <div className="meter" aria-label="Speech activity"><div style={{ width: `${Math.round((running ? level : 0) * 100)}%` }} /></div>
        <p className="tiny muted">KikoeLive listens only to what your computer is playing — never the microphone.</p>

        <button className={`primary ${running ? "stop" : ""}`} disabled={busy || (!running && missing.length > 0)} onClick={() => void toggle()}>
          {running ? "■ Stop" : busy ? "…" : "▶ Start"}
        </button>
        {!running && missing.length > 0 && <p className="tiny warn-text">Install the models below to start.</p>}

        <label className="switch"><input type="checkbox" checked={settings.show_japanese} onChange={(e) => void patch({ show_japanese: e.target.checked })} /> Show original Japanese</label>
      </section>

      <section className="card">
        <h2>Live preview</h2>
        {lines.length === 0 ? <p className="muted">Subtitles will appear here and in the floating overlay.</p> : (
          <ul className="lines">
            {lines.map((l) => (
              <li key={l.seg}>
                {settings.show_japanese && <div className="ja" lang="ja">{l.ja}</div>}
                <div className={`tr ${l.note ? "dim" : ""}`}>{l.text ?? <span className="muted">{l.note ?? "translating…"}</span>}
                  {l.text && l.note && <span className="tag">{l.note}</span>}</div>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section className="card">
        <h2>Overlay</h2>
        <label className="switch"><input type="checkbox" checked={settings.overlay.visible} onChange={(e) => void patch({ overlay: { ...settings.overlay, visible: e.target.checked } })} /> Show floating subtitles</label>
        <div className="grid2">
          <label className="label">Background <input type="range" min={0} max={1} step={0.05} value={settings.overlay.opacity} onChange={(e) => void patch({ overlay: { ...settings.overlay, opacity: Number(e.target.value) } })} /></label>
          <label className="label">Text size <input type="range" min={16} max={60} step={1} value={settings.overlay.font_size} onChange={(e) => void patch({ overlay: { ...settings.overlay, font_size: Number(e.target.value) } })} /></label>
        </div>
        <button className="secondary" onClick={() => void api.resetOverlay().catch((e) => setNotice(String(e)))}>Reset overlay position</button>
        <p className="tiny muted">Drag the overlay anywhere; drag its corner to resize.</p>
      </section>

      <section className="card">
        <h2>Language packs & models</h2>
        <ul className="packs">
          {(["en", "es"] as Lang[]).map((l) => {
            const t = translatorFor(l);
            return <li key={l}><strong>{LANG_LABEL[l]}</strong> <span className={t?.status === "installed" ? "ok-text" : "warn-text"}>{t ? (t.status === "installed" ? `ready · ${t.display_name}` : `needs ${t.display_name}`) : "unavailable"}</span></li>;
          })}
        </ul>
        <ul className="models">
          {models.filter((m) => m.kind !== "translator" || required.some((r) => r.id === m.id) || m.status === "installed").map((m) => {
            const dl = dls[m.id];
            const active = dl && (dl.state === "running" || dl.state === "verifying");
            const needed = required.some((r) => r.id === m.id);
            return (
              <li key={m.id} className={needed ? "needed" : ""}>
                <div className="mrow">
                  <div>
                    <div className="mname">{m.display_name}{needed && <span className="tag">required</span>}</div>
                    <div className="tiny muted">{KIND_LABEL[m.kind]} · {formatBytes(m.size_bytes)} · {m.license}</div>
                  </div>
                  <div className="mstatus">
                    {m.status === "installed" && !active && <span className="ok-text">✓ Ready{m.external ? " (existing file)" : ""}</span>}
                    {m.status === "needs_verification" && !active && <span className="muted">checking…</span>}
                    {(m.status === "missing" || m.status === "invalid") && !active && (
                      <>
                        <button className="secondary small" onClick={() => void api.download(m.id).catch((e) => setNotice(String(e)))}>Download {formatBytes(m.size_bytes)}</button>
                        <button className="link" onClick={() => void useExisting(m)}>Use existing file…</button>
                      </>
                    )}
                    {active && <button className="link" onClick={() => void api.cancelDownload(m.id)}>Cancel</button>}
                  </div>
                </div>
                {active && <div className="bar"><div style={{ width: `${(100 * dl.downloaded) / Math.max(1, dl.total)}%` }} /><span className="tiny">{dl.state === "verifying" ? "Verifying" : `${formatBytes(dl.downloaded)} / ${formatBytes(dl.total)} · ${formatBytes(dl.bps)}/s`}</span></div>}
                {dl?.state === "failed" && <div className="tiny bad-text">{dl.error}</div>}
                {m.status === "invalid" && <div className="tiny bad-text">{m.detail}</div>}
              </li>
            );
          })}
        </ul>
        <p className="tiny muted">Models are stored in {snap.models_dir} and verified with SHA-256. Nothing is downloaded unless you press Download.</p>
      </section>

      <details className="card diag">
        <summary>Diagnostics</summary>
        <p className="tiny">Engine: Whisper on {settings.use_gpu ? snap.gpu_backend.toUpperCase() : "CPU"} · app v{snap.version}</p>
        <label className="switch tiny"><input type="checkbox" disabled={running} checked={settings.use_gpu} onChange={(e) => void patch({ use_gpu: e.target.checked })} /> Use GPU acceleration</label>
        {stats ? (
          <table className="tiny"><tbody>
            {(["en", "es"] as Lang[]).map((l) => (
              <tr key={l}><td>{LANG_LABEL[l]}</td><td>speech end → subtitle {fmt(stats.latency[l].speech_end_to_subtitle_ms.median)} (p95 {fmt(stats.latency[l].speech_end_to_subtitle_ms.p95)})</td></tr>
            ))}
            <tr><td>ASR</td><td>median {fmt(stats.latency.asr_ms.median)} · queue {stats.asr_queue_len}</td></tr>
            <tr><td>Dropped</td><td>{stats.dropped_asr_queue + stats.dropped_asr_stale} segments · {stats.dropped_mt_queue + stats.skipped_mt_stale} untranslated · filtered {stats.filtered}</td></tr>
            <tr><td>Translator</td><td>failures {stats.translation_failures} · restarts {stats.translator_restarts}</td></tr>
            <tr><td>Capture</td><td>{stats.capture.packets} packets · dropped {stats.capture.dropped_packets} · reopened {stats.capture.reopen_count}</td></tr>
          </tbody></table>
        ) : <p className="tiny muted">Start KikoeLive to see live measurements.</p>}
      </details>

      <footer className="tiny muted">Everything runs on this computer. No audio or text leaves it, and nothing is saved.</footer>
    </div>
  );
}

function fmt(ms: number) {
  return ms ? `${(ms / 1000).toFixed(2)} s` : "–";
}

createRoot(document.getElementById("root")!).render(<StrictMode><App /></StrictMode>);
