#!/usr/bin/env python3
"""
Characterize k values achievable with complexity <= 3.

For complexity <= 3, we have at most 3 patterns in the union of rows and columns.

CASE 1: 1 row type (all rows identical)
  Row = R (weight w). k = n*w.
  Columns: each column is constant (all same value).
  Col j = (R[j], R[j], ..., R[j]).
  If R[j]=0: col = 0^n.
  If R[j]=1: col = 1^n.
  Patterns: {R, 0^n (if w<n), 1^n (if w>0)}.
  If 0 < w < n: comp = 3 (R, 0^n, 1^n all distinct).
  If w = 0: comp = 1. If w = n: comp = 1.
  So k = n*w for w=1..n-1 has comp = 3 (or 2 if in S2).

CASE 2: 2 row types, comp <= 2
  Already covered by S2 formula.

CASE 3: 2 row types, comp = 3
  Union = {P, Q} ∪ {column patterns} has exactly 3 elements.
  So exactly 1 column pattern is new (not P or Q).

  Column patterns: {0^n if a>0, 1^n if b>0, σ if c>0, comp(σ) if d>0}
  where a,b,c,d are position type counts and a+b+c+d = n.

  For comp = 3: exactly 1 of {0^n, 1^n, σ, comp(σ)} is new and the rest are in {P,Q}.

CASE 4: 3 row types, comp <= 3
  3 distinct row types {A, B, C}, and all column patterns ∈ {A, B, C}.
  σ ∈ {0,1,2}^n assigns each row to a type.
  Col j = (types[σ[i]][j] for i in range(n)).
  Need each column to be one of {A, B, C}.

  This is the KEY case for achieving comp = 3 where 2 types give comp >= 4.

Let me enumerate comp-3 k values achievable with 3 row types and columns all in the row set.

For 3 row types A, B, C in block arrangement (a rows of A, b rows of B, c rows of C):
Column j depends only on (A[j], B[j], C[j]):
  (0,0,0) -> col = 0^a+b+c = 0^n
  (0,0,1) -> col = (0^{a+b}, 1^c)
  (0,1,0) -> col = (0^a, 1^b, 0^c)
  (0,1,1) -> col = (0^a, 1^{b+c})
  (1,0,0) -> col = (1^a, 0^{b+c})
  (1,0,1) -> col = (1^a, 0^b, 1^c)
  (1,1,0) -> col = (1^{a+b}, 0^c)
  (1,1,1) -> col = 1^n

For comp <= 3 with 3 row types and block arrangement:
Union of {row types} ∪ {col types} must have <= 3 elements.
Row types A, B, C must each be one of the 3 patterns.
Col types must each be one of the 3 patterns.

Since row types ARE the 3 patterns, all column types must also be in {A, B, C}.

If we parameterize A, B, C as left-aligned:
A = (1^w_A, 0^{n-w_A}), etc.
Then columns are determined by w_A, w_B, w_C and a, b, c.

Let me enumerate. For each j:
  If j < min(w_A, w_B, w_C): (1,1,1) -> 1^n. This must be A, B, or C.
  If j >= max(w_A, w_B, w_C): (0,0,0) -> 0^n. Must be A, B, or C.
  Otherwise: mixed, depends on specific w values.

For the column to be A = (1^a, 0^{b+c}):
We need col j = (1^a, 0^b, 0^c) iff (A[j]=1, B[j]=0, C[j]=0).
This means j < w_A and j >= w_B and j >= w_C.
So w_A > w_B and w_A > w_C, and j ∈ [max(w_B, w_C), w_A).

For col = B = (0^a, 1^b, 0^c):
Need A[j]=0, B[j]=1, C[j]=0. So j >= w_A, j < w_B, j >= w_C.

For col = C = (0^a, 0^b, 1^c):
Need A[j]=0, B[j]=0, C[j]=1. So j >= w_A, j >= w_B, j < w_C.

For col = 1^n = (1^{a+b+c}):
Need A[j]=B[j]=C[j]=1. So j < w_A, j < w_B, j < w_C.
This is 1^n. Needs to be one of {A,B,C}.
1^n = A iff w_A = n. 1^n = B iff w_B = n (but then B = 1^n = (0^a, 1^{b+c}) => a=0).
Actually with block arrangement, B = (0^a, 1^b, 0^c) only if B is the actual vector assigned to the b middle rows.

Wait, I'm overcomplicating this. Let me just be general: A, B, C are ANY binary vectors.

For block arrangement with a rows of A, b of B, c of C:
Col j = (A[j])^a + (B[j])^b + (C[j])^c (concatenation of repeated values).

For col j ∈ {A, B, C}: the column must match one of A, B, or C.

This is a strong constraint. Let me think about it position by position.

Col j has the form: (A[j] repeated a times, B[j] repeated b times, C[j] repeated c times).
This is a vector of length n = a+b+c.

For this to equal A: we need A[i] = A[j] for i < a, A[i] = B[j] for a <= i < a+b, A[i] = C[j] for a+b <= i < n.
But A[i] varies with i! So A's first a entries are A[0]..A[a-1], next b are A[a]..A[a+b-1], etc.

Hmm, this only works if A is "block-structured" itself, meaning A has constant value in each of the 3 blocks.

A has value v1 in positions 0..a-1, v2 in positions a..a+b-1, v3 in positions a+b..n-1.
So A = (v1^a, v2^b, v3^c) where v1, v2, v3 ∈ {0, 1}.

Similarly B = (u1^a, u2^b, u3^c) and C = (t1^a, t2^b, t3^c).

Then col j depends on which "block" j falls into.

If j ∈ [0, a): col = (v1^a, u1^b, t1^c). This must equal A, B, or C.
  = A: (v1^a, u1^b, t1^c) = (v1^a, v2^b, v3^c) => u1 = v2, t1 = v3.
  = B: u1 = u2, t1 = u3.
  = C: u1 = t2, t1 = t3.

If j ∈ [a, a+b): col = (v2^a, u2^b, t2^c). Must equal A, B, or C.
If j ∈ [a+b, n): col = (v3^a, u3^b, t3^c). Must equal A, B, or C.

So we have 3 column types, each must be in {A, B, C}.
A = (v1, v2, v3) [in block notation], B = (u1, u2, u3), C = (t1, t2, t3).

Col type 1: (v1, u1, t1)
Col type 2: (v2, u2, t2)
Col type 3: (v3, u3, t3)

These must each be one of {(v1,v2,v3), (u1,u2,u3), (t1,t2,t3)}.

In other words: if M is the 3x3 matrix:
M = [[v1, v2, v3],
     [u1, u2, u3],
     [t1, t2, t3]]

then the COLUMN vectors of M must each be a ROW vector of M.

This is a 3x3 binary matrix whose column set ⊆ row set.

Let me enumerate all such 3x3 binary matrices.
"""

