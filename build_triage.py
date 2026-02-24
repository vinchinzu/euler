#!/usr/bin/env python3
"""Build optimization_triage.csv and optimization_triage.json from validated.json + code analysis."""

import json, csv, sys

# Load all validated entries
entries = {}
with open("validated.json") as f:
    for line in f:
        e = json.loads(line.strip())
        entries[e["problem"]] = e

# Hand-coded analysis for the top 80 slowest problems (from code inspection)
# Format: problem_id -> (speedup_range, time_saved_pct, confidence, bottleneck_class, notes)
# time_saved_pct is estimated % of current runtime that could be saved
analysis = {
    # Batch 1: 10-19s
    782: ("2-4x", 60, "med", "algorithmic", "Sequential 100M complement-symmetry pass; atomic bitset overhead vs C byte array; parallelize symmetry pass"),
    846: ("1.5-2x", 40, "low", "algorithmic", "Exponential DFS cycle enumeration; already parallelized; tuning work decomposition limits"),
    774: ("1.5-2x", 40, "med", "memory_bound", "No parallelism; clone+alloc per MPS step; pre-allocate scratch buffers; add rayon across cores"),
    391: ("2-4x", 60, "high", "memory_bound", "C uses global static arrays; Rust thread-local resizing; tune thread count vs L2/L3 cache"),
    681: ("1.5-2x", 40, "med", "brute_force", "Uneven rayon workload from highly-composite numbers; per-divisor-pair strategy for heavy K"),
    958: ("2-4x", 60, "low", "brute_force", "Single-threaded exponential BFS/DFS; add rayon at top levels; iterative deepening + memoize"),
    737: ("1.5-2x", 40, "low", "brute_force", "Sequential coin placement; C uses fewer sqrts; no parallelism possible; needs math shortcut"),
    829: ("2-4x", 60, "med", "algorithmic", "Single-threaded DFS; parallelize top-level prime enumeration with per-thread ShapeSystem"),
    735: ("1.5-2x", 40, "med", "parallelizable", "Already parallelized; tune CHUNK size; fuse compute_inner sub-loops; isqrt_f overhead"),
    448: ("1.5-2x", 40, "high", "data_structure", "HashMap -> FxHashMap; eliminate dyn Fn dispatch; u64 not i128 for mod arith"),

    # Batch 2: 9-10s
    847: ("2-4x", 60, "high", "data_structure", "HashMap -> FxHashMap; inner Vec allocs -> stack arrays; C uses flat open-addressing hash"),
    606: ("1.5-2x", 40, "high", "mod_arith", "Unnecessary i128 in inner Lucy DP loops; MOD<2^30, products fit u64"),
    378: ("1.2-1.5x", 25, "med", "memory_bound", "850MB working set; right_arr i64->i32 halves 480MB; free SPF before phase 2"),
    962: ("2-4x", 60, "med", "alloc_churn", "Vec per z and per v in inner loop; thread_local scratch buffers; SPF lookup vs trial division"),
    715: ("1.5-2x", 40, "high", "memory_bound", "ff: usize->u32 saves 400MB (800->400MB); parallelize big[i] fill"),
    513: ("4-10x", 75, "med", "algorithmic", "C uses fundamentally different Mobius inversion; port O(sqrt(N)) pair approach"),
    910: ("10x+", 90, "high", "algorithmic", "C uses simple phi recursion (45 lines); Rust uses FxHashMap over 1.9M entries; port C approach"),
    953: ("1.2-1.5x", 25, "low", "parallelizable", "Already well-parallelized; DFS load balance is the only remaining issue"),
    946: ("1.5-2x", 40, "med", "mod_arith", "10^8 iterations of i128 arithmetic; GCC __int128 faster than Rust software i128"),
    536: ("2-4x", 60, "med", "algorithmic", "Single-threaded recursion; parallelize root-level prime iteration with rayon"),

    # Batch 3: 7.8-8.5s
    552: ("1.2-1.5x", 25, "high", "algorithmic", "O(L^2) Garner CRT is fundamental; C identical; no structural improvement visible"),
    427: ("1.5-2x", 40, "med", "algorithmic", "O(N log N) mod-arith; precompute all fk in parallel then accumulate delta sequentially"),
    925: ("2-4x", 60, "low", "algorithmic", "Digit-tree DFS with u128; parallelize 10 start-digit branches per length with rayon"),
    691: ("1.5-2x", 40, "med", "memory_bound", "O(n log^2 n) SA construction; switch to SA-IS O(n); C uses static arrays"),
    931: ("1.5-2x", 40, "med", "algorithmic", "Lucy DP O(N^(2/3)); eliminate get_idx branching with flat two-array layout"),
    238: ("1.5-2x", 40, "med", "algorithmic", "format!() alloc in hot loop -> arithmetic digit extraction; O(D^2) double loop"),
    941: ("2-4x", 60, "med", "parallelizable", "10M serial rank_db calls; thread_local RankScratch + rayon par_iter"),
    886: ("1.2-1.5x", 25, "med", "algorithmic", "Memoized exponential search; C uses globals vs Rust Box<Ctx>; minimal opportunity"),
    461: ("1.2-1.5x", 25, "high", "alloc_churn", "85M pair allocation + sort; par_sort_unstable could help sort phase"),
    379: ("1.2-1.5x", 25, "high", "parallelizable", "Already well-parallelized; nested rayon contention; run T inner loop sequentially"),

    # Batch 4: 7.0-7.6s
    708: ("1.5-2x", 40, "med", "algorithmic", "Redundant sum_floor_quotients calls in DFS; parallelize independent subtrees"),
    558: ("4-10x", 75, "high", "algorithmic", "BigUint allocations dominate; parallelize outer j loop with rayon (independent)"),
    507: ("2-4x", 60, "high", "alloc_churn", "Vec alloc per gauss() call (20M calls); use stack arrays + rayon over 20M iterations"),
    543: ("1.5-2x", 40, "med", "memory_bound", "42 independent count_primes scans; sort queries, single linear scan instead"),
    614: ("1.2-1.5x", 25, "med", "parallelizable", "Already partially parallel; sequential Phase 2 dominates; transpose thread_temps"),
    415: ("1.5-2x", 40, "med", "algorithmic", "Lucy DP three passes fusible; incremental pow(2,g) instead of repeated mod_pow"),
    693: ("2-4x", 60, "high", "alloc_churn", "24MB used[] array zeroed per compute_g call; generation counter for O(1) clear"),
    611: ("1.5-2x", 40, "med", "algorithmic", "Lucy DP + DFS; precompute mod4 flags; reduce divisions in inner DFS loop"),
    559: ("2-4x", 60, "high", "brute_force", "O(N^2*H_N) DP; convolution structure may allow O(N log N); rayon over k values"),
    655: ("1.5-2x", 40, "med", "memory_bound", "10M-entry DP table; eliminate modulo in inner loop; track nonzero indices"),

    # Batch 5: 6.4-7.0s
    797: ("1.5-2x", 40, "med", "mod_arith", "Replace N pow_mod calls with linear inverse sieve; parallelize final sum"),
    540: ("2-4x", 60, "med", "parallelizable", "Vec alloc in get_prime_factors -> stack array; rayon on m-loop"),
    785: ("2-4x", 60, "high", "brute_force", "Rayon on outer m-loop (trivially independent); each m is self-contained"),
    505: ("1.2-1.5x", 25, "low", "algorithmic", "Inherently sequential recursive tree; C identical; minimal opportunity"),
    468: ("1.5-2x", 40, "med", "mod_arith", "Already optimized u32 + unsafe; parallelize small-B phase per-B"),
    417: ("1.2-1.5x", 25, "low", "parallelizable", "Already parallel; u128->u64 in pow_mod_big for small primes"),
    370: ("1.2-1.5x", 25, "low", "parallelizable", "Already parallel; Vec->stack array in get_prime_factors inside closure"),
    585: ("2-4x", 60, "med", "brute_force", "Parallelize O(N log N) second-case sum; optimize is_sq in inner loop"),
    534: ("2-4x", 60, "high", "data_structure", "HashMap -> FxHashMap for profile DP; pre-allocate across k iterations"),
    657: ("2-4x", 60, "high", "mod_arith", "Parallelize 10M pow[] init with rayon; i128->u64 in power_mod"),

    # Batch 6: 5.8-6.4s
    445: ("2-4x", 60, "high", "mod_arith", "i128->u64 in power_mod/mod_inv (MOD<2^30); precompute small p^e table"),
    864: ("2-4x", 60, "med", "algorithmic", "Rayon on Part B DFS; precompute squarefree sieve instead of trial division"),
    680: ("1.2-1.5x", 25, "low", "memory_bound", "Implicit treap already optimized with raw pointers; convert recursion to explicit stack"),
    642: ("1.5-2x", 40, "med", "algorithmic", "Lucy sieve + DFS; parallelize DFS top-level iterations; tri_mod branch elimination"),
    932: ("4-10x", 75, "high", "brute_force", "Single-threaded 10^8 iterations; embarrassingly parallel; precompute digit-length bands"),
    937: ("10x+", 90, "high", "algorithmic", "C uses trivial k%3!=2 formula; Rust has sophisticated sieve; verify and port C approach"),
    971: ("4-10x", 75, "high", "mod_arith", "u128->u64 in pow_mod (p<2^27); rayon over 1M qualifying primes; Vec->stack array"),
    972: ("1.5-2x", 40, "med", "data_structure", "Already parallel with FxHashMap; GeoKey is large; separate map for diameter case"),
    421: ("4-10x", 75, "high", "mod_arith", "u128->u64 pow_mod; rayon over 5.7M primes; embarrassingly parallel"),
    608: ("1.5-2x", 40, "med", "memory_bound", "800MB+ working set; chunked sieve for cache friendliness; not compute-bound"),

    # Batch 7: 5.4-5.8s
    769: ("2-4x", 60, "med", "brute_force", "Rayon on g loop (independent iterations); float sqrt inner loop"),
    947: ("1.5-2x", 40, "med", "alloc_churn", "Reuse scratch Vecs; flat cache instead of HashMap; u64 in Mat2::mul_mod"),
    437: ("2-4x", 60, "med", "parallelizable", "Rayon over primes; u128->u64 in fib_pair (p<2^27)"),
    521: ("1.2-1.5x", 25, "high", "memory_bound", "Algorithm near-optimal O(N^(2/3)); minor inlining only"),
    873: ("2-4x", 60, "high", "mod_arith", "Precompute inverse table (linear recurrence); i128->i64 (MOD<2^30)"),
    459: ("1.5-2x", 40, "med", "memory_bound", "Fuse mex+count loops to halve memory passes; O(N*sqrt(N)) is fundamental"),
    592: ("1.5-2x", 40, "med", "brute_force", "Parallelize f_vals block precomputation (27 independent blocks)"),
    482: ("1.2-1.5x", 25, "low", "parallelizable", "Already parallel; improve dedup with FxHashSet"),
    928: ("1.2-1.5x", 25, "med", "parallelizable", "Already parallel 125 tasks; tighten pruning bounds"),
    433: ("2-4x", 60, "med", "algorithmic", "Rayon over g; make extgcd iterative; both independent optimizations"),

    # Batch 8: 4.7-5.4s
    867: ("1.2-1.5x", 25, "med", "algorithmic", "Profile DP exponential in window; parallelize independent cache warmup"),
    557: ("4-10x", 75, "high", "brute_force", "Rayon over outer a loop; zero shared state; textbook parallel case"),
    637: ("2-4x", 60, "high", "brute_force", "Two independent compute_f calls; rayon within each 10M loop"),
    501: ("1.5-2x", 40, "med", "memory_bound", "pi_small Vec<i64>->Vec<u32> cuts 400MB; rayon on outer p loop"),
    416: ("1.5-2x", 40, "med", "mod_arith", "Parallel M1/M2 mat_pows (independent); deferred mod reduction"),
    650: ("2-4x", 60, "high", "mod_arith", "i128->u64 in power() (MOD<2^30); direct hot-path fix"),
    954: ("1.5-2x", 40, "med", "data_structure", "FxHashMap digit-DP; C uses DFS-with-pruning without hash maps"),
    583: ("1.5-2x", 40, "med", "algorithmic", "Inner O(n^2) loop; C uses O(n) two-pointer; port sorted scan"),
    476: ("1.2-1.5x", 25, "low", "brute_force", "Already parallelized and optimized; minor loop restructuring only"),
    314: ("2-4x", 60, "med", "brute_force", "MAX_STEP=15 vs C's 5; reducing step gives 9x fewer edges; verify correctness"),
}

