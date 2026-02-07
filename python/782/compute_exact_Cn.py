#!/usr/bin/env python3
"""
Compute exact C(n) for small n using efficient methods.

For n <= 4: full brute force (2^(n^2) matrices).
For n = 5: 3-type search (verified C(5)=64).
For n = 6,7: 2-type + targeted 3-type.
For n >= 8: need block staircase + other constructions.

Strategy: compute N2 (formula) + try to determine N3 exactly.
The remaining k values have c(n,k) >= 4.

Actually let me try to use a DIFFERENT approach for larger n:
Use the 3x3 self-contained matrix enumeration (block construction)
PLUS the partition construction with ALL permutations.
PLUS other non-block constructions.

Let me first compute exact C(n) for n=1..8 using computational methods,
then look at the sequence.
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


def brute_force_cnk(n):
    """Full brute force over all 2^(n^2) matrices. Only for n <= 4."""
    best = {}
    total = 1 << (n * n)
    for bits in range(total):
        matrix = []
        for i in range(n):
            row = tuple((bits >> (i * n + j)) & 1 for j in range(n))
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


def cnk_2type(n):
    """Min complexity using 2 row types and any σ."""
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
                s = sum(sigma)
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
                    k += sum(1 for i in range(n) if (P[j] if sigma[i] else Q[j]))

                # Compute k from weights
                wp = sum(P)
                wq = sum(Q)
                k = s * wp + (n - s) * wq

                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp
    return best


def cnk_3type_targeted(n, target_ks):
    """Check if target k values achieve comp <= 3 using 3 row types."""
    achieved = {}
    remaining = set(target_ks)
    all_vecs = list(itertools.product([0, 1], repeat=n))
    nv = len(all_vecs)

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
                    k = sum(types[sigma[i]][j] for i in range(n) for j in range(n))
                    if k not in remaining:
                        continue
                    patterns = set()
                    for v in set(sigma):
                        patterns.add(types[v])
                    for j in range(n):
                        col = tuple(types[sigma[i]][j] for i in range(n))
                        patterns.add(col)
                    comp = len(patterns)
                    if comp <= 3:
                        achieved[k] = 3
                        remaining.discard(k)
    return achieved, remaining


# Compute exact C(n) for n = 1..4 by brute force
print("=== Exact C(n) by brute force (n <= 4) ===")
for n in range(1, 5):
    best = brute_force_cnk(n)
    C = sum(best.get(k, 999) for k in range(n * n + 1))
    print(f"C({n}) = {C}")

# n = 5: use 2-type + 3-type
print("\n=== n = 5 (2-type + 3-type) ===")
n = 5
best5 = cnk_2type(5)
gt3 = [k for k in range(26) if best5.get(k, 999) > 3]
print(f"n=5: 2-type gives {len(gt3)} k with comp>3: {gt3}")
if gt3:
    ach, rem = cnk_3type_targeted(5, gt3)
    print(f"  After 3-type: remaining={sorted(rem)}")
    for k in gt3:
        if k in ach:
            best5[k] = 3
C5 = sum(best5.get(k, 999) for k in range(26))
print(f"C(5) = {C5}")

# n = 6: use 2-type + 3-type
print("\n=== n = 6 (2-type + 3-type) ===")
n = 6
best6 = cnk_2type(6)
gt3_6 = [k for k in range(37) if best6.get(k, 999) > 3]
print(f"n=6: 2-type gives {len(gt3_6)} k with comp>3: {gt3_6}")
if gt3_6:
    ach6, rem6 = cnk_3type_targeted(6, gt3_6)
    print(f"  After 3-type: remaining={sorted(rem6)}")
    for k in gt3_6:
        if k in ach6:
            best6[k] = 3
C6 = sum(best6.get(k, 999) for k in range(37))
print(f"C(6) = {C6}")

# n = 7: use 2-type + 3-type
print("\n=== n = 7 (2-type + 3-type) ===")
n = 7
best7 = cnk_2type(7)
gt3_7 = [k for k in range(50) if best7.get(k, 999) > 3]
print(f"n=7: 2-type gives {len(gt3_7)} k with comp>3: {gt3_7}")
if gt3_7:
    ach7, rem7 = cnk_3type_targeted(7, gt3_7)
    print(f"  After 3-type: remaining={sorted(rem7)}")
    for k in gt3_7:
        if k in ach7:
            best7[k] = 3
C7 = sum(best7.get(k, 999) for k in range(50))
print(f"C(7) = {C7}")

# n = 8: 2-type is feasible, 3-type targeted
print("\n=== n = 8 (2-type + block 3-type) ===")
n = 8
best8 = cnk_2type(8)
gt3_8 = [k for k in range(65) if best8.get(k, 999) > 3]
print(f"n=8: 2-type gives {len(gt3_8)} k with comp>3: {gt3_8}")

# For n=8, full 3-type is too slow (256^3/6 * 6561 ≈ 18B)
# Use block staircase instead
remaining_8 = set(gt3_8)
for w1 in range(n + 1):
    R1 = (1,) * w1 + (0,) * (n - w1)
    for w2 in range(n + 1):
        R2 = (1,) * w2 + (0,) * (n - w2)
        for w3 in range(n + 1):
            R3 = (1,) * w3 + (0,) * (n - w3)
            for a in range(n + 1):
                for b in range(n + 1 - a):
                    c = n - a - b
                    k = a * w1 + b * w2 + c * w3
                    if k not in remaining_8:
                        continue
                    patterns = set()
                    if a > 0: patterns.add(R1)
                    if b > 0: patterns.add(R2)
                    if c > 0: patterns.add(R3)
                    rows = [R1] * a + [R2] * b + [R3] * c
                    for j in range(n):
                        col = tuple(rows[i][j] for i in range(n))
                        patterns.add(col)
                    if len(patterns) <= 3:
                        remaining_8.discard(k)

print(f"  After block 3-type: remaining={sorted(remaining_8)}")

# For remaining, try partition construction
for p in range(1, n):
    for q in range(1, n - p):
        r = n - p - q
        for k_val in [p*p+q*q+r*r, p*p+2*q*r, q*q+2*p*r, r*r+2*p*q, p*q+q*r+p*r]:
            remaining_8.discard(k_val)
            remaining_8.discard(n*n - k_val)
for p in range(1, n):
    for q in range(1, n - p + 1):
        remaining_8.discard(p*p + q*q)
        remaining_8.discard(n*n - p*p - q*q)
        remaining_8.discard(2*p*q)
        remaining_8.discard(n*n - 2*p*q)
for w in range(1, n):
    remaining_8.discard(n * w)

print(f"  After all constructions: remaining={sorted(remaining_8)}")

for k in gt3_8:
    if k not in remaining_8:
        best8[k] = 3
C8 = sum(best8.get(k, 999) for k in range(65))
print(f"C(8) = {C8}")
print(f"  C_max3(8) = {3*64 - 1 - len(compute_comp2_set(8))}")

print("\n=== Summary ===")
print("n : C(n)")
for n, C in [(1, 2), (2, 8), (3, 22), (4, 38)]:
    print(f"{n:2d}: {C}")
print(f" 5: {C5}")
print(f" 6: {C6}")
print(f" 7: {C7}")
print(f" 8: {C8}")
