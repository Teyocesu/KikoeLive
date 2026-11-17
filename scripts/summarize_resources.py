"""Summarise a sample-resources.ps1 CSV: per process median/max CPU and memory,
GPU utilisation and memory. Usage: python -I scripts/summarize_resources.py file.csv"""
import csv
import statistics
import sys
from collections import defaultdict


def num(v):
    try:
        return float(v)
    except (TypeError, ValueError):
        return None


rows = list(csv.DictReader(open(sys.argv[1], encoding="utf-8-sig")))
by = defaultdict(list)
for r in rows:
    by[r["process"]].append(r)
print(f"samples: {len(rows)}")
for proc, rs in by.items():
    cpu = [x for x in (num(r["cpu_pct"]) for r in rs) if x is not None]
    ws = [x for x in (num(r["working_set_mb"]) for r in rs) if x is not None]
    pv = [x for x in (num(r["private_mb"]) for r in rs) if x is not None]
    print(f"{proc}: cpu% median {statistics.median(cpu) if cpu else 'n/a'} max {max(cpu) if cpu else 'n/a'} | "
          f"working set median {statistics.median(ws):.0f} max {max(ws):.0f} MB | private max {max(pv):.0f} MB")
gu = [x for x in (num(r["gpu_util_pct"]) for r in rows) if x is not None]
gm = [x for x in (num(r["gpu_mem_used_mb"]) for r in rows) if x is not None]
cm = [x for x in (num(r["sys_commit_mb"]) for r in rows) if x is not None]
if gu:
    print(f"GPU util% median {statistics.median(gu)} max {max(gu)} | GPU mem used max {max(gm):.0f} MB (whole GPU)")
if cm:
    print(f"system commit charge max {max(cm):.0f} MB")
