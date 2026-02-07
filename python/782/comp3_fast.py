#!/usr/bin/env python3
"""
Fast computation of comp-3 achievable k values.

Key constructions for comp <= 3:

1. Comp 1: k ∈ {0, n^2}

2. Comp 2: k ∈ S2 (known formula)

3. Comp 3 from "partition" construction (A+B+C = 1^n):
   k = wA^2 + wB^2 + wC^2 where wA+wB+wC = n, all >= 1
   k = wA*wB + wB*wC + wC*wA (complement)

4. Comp 3 from "2 groups + zeros" (positions split into A, B, empty):
   Row types: A, B, 0^n. Position j has (A[j],B[j],0): either (1,0,0), (0,1,0), or (0,0,0).
   Columns: I_0 (= A), I_1 (= B), 0^n. All in {A, B, 0^n}. ✓
   σ: σ[i]=0 iff A[i]=1, σ[i]=1 iff B[i]=1, σ[i]=2 otherwise.
   a = wA, b = wB, c = n - wA - wB.
   k = wA*wA + wB*wB + 0 = wA^2 + wB^2, with wA+wB <= n, wA,wB >= 1.
   Complement: n^2 - wA^2 - wB^2.

5. Comp 3 from "2 groups + ones":
   Row types: A, B, 1^n. A+B+1^n has ≥ 1 per position.
   Need (1-A) + (1-B) = 1^n - A - B. For triples (A[j],B[j],1):
   (0,0,1), (0,1,1), (1,0,1), (1,1,1).
   Columns: I_2 (wherever C[j]=1, but C=1^n so always), but that's for triple (0,0,1)->I_2=indicator(σ=2).

   Actually let me just think of it as complement: if M' = J - M, then comp(M') = comp(M) and k' = n^2 - k.
   So "2 groups + ones" gives k = n^2 - (wA'^2 + wB'^2) where A'+B' is the complement configuration.
   This is already covered by construction 4's complement.

6. Comp 3 from "1 row type" (all identical rows):
   Row = R (weight w), 0 < w < n.
   Patterns: {R, 0^n, 1^n}. Comp = 3 if R ≠ 0^n and R ≠ 1^n.
   k = n*w.
   These are k = n*w for w=1..n-1.

7. Block 3-type staircase:
   3 left-aligned row types (1^w1,0^{n-w1}), (1^w2,0^{n-w2}), (1^w3,0^{n-w3}).
   Block arrangement: a, b, c rows.
   Let me optimize: only check for comp <= 3.

Let me compute all achievable k values using constructions 1-6 first,
then add block staircase.
"""

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


def compute_comp3_analytic(n):
    """Comp-3 k values from analytic constructions."""
    achievable = set()

    # Construction 3: partition A+B+C = 1^n
    for wA in range(1, n):
        for wB in range(1, n - wA):
            wC = n - wA - wB
            achievable.add(wA**2 + wB**2 + wC**2)
            achievable.add(wA*wB + wB*wC + wC*wA)

    # Construction 4: 2 groups + zeros: k = wA^2 + wB^2
    for wA in range(1, n):
        for wB in range(1, n - wA + 1):
            achievable.add(wA**2 + wB**2)
            achievable.add(n**2 - wA**2 - wB**2)

    # Construction 6: all identical rows: k = n*w
    for w in range(1, n):
        achievable.add(n * w)

    achievable.discard(0)
    achievable.discard(n * n)
    return achievable


