import { StrictMode, useEffect, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ackAfterPaint, api, LANG_LABEL, onApp, onPipeline, type Lang, type Settings } from "./api";
import "./overlay.css";

// Two-line subtitle: the newest Japanese line, and the newest translation in the
// selected language. A translation is only replaced by a newer one (no blank
// flashes); after a language switch the old line is dimmed until the first line
// in the new language arrives. Everything fades after a pause in speech.
const FADE_AFTER_MS = 7000;

function Overlay() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [lang, setLang] = useState<Lang>("en");
  const [ja, setJa] = useState<{ seg: number; text: string } | null>(null);
  const [tr, setTr] = useState<{ seg: number; text: string; lang: Lang } | null>(null);
  const [faded, setFaded] = useState(false);
  const langRef = useRef<Lang>("en");
  const shownSeg = useRef(0);
  const fadeTimer = useRef<number | undefined>(undefined);

  const touch = () => {
    setFaded(false);
    window.clearTimeout(fadeTimer.current);
    fadeTimer.current = window.setTimeout(() => setFaded(true), FADE_AFTER_MS);
  };

  useEffect(() => {
    api.snapshot().then((s) => { setSettings(s.settings); setLang(s.lang); langRef.current = s.lang; });
    const subs = [
      onApp((e) => {
        if (e.kind === "settings") setSettings(e.settings);
        if (e.kind === "language") { setLang(e.lang); langRef.current = e.lang; }
      }),
      onPipeline((e) => {
        if (e.kind === "transcript") { setJa({ seg: e.seg_id, text: e.ja }); touch(); }
        if (e.kind === "translation") {
          if (e.lang !== langRef.current || e.seg_id < shownSeg.current) return;
          shownSeg.current = e.seg_id;
          setTr({ seg: e.seg_id, text: e.text, lang: e.lang });
          touch();
          ackAfterPaint(e.seg_id, e.lang);
        }
        if (e.kind === "state" && e.state === "stopped") { setJa(null); setTr(null); shownSeg.current = 0; }
      }),
    ];
    return () => { subs.forEach((p) => p.then((u) => u())); };
  }, []);

  if (!settings) return null;
  const o = settings.overlay;
  const stale = tr !== null && tr.lang !== lang;
  const win = getCurrentWindow();

  return (
    <div
      className={`panel ${faded ? "faded" : ""}`}
      style={{ background: `rgba(12, 14, 24, ${o.opacity})`, fontSize: `${o.font_size}px` }}
      onMouseDown={(e) => { if (e.button === 0 && (e.target as HTMLElement).dataset.nodrag === undefined) void win.startDragging(); }}
    >
      {settings.show_japanese && <div className="ja" lang="ja">{ja?.text ?? ""}</div>}
      <div className={`tr ${stale ? "stale" : ""}`} lang={tr?.lang ?? lang}>
        {tr?.text ?? (ja ? "" : <span className="idle">KikoeLive · {LANG_LABEL[lang]}</span>)}
        {stale && <span className="switching" data-nodrag>→ {LANG_LABEL[lang]}</span>}
      </div>
      <button className="hide" data-nodrag title="Hide overlay (show it again from the main window)"
        onClick={() => void api.updateSettings({ overlay: { ...o, visible: false } })}>×</button>
      <div className="grip" data-nodrag title="Resize"
        onMouseDown={(e) => { e.stopPropagation(); void win.startResizeDragging("SouthEast"); }} />
    </div>
  );
}

createRoot(document.getElementById("root")!).render(<StrictMode><Overlay /></StrictMode>);
