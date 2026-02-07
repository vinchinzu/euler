#!/usr/bin/env python3
"""
Smarter approach: Think about c(n,k) structurally.

An n×n binary matrix with r distinct row types and c distinct column types.
The rows are chosen from a set R of r binary strings of length n.
The columns are chosen from a set C of c binary strings of length n.
The complexity is |R ∪ C|.

Key insight: If the matrix has rows from set R (with multiplicities) and columns
from set C (with multiplicities), the number of 1s is determined by the arrangement.

For a matrix where:
- There are r distinct row types, with row type i having multiplicity m_i (sum m_i = n)
- There are c distinct column types, with col type j having multiplicity l_j (sum l_j = n)

The matrix can be thought of as: we choose r row types and partition n rows among them,
and the columns are determined.

Actually, let me think about this differently.

Consider a matrix with entries M[i][j]. The key observation is:

If we have a matrix where we know the set of distinct rows R and the set of
distinct columns C, the complexity is |R ∪ C|.

To minimize complexity for a given k, we want to use as few distinct patterns
as possible.

Let's think about it from the perspective of a matrix built from:
- p distinct "patterns" (binary strings of length n)
- Each row is one of these patterns
- Each column is also one of these patterns

Then complexity ≤ p (could be less if some row patterns equal column patterns).

Actually, complexity = |{row patterns} ∪ {column patterns}|.

So the question is: for each k, what's the minimum number of distinct binary
strings p such that we can build an n×n matrix with exactly k ones, where
every row and every column belongs to a set of p strings?

Hmm, this is tricky. Let me think about specific structures.

BLOCK MATRICES:
If we have a matrix where the first a rows are all 1s in some columns and 0s
elsewhere, the structure is like a block.

Consider a matrix with a block of 1s: rows 1..a, columns 1..b, everything
else 0. Then k = a*b.
Distinct rows: type (1^b, 0^{n-b}) with multiplicity a, type (0^n) with mult n-a -> 2 rows
Distinct cols: type (1^a, 0^{n-a}) with mult b, type (0^n) with mult n-b -> 2 cols
Union: could be 2 or 3 patterns.

Actually, if a=n, then all rows are (1^b, 0^{n-b}) -> 1 row type.
Columns: first b cols are (1^n)=all ones, rest are (0^n)=all zeros -> 2 col types.
But the row pattern (1^b, 0^{n-b}) is length n, while col patterns (1^n) and (0^n) are also length n.
So row pattern ≠ col patterns (unless b=n or b=0).
Complexity = 1 + 2 = 3 (if b != 0 and b != n) or 1+1=2 if b=0 or b=n.

Wait, rows have length n and columns have length n, so they're comparable!

For a block with rows 0..a-1 having 1s in cols 0..b-1:
Row types: (1)*b + (0)*(n-b) [appears a times], (0)*n [appears n-a times]
Col types: (1)*a + (0)*(n-a) [appears b times], (0)*n [appears n-b times]

Row type 1: (1,1,...,1, 0,0,...0) with b ones
Row type 2: (0,0,...,0)
Col type 1: (1,1,...,1, 0,0,...0) with a ones
Col type 2: (0,0,...,0)

Union: {row_type_1, (0,...,0), col_type_1}
If a == b: row_type_1 and col_type_1 are the same! (both have same number of 1s, but
wait, they could still be different patterns)

Hmm wait, row_type_1 = (1)*b + (0)*(n-b) and col_type_1 = (1)*a + (0)*(n-a).
These are the same iff a == b.

So for a block a×b:
- If 0 < a < n and 0 < b < n:
  - If a == b: complexity = 2 (the row/col pattern + all-zeros)
  - If a != b: complexity = 3
- If a == 0 or b == 0 or a == n or b == n: simpler cases

For a == b, k = a^2, and complexity = 2.

This means c(n, a^2) ≤ 2 for a = 0, 1, ..., n.
c(n, 0) = 1 (all zeros), c(n, n^2) = 1 (all ones).
For 0 < a < n: c(n, a^2) ≤ 2.

Can we achieve complexity 1? Only if there's just one pattern that serves as both
every row and every column. A matrix where all rows are the same AND all columns
are the same. This means M[i][j] = r[j] = c[i]. So M[i][j] depends only on j
(constant rows) and also depends only on i (constant cols), meaning M is constant.
So complexity 1 iff k=0 or k=n^2.

So c(n,k) = 1 iff k=0 or k=n^2, else c(n,k) ≥ 2.

Now what values of k can achieve c(n,k) = 2?
We need exactly 2 patterns, say P and Q, such that every row ∈ {P,Q} and every col ∈ {P,Q}.

Case 1: All rows are P and all cols are Q (and P∈{rows}, Q∈{cols}).
But if all rows are P, then all rows are identical: M[i][j] = P[j] for all i.
Then column j is (P[j], P[j], ..., P[j]) = constant vector.
If P[j]=0, col j = (0,...,0); if P[j]=1, col j = (1,...,1).
So columns ∈ {(0,...,0), (1,...,1)}.
We need {columns} ⊆ {P, Q}.
The all-zeros column is (0,...,0).
The all-ones column is (1,...,1).
So we need {(0,...,0), (1,...,1)} ⊆ {P, Q}.
Also, rows = {P}, and P is some binary vector.
Then Q is the other one of {(0,...,0), (1,...,1)}.

Sub-case: P = (0,...,0). Then M is all zeros, k=0, complexity 1.
Sub-case: P = (1,...,1). Then M is all ones, k=n^2, complexity 1.
Sub-case: P is neither all-0 nor all-1. Then columns are {(0,...,0), (1,...,1)},
and {P, Q} = {(0,...,0), (1,...,1)} means P ∈ {(0,...,0), (1,...,1)}. Contradiction.

So all-rows-same doesn't work unless trivial.

Case 2: Some rows are P and some are Q. Say a rows are P and n-a rows are Q.
Then column j = (P[j],...,P[j], Q[j],...,Q[j]) with a copies of P[j] and n-a of Q[j].
For columns to be in {P, Q}:
For each j, the column j (length n) must be either P or Q.
Column j has first a entries = P[j] and last n-a entries = Q[j].

So column j = (P[j]^a, Q[j]^{n-a}) where x^m means x repeated m times.

For this to equal P: P[j]^a must equal (P[0],...,P[a-1]) and Q[j]^{n-a} must equal (P[a],...,P[n-1]).
But P[j]^a = (P[j],P[j],...) is a constant vector of length a.
And (P[0],...,P[a-1]) is the first a entries of P.
So we need all of P[0],...,P[a-1] to be equal (= P[j]), and all of P[a],...,P[n-1] to be equal (= Q[j]).

This is getting complex. Let me think more structurally.

We have a rows of type P and (n-a) rows of type Q (where 1 ≤ a ≤ n-1).
WLOG we can permute columns, but let me think of P and Q as binary vectors.

Column j = (P[j] repeated a times, Q[j] repeated n-a times).
For columns ⊆ {P, Q}:

The possible columns are:
- If P[j]=0, Q[j]=0: column = (0^a, 0^{n-a}) = all zeros
- If P[j]=0, Q[j]=1: column = (0^a, 1^{n-a})
- If P[j]=1, Q[j]=0: column = (1^a, 0^{n-a})
- If P[j]=1, Q[j]=1: column = (1^a, 1^{n-a}) = all ones

And each column must be either P or Q.

P is a vector of length n. It starts with a entries that come from different j values,
then n-a entries. Wait, I'm confusing the structure.

Actually, P and Q are length-n vectors. The matrix has rows indexed 0..n-1.
Rows 0..a-1 are copies of P, rows a..n-1 are copies of Q.

Column j is the vector (M[0][j], M[1][j], ..., M[n-1][j]) = (P[j], ..., P[j], Q[j], ..., Q[j])
with P[j] in positions 0..a-1 and Q[j] in positions a..n-1.

For column j to be in {P, Q}, it must be either P or Q.

If column j = P: then P[i] = P[j] for i=0..a-1 and P[i] = Q[j] for i=a..n-1.
This means the first a entries of P are all P[j], and the last n-a entries of P are all Q[j].
So P = (P[j]^a, Q[j]^{n-a}).

If column j = Q: then Q[i] = P[j] for i=0..a-1 and Q[i] = Q[j] for i=a..n-1.
So Q = (P[j]^a, Q[j]^{n-a}).

Since P and Q are fixed vectors, let's say P = (p^a, q^{n-a}) and Q = (p'^a, q'^{n-a})
where p, q, p', q' ∈ {0,1}.

Since P ≠ Q (otherwise complexity 1), we need (p,q) ≠ (p',q').

Now for each j:
- P[j] ∈ {p, p'} (since P[j] is the value in position j)

Wait, I realize P = (p^a, q^{n-a}) means:
- P[j] = p for j = 0..a-1
- P[j] = q for j = a..n-1

And Q = (p'^a, q'^{n-a}).

Column j:
- For j = 0..a-1: column j = (P[j]^a, Q[j]^{n-a}) = (p^a, p'^{n-a})
  This must equal P or Q.
  P = (p^a, q^{n-a}), Q = (p'^a, q'^{n-a}).
  (p^a, p'^{n-a}) = P iff p'= q.
  (p^a, p'^{n-a}) = Q iff p = p' and p' = q', i.e., p = p' = q'.

- For j = a..n-1: column j = (q^a, q'^{n-a})
  This must equal P or Q.
  (q^a, q'^{n-a}) = P iff q = p and q' = q.
  (q^a, q'^{n-a}) = Q iff q = p' and q' = q'.

This is getting complicated. Let me just enumerate the possible (p,q,p',q') with (p,q) ≠ (p',q').

For each combo, check if columns are valid:

For columns in {0..a-1}: need (p^a, p'^{n-a}) ∈ {P, Q} = {(p^a, q^{n-a}), (p'^a, q'^{n-a})}
- Match P: need p' = q (the last n-a entries match)
- Match Q: need p = p' and p' = q'

For columns in {a..n-1}: need (q^a, q'^{n-a}) ∈ {P, Q}
- Match P: need q = p and q' = q
- Match Q: need q = p' and q' = q'

Let me enumerate all 4 choose 2 = 12 cases (actually there are 4*4-4=12 since (p,q)≠(p',q')):
"""