def compute_comp3_block_staircase(n):
    """Comp-3 k from block staircase with left-aligned row types."""
    achievable = set()
    for w1 in range(n + 1):
        for w2 in range(n + 1):
            for w3 in range(n + 1):
                # Only distinct triples matter for reducing search
                wlist = sorted([w1, w2, w3])
                for a in range(n + 1):
                    for b in range(n + 1 - a):
                        c = n - a - b
                        k = a * w1 + b * w2 + c * w3
                        if k <= 0 or k >= n * n:
                            continue

                        # Compute complexity directly
                        # Columns in block arrangement depend on sorted weights
                        # Actually need to track actual patterns

                        # Row patterns
                        patterns = set()
                        if a > 0: patterns.add(w1)  # stand-in for (1^w1, 0^{n-w1})
                        if b > 0: patterns.add(w2)
                        if c > 0: patterns.add(w3)
                        row_count = len(patterns)

                        # Determine column structure
                        # Sort the weight breakpoints
                        w_vals = sorted(set([0, w1, w2, w3, n]))
                        col_patterns = set()
                        for idx in range(len(w_vals) - 1):
                            lo = w_vals[idx]
                            hi = w_vals[idx + 1]
                            if lo == hi:
                                continue
                            # For j in [lo, hi): determine what each row type has
                            # R_i[j] = 1 iff j < w_i
                            val1 = 1 if lo < w1 else 0
                            val2 = 1 if lo < w2 else 0
                            val3 = 1 if lo < w3 else 0
                            # Column = (val1^a, val2^b, val3^c)
                            col = (val1, a, val2, b, val3, c)  # encoding
                            col_patterns.add(col)

                        # Now check if col patterns are subsets of row patterns
                        # Row patterns: (1^w_i, 0^{n-w_i}) encoded as weight w_i
                        # Col patterns: (x^a, y^b, z^c) where x,y,z ∈ {0,1}
                        # A col pattern matches a row pattern if it IS that row pattern
                        # (x^a, y^b, z^c) with weight x*a + y*b + z*c = weight of some row type
                        # AND the pattern is (1^w, 0^{n-w}) for that weight.
                        # This means: x=1 for first a entries IF a <= w, etc.

                        # Actually, let me just compute exactly.
                        R1 = (1,) * w1 + (0,) * (n - w1) if n > 0 else ()
                        R2 = (1,) * w2 + (0,) * (n - w2) if n > 0 else ()
                        R3 = (1,) * w3 + (0,) * (n - w3) if n > 0 else ()

                        all_patterns = set()
                        if a > 0: all_patterns.add(R1)
                        if b > 0: all_patterns.add(R2)
                        if c > 0: all_patterns.add(R3)

                        rows = [R1] * a + [R2] * b + [R3] * c
                        for j in range(n):
                            col = tuple(rows[i][j] for i in range(n))
                            all_patterns.add(col)

                        if len(all_patterns) <= 3:
                            achievable.add(k)

    achievable.discard(0)
    achievable.discard(n * n)
    return achievable


# Optimized staircase: reduce w1,w2,w3 search by only checking ordered triples
def compute_comp3_block_fast(n):
    """Faster block staircase."""
    achievable = set()
    for w1 in range(n + 1):
        R1 = (1,) * w1 + (0,) * (n - w1)
        for w2 in range(n + 1):
            R2 = (1,) * w2 + (0,) * (n - w2)
            for w3 in range(n + 1):
                R3 = (1,) * w3 + (0,) * (n - w3)

                # Pre-compute column patterns for all possible (a,b,c) block arrangements
                # Column j in block (a, b, c):
                # If j < min(w1,w2,w3): all 1s.
                # Segments between sorted weight values give different patterns.

                # For each (a,b,c):
                for a in range(n + 1):
                    for b in range(n + 1 - a):
                        c = n - a - b
                        k = a * w1 + b * w2 + c * w3
                        if k <= 0 or k >= n * n:
                            continue

                        patterns = set()
                        if a > 0: patterns.add(R1)
                        if b > 0: patterns.add(R2)
                        if c > 0: patterns.add(R3)

                        # Column types: determined by (R1[j], R2[j], R3[j])
                        # Group columns by their triple
                        triples_seen = set()
                        for j in range(n):
                            t = (R1[j], R2[j], R3[j])
                            if t not in triples_seen:
                                triples_seen.add(t)
                                # Column pattern
                                col = (t[0],) * a + (t[1],) * b + (t[2],) * c
                                patterns.add(col)

                        if len(patterns) <= 3:
                            achievable.add(k)

    achievable.discard(0)
    achievable.discard(n * n)
    return achievable


# Test analytic constructions
print("=== Analytic constructions ===")
for n in range(2, 30):
    S2 = compute_comp2_set(n)
    S3a = compute_comp3_analytic(n)
    S_all = S2 | S3a
    N_all = n * n - 1
    N4 = N_all - len(S_all)
    C = 3 * n * n - 1 - len(S2) + N4
    print(f"n={n:2d}: N2={len(S2):3d}, N3a={len(S3a-S2):3d}, total={len(S_all):4d}/{N_all:4d}, N4={N4:3d}, C={C:5d}")

print()
print("Expected: C(2)=8, C(5)=64, C(10)=274, C(20)=1150")
for n in [2, 5, 10, 20]:
    S2 = compute_comp2_set(n)
    S3a = compute_comp3_analytic(n)
    S_all = S2 | S3a
    N4 = n * n - 1 - len(S_all)
    C = 3 * n * n - 1 - len(S2) + N4
    print(f"  C({n}) = {C}")

# Also check block staircase for small n
print()
print("=== Adding block staircase (slow, small n only) ===")
for n in range(2, 15):
    S2 = compute_comp2_set(n)
    S3a = compute_comp3_analytic(n)
    S3b = compute_comp3_block_fast(n)
    S_all = S2 | S3a | S3b
    N_all = n * n - 1
    N4 = N_all - len(S_all)
    extra_from_block = len(S3b - S2 - S3a)
    C = 3 * n * n - 1 - len(S2) + N4
    print(f"n={n:2d}: extra_from_block={extra_from_block:3d}, N4={N4:3d}, C={C:5d}")