import itertools


def enumerate_3x3_selfcontained():
    """Find all 3x3 binary matrices where each column is also a row."""
    results = []
    for M_flat in itertools.product([0, 1], repeat=9):
        M = [M_flat[0:3], M_flat[3:6], M_flat[6:9]]
        rows = set(M)
        cols = set()
        for j in range(3):
            col = (M[0][j], M[1][j], M[2][j])
            cols.add(col)
        if cols <= rows:
            results.append(M)
    return results


matrices = enumerate_3x3_selfcontained()
print(f"Number of 3x3 binary matrices where cols ⊆ rows: {len(matrices)}")

# For each such matrix, and for each (a, b, c) with a+b+c = n,
# we can build an n x n matrix with complexity <= 3.
# k = a*(a*v1 + b*v2 + c*v3) [from rows of type A]
#   + b*(a*u1 + b*u2 + c*u3) [from rows of type B]
#   + c*(a*t1 + b*t2 + c*t3) [from rows of type C]
# = a^2*v1 + ab*v2 + ac*v3 + ab*u1 + b^2*u2 + bc*u3 + ac*t1 + bc*t2 + c^2*t3

# Actually: k = sum over all entries = sum_rows(weight of each row * multiplicity)
# = a*(v1*a + v2*b + v3*c) + b*(u1*a + u2*b + u3*c) + c*(t1*a + t2*b + t3*c)
# Wait, that's not right either. Row type A = (v1^a, v2^b, v3^c) has weight v1*a + v2*b + v3*c.
# There are a copies. Similarly for B and C.
# k = a*(v1*a + v2*b + v3*c) + b*(u1*a + u2*b + u3*c) + c*(t1*a + t2*b + t3*c)

# Let me print the distinct matrices and the k formula
unique = set()
for M in matrices:
    key = tuple(tuple(r) for r in M)
    unique.add(key)

print(f"Distinct matrices: {len(unique)}")

# Group by the set of rows
by_rowset = {}
for key in unique:
    rowset = frozenset(key)
    if rowset not in by_rowset:
        by_rowset[rowset] = []
    by_rowset[rowset].append(key)

print(f"\nGrouped by row set ({len(by_rowset)} groups):")
for rowset, mats in sorted(by_rowset.items(), key=lambda x: len(x[1]), reverse=True):
    print(f"  Rows {set(rowset)}: {len(mats)} matrices")


# Now compute achievable k values for given n
def compute_comp3_block_k_values(n):
    """All k values achievable with comp <= 3 using block 3-type construction."""
    achievable = set()

    for key in unique:
        M = list(key)
        v1, v2, v3 = M[0]
        u1, u2, u3 = M[1]
        t1, t2, t3 = M[2]

        for a in range(n + 1):
            for b in range(n + 1 - a):
                c = n - a - b

                # Check if all 3 row types are actually needed
                # (otherwise, complexity might be < 3 but we should still count)
                k = (a * (v1*a + v2*b + v3*c) +
                     b * (u1*a + u2*b + u3*c) +
                     c * (t1*a + t2*b + t3*c))

                # Compute actual complexity
                A_row = (v1,)*a + (v2,)*b + (v3,)*c
                B_row = (u1,)*a + (u2,)*b + (u3,)*c
                C_row = (t1,)*a + (t2,)*b + (t3,)*c

                patterns = set()
                if a > 0: patterns.add(A_row)
                if b > 0: patterns.add(B_row)
                if c > 0: patterns.add(C_row)

                # Columns
                rows_list = [A_row]*a + [B_row]*b + [C_row]*c
                for j in range(n):
                    col = tuple(rows_list[i][j] for i in range(n))
                    patterns.add(col)

                if len(patterns) <= 3 and 0 < k < n*n:
                    achievable.add(k)

    return achievable


# Verify for small n
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


print("\n=== Achievable k values with block 3-type construction ===")
for n in range(2, 25):
    S2 = compute_comp2_set(n)
    S3_block = compute_comp3_block_k_values(n)
    S_combined = S2 | S3_block
    N_all = n*n - 1
    N_achieved = len(S_combined)
    N4 = N_all - N_achieved
    C_max3 = 3*n*n - 1 - len(S2)
    C_actual = C_max3 + N4
    print(f"n={n:2d}: N2={len(S2):3d}, S3_block={len(S3_block):4d}, combined={N_achieved:4d}/{N_all:4d}, N4={N4:3d}, C={C_actual:5d}")