def check(p, q, pp, qp, a, n):
    """Check if a matrix with row types P=(p^a, q^{n-a}) and Q=(pp^a, qp^{n-a}) is valid."""
    # Column j in 0..a-1 is (p^a, pp^{n-a})
    # Column j in a..n-1 is (q^a, qp^{n-a})
    P = (p,)*a + (q,)*(n-a)
    Q = (pp,)*a + (qp,)*(n-a)

    col_first = (p,)*a + (pp,)*(n-a)
    col_second = (q,)*a + (qp,)*(n-a)

    valid = col_first in (P, Q) and col_second in (P, Q)
    if valid and P != Q:
        # Count ones: a rows of P and (n-a) rows of Q
        k = a * (p*a + q*(n-a)) + (n-a) * (pp*a + qp*(n-a))
        patterns = {P, Q, col_first, col_second}  # some may overlap
        complexity = len(patterns)
        return True, k, complexity
    return False, 0, 0

# For general n and various a values, find valid configurations with complexity 2
print("=== Complexity 2 analysis ===")
for n in range(2, 8):
    k_values_comp2 = set()
    for a in range(1, n):
        for p in range(2):
            for q in range(2):
                for pp in range(2):
                    for qp in range(2):
                        if (p,q) == (pp,qp):
                            continue
                        ok, k, comp = check(p, q, pp, qp, a, n)
                        if ok and comp == 2:
                            k_values_comp2.add(k)
    # Also k=0 (comp 1) and k=n^2 (comp 1) have even lower complexity
    # But for the "achievable with 2" set:
    print(f"n={n}: complexity 2 achievable for k in {sorted(k_values_comp2)}")