# Build full triage list
triage = []
for pid, entry in sorted(entries.items()):
    t_ms = entry.get("time_ms", 0) or 0
    if pid in analysis:
        speedup, pct, conf, bottleneck, notes = analysis[pid]
    elif t_ms >= 3000:
        # Uninspected but still slow: default estimate
        speedup, pct, conf, bottleneck, notes = ("1.5-2x", 35, "low", "unknown", "Not inspected; runtime suggests optimization possible")
    elif t_ms >= 1000:
        speedup, pct, conf, bottleneck, notes = ("1.2-1.5x", 20, "low", "unknown", "Moderate runtime; may benefit from minor optimizations")
    else:
        speedup, pct, conf, bottleneck, notes = ("1x", 0, "n/a", "none", "Fast enough (<1s)")

    triage.append({
        "problem_id": pid,
        "current_runtime_ms": t_ms,
        "estimated_speedup_range": speedup,
        "estimated_time_saved_pct": pct,
        "confidence": conf,
        "bottleneck_class": bottleneck,
        "notes": notes,
    })

# Sort by estimated absolute time savings (runtime * pct)
triage.sort(key=lambda x: x["current_runtime_ms"] * x["estimated_time_saved_pct"] / 100, reverse=True)

# Write CSV
with open("optimization_triage.csv", "w", newline="") as f:
    w = csv.writer(f)
    w.writerow(["problem_id", "current_runtime_ms", "estimated_speedup_range",
                "estimated_time_saved_pct", "confidence", "bottleneck_class", "notes"])
    for row in triage:
        w.writerow([row["problem_id"], row["current_runtime_ms"], row["estimated_speedup_range"],
                     row["estimated_time_saved_pct"], row["confidence"], row["bottleneck_class"],
                     row["notes"]])

