#!/usr/bin/env python3
"""
Fast computation of c(n,k) for n=6.

For n=6:
- 2-type: 64^2 * 64 / 2 ≈ 131k. Fast.
- 3-type for remaining: try (P,Q,R) triples and σ ∈ {0,1,2}^6 (729 values).
  64^3/6 * 729 ≈ 32M. Should be OK with optimized code.

Actually, we only need 3-type for k values that don't achieve comp<=3 with 2 types.
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
        wp = sum(P)
        for iq in range(ip, nv):
            Q = all_vecs[iq]
            wq = sum(Q)
            for sigma in all_vecs:
                s = sum(sigma)
                k = s * wp + (n - s) * wq if P != Q else n * wp
                # Need exact k based on actual matrix entries
                # Actually k = sum over j of (count of 1s in column j)
                # = sum over i,j of M[i][j]
                # = sum over i of (weight of row i)
                # = s * wp + (n-s) * wq when sigma has s ones

                # But wait: this is only correct if P and Q are the row types
                # and sigma selects between them. Yes, that's correct.

                patterns = set()
                if s > 0:
                    patterns.add(P)
                if s < n:
                    patterns.add(Q)
                for j in range(n):
                    if P[j] == Q[j]:
                        col = (P[j],) * n
                    elif P[j] == 1:  # P[j]=1, Q[j]=0
                        col = sigma
                    else:  # P[j]=0, Q[j]=1
                        col = tuple(1 - sigma[i] for i in range(n))
                    patterns.add(col)

                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp
    return best


def compute_cnk_3types(n, target_ks=None):
    """For each k, find min comp using 3 row types. If target_ks given, only check those."""
    best = {}
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)

    remaining = set(target_ks) if target_ks else None
    count = 0

    for ip in range(nv):
        if remaining is not None and not remaining:
            break
        P = all_vecs[ip]
        for iq in range(ip, nv):
            if remaining is not None and not remaining:
                break
            Q = all_vecs[iq]
            for ir in range(iq, nv):
                if remaining is not None and not remaining:
                    break
                R = all_vecs[ir]
                types = [P, Q, R]
                for sigma in itertools.product(range(3), repeat=n):
                    # Compute k quickly
                    k = 0
                    for j in range(n):
                        for i in range(n):
                            k += types[sigma[i]][j]

                    if remaining is not None and k not in remaining:
                        continue

                    patterns = set()
                    used = set(sigma)
                    for u in used:
                        patterns.add(types[u])
                    for j in range(n):
                        col = tuple(types[sigma[i]][j] for i in range(n))
                        patterns.add(col)

                    comp = len(patterns)
                    if k not in best or comp < best[k]:
                        best[k] = comp
                        if comp <= 3 and remaining is not None:
                            remaining.discard(k)

                    count += 1

    if count > 0:
        print(f"  Processed {count} combinations")
    return best


# n = 6
print("=== n = 6 ===")
n = 6
best2 = compute_cnk_2types(n)
dist2 = Counter(best2.get(k, 999) for k in range(n*n+1))
print(f"2-type dist: {dict(sorted(dist2.items()))}")

need_more = sorted(k for k in range(n*n+1) if best2.get(k, 999) > 3)
print(f"k with 2-type comp > 3: {need_more}")

# Try 3-type for those k values
if need_more:
    print(f"Searching 3-type for {len(need_more)} k values...")
    best3 = compute_cnk_3types(n, set(need_more))

    still_gt3 = sorted(k for k in need_more if best3.get(k, 999) > 3)
    print(f"  Still > 3 after 3-type: {still_gt3}")

    # Compute C(n)
    combined = {}
    for k in range(n*n+1):
        c2 = best2.get(k, 999)
        c3 = best3.get(k, 999) if k in best3 else 999
        combined[k] = min(c2, c3)
    C = sum(combined[k] for k in range(n*n+1))
    S2 = compute_comp2_set(n)
    C_max3 = 3*n*n - 1 - len(S2)
    print(f"C({n}) = {C}, formula(max3) = {C_max3}")
    dist = Counter(combined[k] for k in range(n*n+1))
    print(f"Combined dist: {dict(sorted(dist.items()))}")
else:
    S2 = compute_comp2_set(n)
    C = sum(best2.get(k, 999) for k in range(n*n+1))
    print(f"C({n}) = {C}")
