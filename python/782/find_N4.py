#!/usr/bin/env python3
"""
For small n, compute exact c(n,k) and find the k values with c(n,k) >= 4.

Strategy:
- For n<=4: full brute force over all 2^(n^2) matrices
- For n=5: 3-type search (already confirmed C(5)=64, so all c(n,k) <= 3)
- For n=6,7: need to check if c(n,k) > 3 exists

For n=6: 2^36 ≈ 69 billion full brute force is infeasible.
But we can use 2-type direct enumeration (P,Q,σ) to get comp values,
and for k values that still show comp >= 4, try 3-type to bring them down.

For n=6: 2-type gives 64^2 * 64 = 262144 combinations (fast).
3-type for remaining k: 64^3 * 3^6 ≈ 191M (feasible but slow).

Actually, let me be smarter. For k values with c_2type >= 4,
try 3-type search only for those specific k values.
"""

import itertools
from collections import Counter


def compute_comp2_set(n):
    s = set()
    for c in range(1, n):
        s.add(c * c)
        s.add(n * n - c * c)
    for x in range(1, n):
        y = n - x
        s.add(x * x + y * y)
        s.add(2 * x * y)
    s.discard(0)
    s.discard(n * n)
    return s


def compute_cnk_2types(n):
    """For each k, find min complexity using 2 row types."""
    best = {}
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)
    for ip in range(nv):
        P = all_vecs[ip]
        for iq in range(ip, nv):
            Q = all_vecs[iq]
            for sigma in all_vecs:
                k = 0
                patterns = set()
                if 1 in sigma:
                    patterns.add(P)
                if 0 in sigma:
                    patterns.add(Q)
                for j in range(n):
                    col = tuple(P[j] if sigma[i] else Q[j] for i in range(n))
                    patterns.add(col)
                    k += sum(col)
                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp
    return best


def check_comp3_for_k_values(n, target_ks):
    """Try 3-type search to find comp <= 3 for specific k values."""
    achieved = set()
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)
    remaining = set(target_ks)

    for ip in range(nv):
        if not remaining:
            break
        P = all_vecs[ip]
        for iq in range(ip, nv):
            if not remaining:
                break
            Q = all_vecs[iq]
            for ir in range(iq, nv):
                if not remaining:
                    break
                R = all_vecs[ir]
                types = [P, Q, R]
                for sigma in itertools.product(range(3), repeat=n):
                    k = 0
                    patterns = set()
                    for i in range(n):
                        patterns.add(types[sigma[i]])
                    for j in range(n):
                        col = tuple(types[sigma[i]][j] for i in range(n))
                        patterns.add(col)
                        k += sum(col)
                    if k in remaining and len(patterns) <= 3:
                        achieved.add(k)
                        remaining.discard(k)

    return achieved, remaining


# First, run 2-type analysis for n=6,7,8
for n in [6, 7, 8]:
    print(f"\n=== n = {n} ===")
    best = compute_cnk_2types(n)
    dist = Counter(best.get(k, 999) for k in range(n*n+1))
    print(f"2-type distribution: {dict(sorted(dist.items()))}")

    # k values needing more than 2 types to get comp <= 3
    need_3type = sorted(k for k in range(n*n+1) if best.get(k, 999) > 3)
    print(f"k with 2-type comp > 3: {len(need_3type)} values: {need_3type[:30]}")

    if n <= 7 and need_3type:
        print(f"Trying 3-type search for {len(need_3type)} k values...")
        achieved, remaining = check_comp3_for_k_values(n, need_3type)
        if remaining:
            print(f"  Still need comp >= 4: {sorted(remaining)}")
        else:
            print(f"  ALL achieved comp <= 3!")

        # Now compute exact C(n)
        C = 0
        for k in range(n*n+1):
            c = best.get(k, 999)
            if k in achieved:
                c = 3  # we know comp <= 3 from 3-type search
            C += c
        S2 = compute_comp2_set(n)
        print(f"  C({n}) = {C}, formula(max3) = {3*n*n - 1 - len(S2)}")
