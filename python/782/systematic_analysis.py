#!/usr/bin/env python3
"""
Systematic analysis of C(n) for Project Euler 782.

Key facts:
- c(n,k) = 1 for k in {0, n^2}
- c(n,k) = 2 for k in S2 (derived closed form)
- c(n,k) >= 3 for remaining k
- C(n) = sum of c(n,k) for k=0..n^2

If c(n,k) in {1,2,3} for all k:
  C(n) = 2 + 2*|S2| + 3*(n^2-1-|S2|) = 2 + 2*N2 + 3*(n^2-1-N2) = 3n^2-1-N2

If some k have c(n,k) = 4:
  C(n) = 3n^2 - 1 - N2 + N4  (where N4 = count of k with c=4)

Let's verify this and find the pattern.
"""

import itertools
from collections import Counter


def compute_comp2_set(n):
    """Compute the set of k values with c(n,k) = 2."""
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


def C_assuming_max3(n):
    """C(n) assuming max c(n,k) = 3."""
    N2 = len(compute_comp2_set(n))
    return 3 * n * n - 1 - N2


# First, verify C(n) for small n by brute force
def brute_force_cnk(n):
    """Exact c(n,k) for all k by brute force over all n*n binary matrices."""
    best = {}
    for bits in range(1 << (n*n)):
        matrix = []
        for i in range(n):
            row = tuple((bits >> (i*n + j)) & 1 for j in range(n))
            matrix.append(row)

        k = sum(sum(r) for r in matrix)
        patterns = set(matrix)
        for j in range(n):
            col = tuple(matrix[i][j] for i in range(n))
            patterns.add(col)
        comp = len(patterns)

        if k not in best or comp < best[k]:
            best[k] = comp
    return best


def brute_force_cnk_3types(n):
    """Exact c(n,k) using up to 3 row types (faster than full brute force for n=5,6)."""
    best = {}
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)

    # 1 row type
    for P in all_vecs:
        k = n * sum(P)
        patterns = set()
        patterns.add(P)
        for j in range(n):
            patterns.add(tuple(P[j] for _ in range(n)))
        comp = len(patterns)
        if k not in best or comp < best[k]:
            best[k] = comp

    # 2 row types
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

    # 3 row types (for n <= 5)
    if n <= 5:
        for ip in range(nv):
            P = all_vecs[ip]
            for iq in range(ip, nv):
                Q = all_vecs[iq]
                for ir in range(iq, nv):
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
                        comp = len(patterns)
                        if k not in best or comp < best[k]:
                            best[k] = comp

    return best


print("=== Brute force c(n,k) for small n ===")
print()

for n in range(1, 5):
    best = brute_force_cnk(n)
    C = sum(best.get(k, 999) for k in range(n*n+1))
    dist = Counter(best[k] for k in range(n*n+1))
    max_c = max(best.values())
    print(f"n={n}: C={C}, max_c={max_c}, dist={dict(sorted(dist.items()))}")
    S2 = compute_comp2_set(n)
    N2 = len(S2)
    C_formula = C_assuming_max3(n)
    print(f"  N2={N2}, C_formula(max3)={C_formula}, match={C==C_formula}")

    # Show c(n,k) for each k
    if n <= 4:
        vals = [best.get(k, '?') for k in range(n*n+1)]
        print(f"  c({n},k): {vals}")

print()
print("=== 3-type search for n=5 ===")
n = 5
best5 = brute_force_cnk_3types(5)
C5 = sum(best5.get(k, 999) for k in range(26))
dist5 = Counter(best5[k] for k in range(26))
max_c5 = max(best5.values())
print(f"n=5: C={C5}, max_c={max_c5}, dist={dict(sorted(dist5.items()))}")
S2_5 = compute_comp2_set(5)
N2_5 = len(S2_5)
print(f"  N2={N2_5}, C_formula(max3)={C_assuming_max3(5)}, match={C5==C_assuming_max3(5)}")

# Which k values have c(5,k) = 3?
c3_vals = sorted(k for k in range(26) if best5.get(k, 999) == 3)
print(f"  c(5,k)=3 for k in {c3_vals}")

print()
print("=== C(n) with formula for n=1..30 ===")
for n in range(1, 31):
    S2 = compute_comp2_set(n)
    N2 = len(S2)
    C_max3 = 3 * n * n - 1 - N2
    print(f"n={n:2d}: N2={N2:4d}, C(max3)={C_max3:6d}")

print()
print("Expected values: C(2)=8, C(5)=64, C(10)=274, C(20)=1150")
for n in [2, 5, 10, 20]:
    print(f"  C_formula({n}) = {C_assuming_max3(n)}")