# Let me also look at what k values need complexity >= 3
print("\n=== Full c(n,k) comparison ===")
for n in range(2, 5):
    # Brute force c(n,k)
    cnk = {}
    total = n * n
    for bits in range(2 ** total):
        k = bin(bits).count('1')
        matrix = []
        for i in range(n):
            row = []
            for j in range(n):
                row.append((bits >> (i * n + j)) & 1)
            matrix.append(row)

        patterns = set()
        for row in matrix:
            patterns.add(tuple(row))
        for j in range(n):
            col = tuple(matrix[i][j] for i in range(n))
            patterns.add(col)
        c = len(patterns)

        if k not in cnk or c < cnk[k]:
            cnk[k] = c

    comp2_set = set()
    for a in range(1, n):
        for p in range(2):
            for q in range(2):
                for pp in range(2):
                    for qp in range(2):
                        if (p,q) == (pp,qp):
                            continue
                        ok, k, comp = check(p, q, pp, qp, a, n)
                        if ok and comp <= 2:
                            comp2_set.add(k)
    comp2_set.add(0)
    comp2_set.add(n*n)

    print(f"\nn={n}:")
    for k in range(n*n+1):
        mark = "✓" if k in comp2_set else "✗"
        print(f"  c({n},{k}) = {cnk[k]}  comp2:{mark}")
