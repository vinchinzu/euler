#!/usr/bin/env python3
"""Profile all Rust solutions >1s.

Pass 1: perf stat on release binaries (fast — hardware counters for all).
Pass 2: perf record on profiling binaries (slow — function hotspots for top 50).
"""

import csv, json, os, re, subprocess, sys, time
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RUST = ROOT / "rust"
PROFILES = RUST / "profiles"
VALIDATED = ROOT / "validated.json"
THRESHOLD_MS = 1000
PERF_FREQ = 997
TOP_N_RECORD = 50  # only do perf record for the N slowest

PERF_EVENTS = (
    "cycles,instructions,cache-references,cache-misses,"
    "branches,branch-misses,L1-dcache-load-misses,LLC-load-misses"
)


def load_problems():
    problems = []
    with open(VALIDATED) as f:
        for line in f:
            e = json.loads(line.strip())
            if e.get("status") == "OK" and e.get("time_ms", 0) > THRESHOLD_MS:
                problems.append((e["problem"], e["time_ms"]))
    problems.sort(key=lambda x: -x[1])
    return problems


def run_perf_stat(binary, timeout_s):
    try:
        r = subprocess.run(
            ["perf", "stat", "-e", PERF_EVENTS, "-x", ";", str(binary)],
            capture_output=True, text=True, timeout=timeout_s,
        )
    except subprocess.TimeoutExpired:
        return None
    counters = {}
    for line in r.stderr.splitlines():
        parts = line.strip().split(";")
        if len(parts) >= 3:
            val_str = parts[0].strip().replace(",", "")
            # Strip :u/:k suffix from counter name (e.g. "cycles:u" -> "cycles")
            name = re.sub(r":[uk]+$", "", parts[2].strip())
            try:
                counters[name] = int(val_str)
            except ValueError:
                pass
    return counters


def run_perf_record(binary, timeout_s):
    """Run perf record + report, return list of (pct, func_name)."""
    perf_data = f"/tmp/perf_euler_{os.getpid()}.data"
    try:
        subprocess.run(
            ["perf", "record", "-g", "--call-graph", "dwarf",
             "-F", str(PERF_FREQ), "-o", perf_data, "--", str(binary)],
            capture_output=True, timeout=timeout_s,
        )
        r = subprocess.run(
            ["perf", "report", "-i", perf_data,
             "--stdio", "--no-children", "-n", "--percent-limit", "1.0"],
            capture_output=True, text=True, timeout=60,
        )
    except subprocess.TimeoutExpired:
        return []
    finally:
        if os.path.exists(perf_data):
            os.unlink(perf_data)

    funcs = []
    for line in r.stdout.splitlines():
        # Match lines like: "    95.11%           934  p648     p648              [.] p648::main"
        m = re.match(r"\s*(\d+\.\d+)%\s+(\d+)\s+\S+\s+\S+\s+\[.\]\s+(.*)", line)
        if m:
            pct = float(m.group(1))
            func = re.sub(r"::h[0-9a-f]{16}$", "", m.group(3).strip())
            funcs.append((pct, func))
    return funcs


def classify(c):
    if not c:
        return "UNKNOWN", {}
    cyc = max(c.get("cycles", 1), 1)
    ins = c.get("instructions", 0)
    ipc = ins / cyc
    cr = max(c.get("cache-references", 1), 1)
    cm = c.get("cache-misses", 0)
    cmp = cm / cr * 100
    br = max(c.get("branches", 1), 1)
    bm = c.get("branch-misses", 0)
    bmp = bm / br * 100
    llc = c.get("LLC-load-misses", 0)
    l1m = c.get("L1-dcache-load-misses", 0)

    tags = []
    if ipc < 0.8:
        tags.append("MEMORY-BOUND" if llc > 500_000 else "STALLED")
    elif ipc < 1.5:
        tags.append("MODERATE-IPC")
    elif ipc >= 2.5:
        tags.append("COMPUTE-EFFICIENT")
    else:
        tags.append("COMPUTE-BOUND")
    if cmp > 30:
        tags.append("HIGH-CACHE-MISS")
    if bmp > 3:
        tags.append("BRANCH-HEAVY")

    metrics = dict(ipc=ipc, cache_miss_pct=cmp, branch_miss_pct=bmp,
                   llc_misses=llc, l1_misses=l1m)
    return " | ".join(tags), metrics