# Write JSON (keyed by problem_id)
triage_json = {}
for row in triage:
    pid = row["problem_id"]
    triage_json[str(pid)] = {
        "problem": pid,
        "current_runtime_ms": row["current_runtime_ms"],
        "estimated_speedup_range": row["estimated_speedup_range"],
        "estimated_time_saved_pct": row["estimated_time_saved_pct"],
        "confidence": row["confidence"],
        "bottleneck_class": row["bottleneck_class"],
        "notes": row["notes"],
    }

with open("optimization_triage.json", "w") as f:
    json.dump(triage_json, f, indent=2)

# Summary stats
total = len(triage)
inspected = len(analysis)
high_conf = [t for t in triage if t["confidence"] == "high" and t["estimated_time_saved_pct"] >= 40]
top15 = triage[:15]

total_time_ms = sum(t["current_runtime_ms"] for t in triage)
est_savings_ms = sum(t["current_runtime_ms"] * t["estimated_time_saved_pct"] / 100 for t in triage)

print(f"\n{'='*80}")
print(f"OPTIMIZATION TRIAGE SUMMARY")
print(f"{'='*80}")
print(f"Total problems scanned:     {total}")
print(f"Problems code-inspected:    {inspected}")
print(f"Total current runtime:      {total_time_ms/1000:.1f}s ({total_time_ms/60000:.1f} min)")
print(f"Estimated saveable time:    {est_savings_ms/1000:.1f}s ({est_savings_ms/60000:.1f} min)")
print(f"High-confidence targets:    {len(high_conf)}")
print()

