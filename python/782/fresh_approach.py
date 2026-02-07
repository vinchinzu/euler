#!/usr/bin/env python3
"""
Fresh approach: compute C(n) correctly for small n using the verified method,
and find a formula that matches ALL given test values.

VERIFIED FACTS:
- c(n,k) = 1 for k ∈ {0, n^2}
- c(n,k) = 2 for k in S (our formula set, verified for n ≤ 10)
- For n ≤ 4: c(n,k) ∈ {1,2,3} (full brute force)
- For n = 5: c(n,k) ∈ {1,2,3} (3-row-type enumeration)
- C(2) = 8 ✓, C(5) = 64 ✓, C(10) = 274 ✓
- C(20) = 1144 with my formula (should be 1150)

The discrepancy at n=20 suggests either:
(a) Some k values I claim as c=2 are actually c≥3 (overcounting S), or
(b) Some k values have c > 3

Since (a) has been ruled out by direct matrix construction for n ≤ 10
and the formula matches through n=10, maybe (b) is the issue but only
for n > 10.

Alternative hypothesis: maybe my formula for S is incomplete.
There could be additional complexity-2 configurations I haven't considered.

Wait, let me reconsider. Maybe there are complexity-2 matrices that DON'T
use the P/Q/σ structure I analyzed. Could there be a matrix with comp=2
where the rows are NOT divided into exactly 2 types?

If comp = 2, there are exactly 2 patterns. Each row is one of them, and
each column is one of them. So the rows ARE divided into exactly 2 types
(or all one type). I covered the all-one-type case (comp=1 for k=0,n^2,
and comp ≤ 3 for other all-same-row matrices).

So my analysis IS complete for comp=2. The S set is correct.

Therefore, C(20) with my formula IS 1144, and either:
(a) The problem gives C(20) = 1150 and I'm wrong somewhere, or
(b) My formula is correct and the problem value is different from what I extracted.

Let me try yet another interpretation of the problem.

ALTERNATIVE: What if "distinct rows and columns" means the number of distinct
rows PLUS the number of distinct columns (counted separately)?

Let me check with the examples:
Matrix A = [[1,0,1],[0,0,0],[1,0,1]]:
Distinct rows: {(1,0,1), (0,0,0)} -> 2
Distinct cols: {(1,0,1), (0,0,0)} -> 2
Sum: 4.
But problem says complexity = 2.

Matrix B = [[0,0,0],[0,0,0],[1,1,1]]:
Distinct rows: {(0,0,0), (1,1,1)} -> 2
Distinct cols: {(0,0,1)} -> 1
Sum: 3.
Problem says complexity = 3.

For matrix A, sum = 4 but problem says 2. So "sum" interpretation is wrong.
"Union" interpretation: |{(1,0,1),(0,0,0)}| = 2 ✓ for A, |{(0,0,0),(1,1,1),(0,0,1)}| = 3 ✓ for B.

So union is correct. My formula should be right.

But wait: what if the problem means "distinct rows" = rows that are unique
(appear only once), not "number of distinct row patterns"?

For matrix A: rows are (1,0,1), (0,0,0), (1,0,1). Row (0,0,0) appears once (unique).
Row (1,0,1) appears twice (not unique). So "distinct rows" = 1 (only the middle row).
Cols: col0=(1,0,1), col1=(0,0,0), col2=(1,0,1). Col (0,0,0) unique. (1,0,1) appears twice.
"Distinct columns" = 1.
"distinct rows and columns" = 1+1 = 2. Matches!

For matrix B: rows are (0,0,0), (0,0,0), (1,1,1). (0,0,0) appears twice, (1,1,1) once.
Unique rows: 1.
Cols: all (0,0,1). Appears 3 times. Unique cols: 0.
Total: 1+0 = 1. Problem says 3. DOESN'T MATCH!

OK that interpretation is wrong too.

Let me try yet another: "distinct rows and columns" = # of distinct MULTISETS
of (row, column) pairs? No, that doesn't make sense.

What about: complexity = |distinct rows| + |distinct columns| where |distinct X|
means the number of different X patterns?

Matrix A: 2 distinct row patterns + 2 distinct col patterns = 4. Problem says 2. Wrong.

What about: complexity = max(|distinct rows|, |distinct columns|)?
A: max(2, 2) = 2. B: max(2, 1) = 2. Problem says B has 3. Wrong.

What about: complexity = |distinct rows| * |distinct columns|?
A: 2*2 = 4. Wrong.

I keep coming back to the union interpretation being the only one that works
for both examples. And it gives C(2) = 8, C(5) = 64, C(10) = 274.
But C(20) = 1144 ≠ 1150.

Hmm, wait. Let me re-read the problem statement one more time.
The problem says: "distinct set {000, 101}" for matrix A.
It says: "distinct set {000, 001, 111}" for matrix B.

These ARE the unions! For A: rows give {101, 000}, cols give {101, 000}.
Union = {000, 101}. Size = 2.

For B: rows give {000, 111}, cols give {001}. Union = {000, 001, 111}. Size = 3.

OK so the union interpretation is definitely correct. I must have an error in
my formula for large n.

Let me verify C(n) by a completely independent method for n=6,7,8.
"""