def main():
    problems = load_problems()
    print(f"=== Pass 1: perf stat on {len(problems)} problems ===")
    sys.stdout.flush()

    PROFILES.mkdir(exist_ok=True)
    rows = []
    total = len(problems)

    # --- Pass 1: perf stat (release binaries, fast) ---
    release_bin = RUST / "target" / "release"
    t_start = time.time()
    for i, (prob, time_ms) in enumerate(problems):
        binary = release_bin / f"p{prob}"
        if not binary.exists():
            print(f"[{i+1}/{total}] p{prob}: SKIP (no binary)")
            sys.stdout.flush()
            continue

        timeout_s = max(30, int(time_ms / 1000 * 3 + 10))
        counters = run_perf_stat(binary, timeout_s)
        tag, metrics = classify(counters)
        ipc = metrics.get("ipc", 0)
        cmp = metrics.get("cache_miss_pct", 0)
        bmp = metrics.get("branch_miss_pct", 0)

        print(f"[{i+1}/{total}] p{prob} {time_ms/1000:>6.2f}s  "
              f"IPC={ipc:.2f}  cache={cmp:.1f}%  branch={bmp:.1f}%  -> {tag}")
        sys.stdout.flush()

        rows.append(dict(prob=prob, time_ms=time_ms, funcs=[],
                         ipc=ipc, cache_miss_pct=cmp, branch_miss_pct=bmp,
                         llc_misses=metrics.get("llc_misses", 0),
                         l1_misses=metrics.get("l1_misses", 0),
                         tag=tag, counters=counters))

    pass1_time = time.time() - t_start
    print(f"\nPass 1 done in {pass1_time:.0f}s")
    sys.stdout.flush()

    # --- Pass 2: perf record (profiling binaries, top N only) ---
    prof_bin = RUST / "target" / "profiling"
    if prof_bin.exists():
        top_n = rows[:TOP_N_RECORD]
        print(f"\n=== Pass 2: perf record on top {len(top_n)} slowest (with debug symbols) ===")
        sys.stdout.flush()

        for i, row in enumerate(top_n):
            prob = row["prob"]
            binary = prof_bin / f"p{prob}"
            if not binary.exists():
                print(f"  [{i+1}/{len(top_n)}] p{prob}: SKIP (no profiling binary)")
                sys.stdout.flush()
                continue

            timeout_s = max(30, int(row["time_ms"] / 1000 * 3 + 10))
            print(f"  [{i+1}/{len(top_n)}] p{prob} ({row['time_ms']/1000:.1f}s) ...", end=" ", flush=True)
            funcs = run_perf_record(binary, timeout_s)
            row["funcs"] = funcs
            top_f = funcs[0] if funcs else (0, "?")
            print(f"{top_f[0]:.0f}% {top_f[1][:50]}")
            sys.stdout.flush()

            # Save per-problem profile
            with open(PROFILES / f"p{prob}.prof", "w") as f:
                f.write(f"=== Problem {prob} ({row['time_ms']/1000:.2f}s) ===\n\n")
                f.write(f"IPC: {row['ipc']:.2f}  Cache miss: {row['cache_miss_pct']:.1f}%  "
                        f"Branch miss: {row['branch_miss_pct']:.1f}%\n")
                f.write(f"Classification: {row['tag']}\n\n")
                f.write("--- Top Functions ---\n")
                for pct, func in funcs[:15]:
                    f.write(f"  {pct:6.2f}%  {func}\n")
                f.write("\n")
    else:
        print(f"\nSkipping Pass 2 (no profiling binaries at {prof_bin})")

    # --- Write outputs ---
    # CSV
    with open(PROFILES / "perf_stats.csv", "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(["problem", "time_ms", "ipc", "cache_miss_pct",
                     "branch_miss_pct", "llc_misses", "l1_misses",
                     "classification", "top_func", "top_func_pct"])
        for r in rows:
            tf = r["funcs"][0] if r["funcs"] else (0, "")
            w.writerow([r["prob"], r["time_ms"], f"{r['ipc']:.2f}",
                        f"{r['cache_miss_pct']:.1f}", f"{r['branch_miss_pct']:.1f}",
                        r["llc_misses"], r["l1_misses"], r["tag"],
                        tf[1], f"{tf[0]:.1f}"])

    # Markdown summary
    with open(PROFILES / "SUMMARY.md", "w") as f:
        f.write("# Profiling Summary — Rust Solutions >1s\n\n")
        f.write(f"Generated: {time.strftime('%Y-%m-%d %H:%M')}  |  "
                f"Problems: {len(rows)}  |  Profiling time: {time.time()-t_start:.0f}s\n\n")

        f.write("## Classification Counts\n\n")
        tc = Counter()
        for r in rows:
            for t in r["tag"].split(" | "):
                tc[t] += 1
        for t, cnt in tc.most_common():
            f.write(f"- **{t}**: {cnt}\n")

        # Memory-bound section
        mem = [r for r in rows if "MEMORY" in r["tag"]]
        if mem:
            f.write(f"\n## Memory-Bound ({len(mem)} problems)\n")
            f.write("Low IPC + high LLC misses. Opportunities: data layout, cache blocking, smaller types.\n\n")
            f.write("| Prob | Time | IPC | Cache% | LLC miss | Top Function |\n")
            f.write("|------|------|-----|--------|----------|-------------|\n")
            for r in sorted(mem, key=lambda x: -x["time_ms"]):
                tf = r["funcs"][0][1][:40] if r["funcs"] else ""
                f.write(f"| p{r['prob']} | {r['time_ms']/1000:.1f}s | {r['ipc']:.2f} | "
                        f"{r['cache_miss_pct']:.0f}% | {r['llc_misses']:,} | {tf} |\n")

        # Stalled
        stl = [r for r in rows if "STALLED" in r["tag"]]
        if stl:
            f.write(f"\n## Stalled / Low IPC ({len(stl)} problems)\n")
            f.write("Low IPC, not memory-bound. Opportunities: reduce serialized deps, avoid div/sqrt chains.\n\n")
            f.write("| Prob | Time | IPC | Cache% | Branch% | Top Function |\n")
            f.write("|------|------|-----|--------|---------|-------------|\n")
            for r in sorted(stl, key=lambda x: -x["time_ms"]):
                tf = r["funcs"][0][1][:40] if r["funcs"] else ""
                f.write(f"| p{r['prob']} | {r['time_ms']/1000:.1f}s | {r['ipc']:.2f} | "
                        f"{r['cache_miss_pct']:.0f}% | {r['branch_miss_pct']:.1f}% | {tf} |\n")

        # Branch-heavy
        brh = [r for r in rows if "BRANCH" in r["tag"]]
        if brh:
            f.write(f"\n## Branch-Heavy ({len(brh)} problems)\n")
            f.write(">3% misprediction. Opportunities: branchless ops, lookup tables, sorted traversal.\n\n")
            f.write("| Prob | Time | IPC | Branch% | Top Function |\n")
            f.write("|------|------|-----|---------|-------------|\n")
            for r in sorted(brh, key=lambda x: -x["time_ms"]):
                tf = r["funcs"][0][1][:40] if r["funcs"] else ""
                f.write(f"| p{r['prob']} | {r['time_ms']/1000:.1f}s | {r['ipc']:.2f} | "
                        f"{r['branch_miss_pct']:.1f}% | {tf} |\n")

        # High cache miss
        hcm = [r for r in rows if "HIGH-CACHE" in r["tag"]]
        if hcm:
            f.write(f"\n## High Cache Miss Rate ({len(hcm)} problems)\n")
            f.write(">30% of cache refs miss. Opportunities: flatten arrays, blocking, prefetch.\n\n")
            f.write("| Prob | Time | IPC | Cache% | LLC miss | Top Function |\n")
            f.write("|------|------|-----|--------|----------|-------------|\n")
            for r in sorted(hcm, key=lambda x: -x["time_ms"]):
                tf = r["funcs"][0][1][:40] if r["funcs"] else ""
                f.write(f"| p{r['prob']} | {r['time_ms']/1000:.1f}s | {r['ipc']:.2f} | "
                        f"{r['cache_miss_pct']:.0f}% | {r['llc_misses']:,} | {tf} |\n")

        # Concentrated hotspots (function-level data from pass 2)
        conc = [(r, r["funcs"][0]) for r in rows if r["funcs"] and r["funcs"][0][0] > 70]
        if conc:
            f.write(f"\n## Concentrated Hotspots ({len(conc)} problems)\n")
            f.write("Single function >70% — easiest to micro-optimize.\n\n")
            f.write("| Prob | Time | Top% | Function |\n")
            f.write("|------|------|------|----------|\n")
            for r, (pct, func) in sorted(conc, key=lambda x: -x[0]["time_ms"]):
                f.write(f"| p{r['prob']} | {r['time_ms']/1000:.1f}s | {pct:.0f}% | `{func[:60]}` |\n")

        # Full table
        f.write(f"\n## All Problems\n\n")
        f.write("| Prob | Time | IPC | Cache% | Branch% | LLC miss | Classification |\n")
        f.write("|------|------|-----|--------|---------|----------|----------------|\n")
        for r in rows:
            f.write(f"| p{r['prob']} | {r['time_ms']/1000:.1f}s | {r['ipc']:.2f} | "
                    f"{r['cache_miss_pct']:.0f}% | {r['branch_miss_pct']:.1f}% | "
                    f"{r['llc_misses']:,} | {r['tag']} |\n")

    print(f"\nDone. Results in {PROFILES}/")
    print(f"  SUMMARY.md  — classified optimization targets")
    print(f"  perf_stats.csv — raw data for all {len(rows)} problems")
    print(f"  p*.prof — function-level profiles for top {TOP_N_RECORD}")


if __name__ == "__main__":
    main()
