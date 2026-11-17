"""Concatenate synthesised lines into one 48 kHz stereo "stream" WAV with known
speech boundaries (truth file), optionally under synthetic game-style BGM/SFX.
Dev tooling only; everything it produces is synthetic and stays out of Git.

Usage:
  python -I scripts/build_tts_mix.py eval/corpus/conversational.jsonl eval/data/tts/lines eval/data/tts/clean none
  python -I scripts/build_tts_mix.py eval/corpus/conversational.jsonl eval/data/tts/lines eval/data/tts/bgm 12
(last argument: BGM signal-to-noise ratio in dB, or "none")
"""
import array
import json
import math
import random
import sys
import wave

OUT_RATE = 48_000


def read_wav(path):
    with wave.open(path, "rb") as w:
        ch, width, rate, n = w.getnchannels(), w.getsampwidth(), w.getframerate(), w.getnframes()
        raw = w.readframes(n)
    if width != 2:
        raise SystemExit(f"{path}: expected 16-bit PCM, got {width * 8}-bit")
    a = array.array("h", raw)
    mono = [sum(a[i:i + ch]) / ch / 32768.0 for i in range(0, len(a), ch)]
    return mono, rate


def resample_linear(x, src, dst):
    if src == dst:
        return list(x)
    n = int(len(x) * dst / src)
    out = []
    for i in range(n):
        t = i * src / dst
        k = int(t)
        f = t - k
        a = x[k] if k < len(x) else 0.0
        b = x[k + 1] if k + 1 < len(x) else 0.0
        out.append(a + (b - a) * f)
    return out


def speech_bounds(x, rate, thresh=0.01):
    win = int(rate * 0.01)
    idx = [i for i in range(0, len(x) - win, win) if max(abs(v) for v in x[i:i + win]) > thresh]
    if not idx:
        return 0, len(x)
    return idx[0], min(len(x), idx[-1] + win)


def bgm(n, rng):
    """Simple chord progression + kick + occasional noise-burst SFX."""
    out = [0.0] * n
    chords = [(220.0, 277.2, 329.6), (196.0, 246.9, 293.7), (174.6, 220.0, 261.6), (196.0, 246.9, 311.1)]
    beat = int(OUT_RATE * 0.5)
    for i in range(n):
        c = chords[(i // (beat * 4)) % len(chords)]
        t = i / OUT_RATE
        v = sum(math.sin(2 * math.pi * f * t) for f in c) / 3 * 0.5
        ph = (i % beat) / OUT_RATE
        v += math.sin(2 * math.pi * 55 * ph) * math.exp(-ph * 18) * 0.8  # kick
        out[i] = v
    for _ in range(n // (OUT_RATE * 3)):
        start = rng.randrange(0, max(1, n - OUT_RATE // 4))
        for k in range(OUT_RATE // 6):
            out[start + k] += rng.uniform(-1, 1) * math.exp(-k / (OUT_RATE * 0.04)) * 0.9
    return out


def rms(x):
    return math.sqrt(sum(v * v for v in x) / max(1, len(x)))


def main():
    corpus, lines_dir, out_prefix, snr = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    rng = random.Random(7)
    items = [json.loads(l) for l in open(corpus, encoding="utf-8") if l.strip()]
    track = [0.0] * int(OUT_RATE * 1.0)
    truth = []
    speech_samples = []
    for it in items:
        x, rate = read_wav(f"{lines_dir}/{it['id']}.wav")
        x = resample_linear(x, rate, OUT_RATE)
        s, e = speech_bounds(x, OUT_RATE)
        x = x[s:e]
        # second line of a context pair follows quickly, like real speech
        gap = 0.35 if it["id"].endswith("b") else rng.uniform(0.7, 1.5)
        start = len(track)
        track.extend(v * 0.6 for v in x)
        speech_samples.extend(v * 0.6 for v in x)
        truth.append({**it, "start_s": round(start / OUT_RATE, 3), "end_s": round(len(track) / OUT_RATE, 3)})
        track.extend([0.0] * int(OUT_RATE * gap))
    track.extend([0.0] * OUT_RATE)
    if snr != "none":
        music = bgm(len(track), rng)
        target = rms(speech_samples) / (10 ** (float(snr) / 20))
        scale = target / max(1e-9, rms(music))
        track = [a + b * scale for a, b in zip(track, music)]
    peak = max(abs(v) for v in track)
    if peak > 0.98:
        track = [v * 0.98 / peak for v in track]
    pcm = array.array("h", [int(max(-1, min(1, v)) * 32767) for v in track for _ in (0, 1)])
    with wave.open(out_prefix + ".wav", "wb") as w:
        w.setnchannels(2)
        w.setsampwidth(2)
        w.setframerate(OUT_RATE)
        w.writeframes(pcm.tobytes())
    with open(out_prefix + ".truth.jsonl", "w", encoding="utf-8") as f:
        for t in truth:
            f.write(json.dumps(t, ensure_ascii=False) + "\n")
    print(f"{out_prefix}.wav: {len(track) / OUT_RATE:.1f} s, {len(truth)} utterances, bgm snr={snr}")


if __name__ == "__main__":
    main()