import itertools

def find_comp2_direct(n):
    """Direct enumeration of comp-2 k values for n up to ~8."""
    achievable = set()
    all_vecs = list(itertools.product([0,1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P >= Q:
                continue

            # Determine column constraints
            has_10 = any(P[j]==1 and Q[j]==0 for j in range(n))
            has_01 = any(P[j]==0 and Q[j]==1 for j in range(n))

            # Check constant column constraints
            valid = True
            for j in range(n):
                if P[j] == Q[j] == 0:
                    if (0,)*n != P and (0,)*n != Q:
                        valid = False
                        break
                if P[j] == Q[j] == 1:
                    if (1,)*n != P and (1,)*n != Q:
                        valid = False
                        break
            if not valid:
                continue

            # Determine valid σ values
            possible_sigmas = set()
            if has_10 and has_01:
                comp_P = tuple(1-x for x in P)
                if comp_P == Q:
                    possible_sigmas.add(P)
                    possible_sigmas.add(Q)
            elif has_10:
                possible_sigmas.add(P)
                possible_sigmas.add(Q)
            elif has_01:
                possible_sigmas.add(tuple(1-x for x in P))
                possible_sigmas.add(tuple(1-x for x in Q))
            else:
                continue

            for sigma in possible_sigmas:
                s = sum(sigma)
                k = s * sum(P) + (n - s) * sum(Q)

                # Verify
                matrix = [P if sigma[i] else Q for i in range(n)]
                patterns = set(matrix)
                for j in range(n):
                    patterns.add(tuple(matrix[i][j] for i in range(n)))
                if len(patterns) <= 2 and 0 < k < n*n:
                    achievable.add(k)

    return achievable


def compute_comp2_formula(n):
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


# Verify for n = 6, 7, 8, 9, 10
for nn in range(6, 11):
    direct = find_comp2_direct(nn)
    formula = compute_comp2_formula(nn)
    Cn = 2*nn*nn + (nn*nn - 1 - len(formula))
    print(f"n={nn}: direct={len(direct)}, formula={len(formula)}, match={direct==formula}, C={Cn}")


# Now let me try to check whether c(n,k) ≤ 3 for n=6 by 3-type enumeration.
# For n=6: 64^3/6 * 3^6 ≈ too many. Let me use a smarter approach.

# For n=6, compute c(n,k) exactly using 2-type approach with all possible σ.
def compute_cnk_2types(n):
    """For each k, find min complexity using 2 row types with ALL σ."""
    best = {}
    all_vecs = list(itertools.product([0,1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P > Q:
                continue
            for sigma in all_vecs:
                patterns = set()
                if 1 in sigma:
                    patterns.add(P)
                if 0 in sigma:
                    patterns.add(Q)
                k = 0
                for j in range(n):
                    col = tuple(P[j] if sigma[i] else Q[j] for i in range(n))
                    patterns.add(col)
                    k += sum(col)
                comp = len(patterns)
                if k not in best or comp < best[k]:
                    best[k] = comp
    return best


print("\nExact c(n,k) using 2 row types:")
for nn in [5, 6]:
    best = compute_cnk_2types(nn)
    C = sum(best.get(k, 999) for k in range(nn*nn+1))
    max_c = max(best.get(k, 0) for k in range(nn*nn+1))
    gt3 = [k for k in range(nn*nn+1) if best.get(k, 999) > 3]
    print(f"n={nn}: C_2type={C}, max_c={max_c}, k_gt3={gt3}")
    # For k values that still have comp>3 with 2 types, need 3+ types.
    # But brute force for n=5 showed C(5)=64, and 2-type gives 68.
    # So 3-type brings some k values from 4 down to 3.