print(f"TOP 15 HIGHEST-PRIORITY OPTIMIZATION TARGETS")
print(f"{'─'*80}")
print(f"{'Rank':>4s}  {'Problem':>7s}  {'Runtime':>8s}  {'Speedup':>8s}  {'Conf':>5s}  {'Bottleneck':<18s}  Notes")
print(f"{'─'*80}")
for i, t in enumerate(top15, 1):
    rt = f"{t['current_runtime_ms']/1000:.1f}s"
    print(f"{i:>4d}  p{t['problem_id']:<6d}  {rt:>8s}  {t['estimated_speedup_range']:>8s}  {t['confidence']:>5s}  {t['bottleneck_class']:<18s}  {t['notes'][:50]}")
print()

# Data quality gaps
print("DATA QUALITY NOTES:")
no_time = [pid for pid, e in entries.items() if e.get("time_ms") is None]
if no_time:
    print(f"  - {len(no_time)} problems missing time_ms: {no_time[:10]}...")
else:
    print("  - All 984 problems have time_ms recorded")

wrong = [pid for pid, e in entries.items() if e.get("status") != "OK"]
if wrong:
    print(f"  - {len(wrong)} problems with non-OK status: {wrong[:10]}...")
else:
    print("  - All problems have status OK")

slow_uninspected = [t for t in triage if t["current_runtime_ms"] >= 3000 and t["bottleneck_class"] == "unknown"]
if slow_uninspected:
    pids = [t["problem_id"] for t in slow_uninspected]
    print(f"  - {len(slow_uninspected)} problems >=3s not code-inspected: {pids}")
else:
    print("  - All problems >=3s were code-inspected")

print(f"\nCreated files:")
print(f"  euler/optimization_triage.csv")
print(f"  euler/optimization_triage.json")
