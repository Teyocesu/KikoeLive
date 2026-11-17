"""Build the FLEURS evaluation subset (dev tooling, not shipped).

FLEURS (Google, CC-BY 4.0) gives real read Japanese speech; its sentences come
from FLoRes (CC-BY-SA 4.0), so the en_us and es_419 rows with the same id are
independent professional translations of the same sentence.

Usage: python -I scripts/prepare_fleurs.py eval/data/fleurs 40
Writes:
  eval/corpus/fleurs_dev_subset.jsonl   text only (committed, with attribution)
  eval/data/fleurs/manifest.jsonl       + local wav paths (not committed)
"""
import json
import os
import sys


def read_tsv(path):
    rows = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if len(parts) < 3:
                continue
            sid, wav, raw = parts[0], parts[1], parts[2]
            rows.setdefault(sid, (wav, raw))  # first recording per sentence
    return rows


def main():
    root = sys.argv[1]
    n = int(sys.argv[2]) if len(sys.argv) > 2 else 40
    ja = read_tsv(os.path.join(root, "ja_jp_dev.tsv"))
    en = read_tsv(os.path.join(root, "en_us_dev.tsv"))
    es = read_tsv(os.path.join(root, "es_419_dev.tsv"))
    common = sorted(set(ja) & set(en) & set(es), key=int)
    step = max(1, len(common) // n)
    chosen = common[::step][:n]
    audio_dir = os.path.join(root, "ja_dev_audio")
    wavs = {}
    for dirpath, _, files in os.walk(audio_dir):
        for fn in files:
            wavs[fn] = os.path.join(dirpath, fn)
    corpus_path = os.path.join("eval", "corpus", "fleurs_dev_subset.jsonl")
    with open(corpus_path, "w", encoding="utf-8") as c, open(os.path.join(root, "manifest.jsonl"), "w", encoding="utf-8") as m:
        for sid in chosen:
            item = {"id": f"f{sid}", "ja": ja[sid][1], "en": en[sid][1], "es": es[sid][1], "tags": ["fleurs"]}
            c.write(json.dumps(item, ensure_ascii=False) + "\n")
            wav = wavs.get(ja[sid][0])
            if wav:
                item["wav"] = os.path.abspath(wav)
                m.write(json.dumps(item, ensure_ascii=False) + "\n")
    print(f"common ids: {len(common)}, chosen: {len(chosen)} -> {corpus_path}")


if __name__ == "__main__":
    main()
