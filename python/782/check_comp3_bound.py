#!/usr/bin/env python3
"""
Can we always achieve c(n,k) ≤ 3?

Key idea: use matrices where ALL rows and ALL columns come from a set of 3 patterns.
With 3 patterns, we have much more flexibility.

For complexity ≤ 3 with patterns {A, B, C}:
- Each row ∈ {A, B, C} and each column ∈ {A, B, C}.

Construction: "Corner" or "staircase" blocks.

Consider: a1 rows of all-zeros, a2 rows with 1s in first b positions, a3 rows of all-ones.
k = a2*b + a3*n.

Column types:
- Cols 0..b-1: has 0s in first a1 rows, 1s in next a2 rows, 1s in last a3 rows.
  = (0^a1, 1^{a2+a3})
- Cols b..n-1: has 0s in first a1 rows, 0s in next a2 rows, 1s in last a3 rows.
  = (0^{a1+a2}, 1^a3)

Row patterns: (0^n), (1^b, 0^{n-b}), (1^n).
Col patterns: (0^a1, 1^{a2+a3}), (0^{a1+a2}, 1^a3).
Union: up to 5, but some may coincide.

For example, if a1=0: no (0^n) row. Cols: (1^{a2+a3})=(1^n), and (0^{a2}, 1^{a3}).
Row types: (1^b, 0^{n-b}), (1^n).
Col types: (1^n), (0^{a2}, 1^{a3}).
(1^n) is shared. So union = {(1^b, 0^{n-b}), (1^n), (0^{a2}, 1^{a3})}.
If a2 = n-b: this is {(1^b, 0^{n-b}), (1^n), (0^{n-b}, 1^b)} = 3 patterns if b < n.
k = a2*b + a3*n = (n-a3)*b + a3*n = nb - a3*b + a3*n = nb + a3(n-b).

Hmm, let me try a different approach.

Consider the matrix M where:
M[i][j] = 1 iff (i < a1 and j < b1) or (i < a2 and j < b2)
where a1 ≤ a2 and b1 ≥ b2 (or a1 ≥ a2 and b1 ≤ b2).

This is a "staircase" pattern. Let me think about this differently.

Actually, let me just try the 3-row-type approach with σ∈{0,1,2}^n for small n.
For n=5,6, use 3 row types and see if we can get complexity 3 for all k.
"""

import itertools

def compute_cnk_3types(n):
    """For each k, find min complexity using up to 3 row types."""
    best = {}

    all_vecs = list(itertools.product([0, 1], repeat=n))
    nvecs = len(all_vecs)

    # For 3 row types P, Q, R:
    # σ ∈ {0,1,2}^n: σ[i] tells which row type row i uses.
    # M[i][j] = P[j] if σ[i]=0, Q[j] if σ[i]=1, R[j] if σ[i]=2.
    # Column j = tuple(types[σ[i]][j] for i in range(n)) where types = [P, Q, R].

    # This is too many combinations for n=6:
    # C(2^6, 3) * 3^6 ≈ 46k * 729 ≈ 34M. Might be feasible.

    # For n=5: C(32, 3) * 3^5 ≈ 5k * 243 ≈ 1.2M. Feasible.

    # Actually, let me also include 1 and 2 row types.
    # Start with 2 types (already done) and then try 3 types for remaining.

    # First, compute best with 2 types
    for i_p in range(nvecs):
        P = all_vecs[i_p]
        for i_q in range(i_p, nvecs):
            Q = all_vecs[i_q]
            for sigma in all_vecs:
                k = 0
                patterns = set()
                for row_type in set(sigma):
                    patterns.add(P if row_type == 0 else Q)
                # Actually sigma here is binary, 0=P, 1=Q
                row_patterns = set()
                if 0 in set(sigma) or 1 in set(sigma):
                    pass

    # This is getting convoluted. Let me just do it directly.

    # For n=5, full 3-type search:
    if n <= 5:
        # Enumerate all (P, Q, R) triples and all σ ∈ {0,1,2}^n
        count = 0
        for i_p in range(nvecs):
            P = all_vecs[i_p]
            for i_q in range(i_p, nvecs):
                Q = all_vecs[i_q]
                for i_r in range(i_q, nvecs):
                    R = all_vecs[i_r]
                    types = [P, Q, R]
                    for sigma in itertools.product(range(3), repeat=n):
                        k = 0
                        patterns = set()
                        for i in range(n):
                            patterns.add(types[sigma[i]])
                        for j in range(n):
                            col = tuple(types[sigma[i]][j] for i in range(n))
                            patterns.add(col)
                            k += sum(1 for i in range(n) if types[sigma[i]][j] == 1)

                        comp = len(patterns)
                        if k not in best or comp < best[k]:
                            best[k] = comp
                        count += 1
                        if count % 10000000 == 0:
                            print(f"  Processed {count // 1000000}M combinations...")

        print(f"  Total combinations: {count}")

    return best


for n in [5]:
    print(f"\nn={n} (3-type search):")
    best = compute_cnk_3types(n)
    total = 0
    for k in range(n*n+1):
        c = best.get(k, 999)
        total += c
        if c > 3:
            print(f"  c({n},{k}) = {c}  <-- still > 3!")
    print(f"  C({n}) = {total}")
    print(f"  Expected: C(5) = 64")
