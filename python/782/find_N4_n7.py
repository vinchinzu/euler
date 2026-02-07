#!/usr/bin/env python3
"""Find N4 for n=7,8 using optimized approach."""

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
                k = s * wp + (n - s) * wq

                patterns = set()
                if s > 0:
                    patterns.add(P)
                if s < n:
                    patterns.add(Q)
                for j in range(n):
                    if P[j] == Q[j]:
                        col = (P[j],) * n
                    elif P[j] == 1:
                        col = sigma
                    else:
                        col = tuple(1 - sigma[i] for i in range(n))
                    patterns.add(col)

                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp
    return best


def check_comp3_targeted(n, target_ks):
    """Check if target k values can achieve comp <= 3 with 3 row types.

    Optimization: use block-type 3-row constructions (fast O(n^4) parametric)
    before falling back to full enumeration.
    """
    achieved = set()
    remaining = set(target_ks)

    # Block construction: rows of type 0^n, (1^w 0^{n-w}), 1^n
    # with a1, a2, a3 rows of each type in block arrangement
    for w in range(1, n):
        for a1 in range(n + 1):
            for a2 in range(n + 1 - a1):
                a3 = n - a1 - a2
                k = a1 * n + a2 * w
                if k not in remaining:
                    continue

                patterns = set()
                if a1 > 0: patterns.add((1,) * n)
                if a2 > 0: patterns.add((1,) * w + (0,) * (n - w))
                if a3 > 0: patterns.add((0,) * n)

                if w > 0:
                    patterns.add((1,) * (a1 + a2) + (0,) * a3)
                if w < n:
                    patterns.add((1,) * a1 + (0,) * (n - a1))

                if len(patterns) <= 3:
                    achieved.add(k)
                    remaining.discard(k)

    if not remaining:
        return achieved, remaining

    # Generalized block: 3 row types R1, R2, R3 with block arrangement (R1 first, then R2, then R3)
    # R1 = (1^w1, 0^{n-w1}), R2 = (1^w2, 0^{n-w2}), R3 = (1^w3, 0^{n-w3})
    # a1 rows of R1, a2 rows of R2, a3 rows of R3
    for w1 in range(n + 1):
        for w2 in range(n + 1):
            for w3 in range(n + 1):
                for a1 in range(n + 1):
                    for a2 in range(n + 1 - a1):
                        a3 = n - a1 - a2
                        k = a1 * w1 + a2 * w2 + a3 * w3
                        if k not in remaining:
                            continue

                        # Build actual matrix and compute complexity
                        R1 = (1,) * w1 + (0,) * (n - w1)
                        R2 = (1,) * w2 + (0,) * (n - w2)
                        R3 = (1,) * w3 + (0,) * (n - w3)

                        patterns = set()
                        if a1 > 0: patterns.add(R1)
                        if a2 > 0: patterns.add(R2)
                        if a3 > 0: patterns.add(R3)

                        rows = [R1] * a1 + [R2] * a2 + [R3] * a3
                        for j in range(n):
                            col = tuple(rows[i][j] for i in range(n))
                            patterns.add(col)

                        if len(patterns) <= 3:
                            achieved.add(k)
                            remaining.discard(k)

    if not remaining:
        return achieved, remaining

    print(f"  After block constructions, remaining: {sorted(remaining)}")

    # Full 3-type enumeration for remaining k values
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)

    count = 0
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
                    for j in range(n):
                        for i in range(n):
                            k += types[sigma[i]][j]

                    if k not in remaining:
                        continue

                    patterns = set()
                    used = set(sigma)
                    for u in used:
                        patterns.add(types[u])
                    for j in range(n):
                        col = tuple(types[sigma[i]][j] for i in range(n))
                        patterns.add(col)

                    if len(patterns) <= 3:
                        achieved.add(k)
                        remaining.discard(k)

                    count += 1

    print(f"  Full 3-type: processed {count}, remaining: {sorted(remaining)}")
    return achieved, remaining


for n in [7]:
    print(f"\n=== n = {n} ===")
    best2 = compute_cnk_2types(n)
    dist2 = Counter(best2.get(k, 999) for k in range(n*n+1))
    print(f"2-type dist: {dict(sorted(dist2.items()))}")

    need_more = sorted(k for k in range(n*n+1) if best2.get(k, 999) > 3)
    print(f"k with 2-type comp > 3: {need_more}")

    if need_more:
        achieved, remaining = check_comp3_targeted(n, need_more)

        S2 = compute_comp2_set(n)
        N4 = len(remaining)
        C_max3 = 3*n*n - 1 - len(S2)
        C_actual = C_max3 + N4
        print(f"N4 = {N4}, C({n}) = {C_actual}, formula(max3) = {C_max3}")
    else:
        S2 = compute_comp2_set(n)
        print(f"All k achieve comp <= 3. C({n}) = {3*n*n-1-len(S2)}")
