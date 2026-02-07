#!/usr/bin/env python3
"""
Enumerate comp-3 k values using triple-based approach.

For 3 row types in block arrangement (a rows of P, b of Q, c of R):
Each position j has a triple t_j = (P[j], Q[j], R[j]) ∈ {0,1}^3.
Let n_t = count of positions with triple t.

Column pattern for triple t = (p,q,r) is: (p^a, q^b, r^c) (a copies of p, b of q, c of r).
This vector has length n = a+b+c.

Row patterns:
P = vector where P[j] = t_j[0] for all j.
Q = vector where Q[j] = t_j[1] for all j.
R = vector where R[j] = t_j[2] for all j.

Weights:
wP = sum(n_t * t[0] for t in T) = n_{1**}
wQ = sum(n_t * t[1] for t in T) = n_{*1*}
wR = sum(n_t * t[2] for t in T) = n_{**1}

k = a*wP + b*wQ + c*wR

Column pattern for triple t = (p^a, q^b, r^c) where a+b+c = n.
Row P = (t_j[0] for j): P is a vector where the value depends on the triple at each position.

For comp <= 3: |{row patterns used} ∪ {column patterns}| <= 3.
Row patterns used: subset of {P, Q, R} depending on which of a,b,c > 0.
Column patterns: {col(t) : t ∈ T, n_t > 0}.

The key constraint is that BOTH the row patterns AND column patterns must come from
the same set of at most 3 vectors.

Now, col(t) = (t[0]^a, t[1]^b, t[2]^c).
P = (t_0[0], t_1[0], ..., t_{n-1}[0]).

For col(t) to equal P: (t[0]^a, t[1]^b, t[2]^c) must equal P.
P has first a entries = {t_j[0] for j in first a positions}.
But the first a positions are arbitrary (they depend on how positions are arranged).

Wait, NO. The ROWS are in block arrangement (first a rows use P, etc.).
The COLUMNS (positions) can be in any order. The column at position j:
col_j[i] = types[σ[i]][j]. With σ = (0^a, 1^b, 2^c) (block):
col_j = (P[j], ..., P[j], Q[j], ..., Q[j], R[j], ..., R[j])
       = (P[j]^a, Q[j]^b, R[j]^c)
       = (t_j[0]^a, t_j[1]^b, t_j[2]^c)

For this to equal P = (t_0[0], t_1[0], ..., t_{n-1}[0]):
(t_j[0]^a, t_j[1]^b, t_j[2]^c) = (t_0[0], t_1[0], ..., t_{n-1}[0])

The LHS is block-constant:
First a entries = t_j[0] (all same).
Next b entries = t_j[1].
Last c entries = t_j[2].

The RHS = P: entries are t_i[0] for i = 0,...,n-1.
For the first a entries: P[0]=t_0[0], P[1]=t_1[0], ..., P[a-1]=t_{a-1}[0].
For these to all equal t_j[0]: all positions 0..a-1 must have their first triple component = t_j[0].

This is only possible if we ARRANGE positions so that:
- Positions 0..a-1 have first component = t_j[0]
- Positions a..a+b-1 have first component = t_j[1]
- Positions a+b..n-1 have first component = t_j[2]

This means: the positions are arranged so that their first triple component matches
the block structure. In other words, the column arrangement mirrors the row arrangement
for the first component.

Hmm, this is very restrictive. We need positions arranged so that P is block-constant.
Similarly for Q and R to be block-constant.

P is block-constant iff all triples in positions 0..a-1 have the same first component,
all in positions a..a+b-1 have the same first component (possibly different from block 1),
and all in a+b..n-1 have the same first component.

This is the 3x3 matrix approach I already did!

But the ACTUAL SOLUTION found for n=8, k=23 uses non-block-constant P.
P = (0,0,0,0,0,0,0,1). With a=4: P's first 4 entries are all 0. P is (0,0,0,0,0,0,0,1).
P IS block-constant on [0,4) (all 0), but NOT on [4,7) or [7,8).

Actually wait: with a=4, b=3, c=1:
Block 0 (rows 0-3): use P
Block 1 (rows 4-6): use Q
Block 2 (row 7): use R

The COLUMN patterns at position j depend on (P[j], Q[j], R[j]).
P = (0,0,0,0,0,0,0,1)
Q = (0,0,0,0,1,1,1,1)
R = (1,1,1,1,1,1,1,0)

Triples at each position:
j=0: (0,0,1)
j=1: (0,0,1)
j=2: (0,0,1)
j=3: (0,0,1)
j=4: (0,1,1)
j=5: (0,1,1)
j=6: (0,1,1)
j=7: (1,1,0)

Column patterns:
j=0: (0^4, 0^3, 1^1) = (0,0,0,0,0,0,0,1) = P!
j=4: (0^4, 1^3, 1^1) = (0,0,0,0,1,1,1,1) = Q!
j=7: (1^4, 1^3, 0^1) = (1,1,1,1,1,1,1,0) = R!

So P, Q, R are NOT block-constant (P[7]=1 while P[0..6]=0).
But the column patterns still match {P, Q, R}.

This works because:
- Triple (0,0,1) gives column (0^4, 0^3, 1^1) = P
- Triple (0,1,1) gives column (0^4, 1^3, 1^1) = Q
- Triple (1,1,0) gives column (1^4, 1^3, 0^1) = R

So column(t) = P when t = (0,0,1), column(t) = Q when t = (0,1,1), column(t) = R when t = (1,1,0).

The ROW patterns: P has entries (0,0,0,0,0,0,0,1), which are NOT block-constant with respect to the column-block structure. BUT the constraint is that the column patterns match the rows, which they do because each column pattern (0^4,0^3,1^1), (0^4,1^3,1^1), (1^4,1^3,0^1) happens to equal P, Q, R respectively.

So the 3x3 matrix approach works ONLY when rows are block-constant. For non-block-constant rows, we need a more general analysis.

The general constraint is:
For each triple t occurring in the position set,
the column pattern (t[0]^a, t[1]^b, t[2]^c) must be one of P, Q, R.

P is determined by the position arrangement: P[j] = t_j[0].
Q[j] = t_j[1], R[j] = t_j[2].

So the constraint is:
For each t occurring, (t[0]^a, t[1]^b, t[2]^c) must equal one of:
P = (t_0[0], t_1[0], ..., t_{n-1}[0])
Q = (t_0[1], t_1[1], ..., t_{n-1}[1])
R = (t_0[2], t_1[2], ..., t_{n-1}[2])

This means (t[0]^a, t[1]^b, t[2]^c) must have:
- Its first a entries: all = t[0]. These correspond to positions 0..a-1.
  If it equals P: P[i] = t[0] for i = 0..a-1. So t_i[0] = t[0] for i in [0,a).
  If it equals Q: Q[i] = t[0] for i = 0..a-1. So t_i[1] = t[0] for i in [0,a).
  If it equals R: R[i] = t[0] for i = 0..a-1. So t_i[2] = t[0] for i in [0,a).

- Its next b entries: all = t[1]. Correspond to positions a..a+b-1.
  Similar constraints.

- Its last c entries: all = t[2]. Correspond to positions a+b..n-1.
  Similar constraints.

So the constraint is that for each triple t in the set:
column(t) = P, Q, or R, which means that within each of the 3 "column blocks"
(positions [0,a), [a,a+b), [a+b,n)), the triples must satisfy specific constraints.

Let me define:
block_0 = positions 0..a-1
block_1 = positions a..a+b-1
block_2 = positions a+b..n-1

For triple t at position j:
If j ∈ block_0 and column(t) = P: t_i[0] = t[0] for all i in block_0.
If j ∈ block_1 and column(t) = P: t_i[0] = t[1] for all i in block_1.
If j ∈ block_2 and column(t) = P: t_i[0] = t[2] for all i in block_2.

This means: within each column block, the first (resp. second, third) component of
all triples must be constant IF any triple in that block maps to P (resp. Q, R).

Actually, the constraint is more nuanced. Each column maps to SOME row type.
Different columns in the same block can map to different row types.

Let me group positions by which row type their column maps to:
- Positions where column(t) = P: these are positions j with triple t such that (t[0]^a, t[1]^b, t[2]^c) = P.
- Similarly for Q and R.

Within block_0 (first a positions):
If col(t) = P: entries in block_0 are t[0]'s, matching P entries which are t_i[0] for i in block_0.
  So t_i[0] = t[0] for all i in block_0. This means: all triples in block_0 that map their column to P must have the same first component. And that component must equal the first component of ALL triples in block_0 (since P[i] = t_i[0] and we need P[i] = t[0] for column positions in block_0).

Hmm wait, let me reconsider. Let me say position j has triple t_j, and column(j) maps to row type f(j) ∈ {P, Q, R}.

column(j) = (t_j[0]^a, t_j[1]^b, t_j[2]^c)
This must equal f(j) ∈ {P, Q, R}.

If f(j) = P: entry i of column j = t_j[σ_i] where σ_i=0 for i<a, 1 for a<=i<a+b, 2 for rest.
So: for i in [0,a): t_j[0] = P[i] = t_i[0].
    for i in [a,a+b): t_j[1] = P[i] = t_i[0].
    for i in [a+b,n): t_j[2] = P[i] = t_i[0].

This means t_j[0] = t_i[0] for all i in block_0 (i.e., all positions in block_0 have the same first component = t_j[0]).
And t_j[1] = t_i[0] for all i in block_1.
And t_j[2] = t_i[0] for all i in block_2.

So: if column j maps to P:
  - All positions in block_0 have first component = t_j[0].
  - All positions in block_1 have first component = t_j[1].
  - All positions in block_2 have first component = t_j[2].

If column j' also maps to P (same or different block):
  Same constraints: t_{j'}[0] = first component of block_0, etc.

So: for ALL columns mapping to P:
  They all have the same t_j[0] (= first component constraint for block_0).
  They all have the same t_j[1] (= first component constraint for block_1).
  They all have the same t_j[2] (= first component constraint for block_2).

So all columns mapping to P have the SAME triple! (t_j[0], t_j[1], t_j[2]) is constant.

Similarly, all columns mapping to Q have the same triple, and all mapping to R have the same triple.

So: we have AT MOST 3 distinct triples (one for each row type they map to).
Plus: the constraints determine that within each block, the first (or second or third) component of all triples is constant.

This is a strong structural constraint. Let me enumerate over it.

Let the 3 triples be t_P, t_Q, t_R (the triples of positions mapping to P, Q, R).
(Some might not exist if no position maps to that type.)

Constraints:
1) All positions in block_0 have first component = t_P[0] (if P-mapping positions exist).
   But also first component = t_Q[0] (if Q-mapping positions exist).
   So t_P[0] = t_Q[0] = t_R[0] (for any types with mapping positions).
   Let α = this common value.

2) All positions in block_1 have first component = t_P[1] = t_Q[1] = t_R[1] = β.

3) All positions in block_2 have first component = t_P[2] = t_Q[2] = t_R[2] = γ.

Wait, that can't be right. Let me re-derive.

If column j maps to P:
  For i in block_0: t_j[0] = t_i[0].
  t_i is the triple at position i, and we need its first component = t_j[0].

If column j' maps to Q:
  For i in block_0: t_{j'}[0] = t_i[1].
  t_i first component is still t_i[0], but we need t_i[1] = t_{j'}[0].
  Wait, no. column j' = (t_{j'}[0]^a, t_{j'}[1]^b, t_{j'}[2]^c).
  This must equal Q = (t_i[1] for each position i).
  For i in block_0: Q[i] = t_i[1]. And column j' entry at row i (i in block_0) = t_{j'}[0].
  So t_{j'}[0] = t_i[1] for all i in block_0.
  This means all positions in block_0 have second component = t_{j'}[0].

So positions in block_0 have first component constrained by P-mapping columns,
second component constrained by Q-mapping columns, and third by R-mapping columns.

Specifically:
- If any column maps to P: all positions in block_0 have first comp = some constant α_0.
  All positions in block_1 have first comp = some constant α_1.
  All positions in block_2 have first comp = some constant α_2.
  (Where α_0, α_1, α_2 are determined by the triple of the P-mapping column.)
  In fact, t_P = (α_0, α_1, α_2) for the unique triple of P-mapping columns.

- If any column maps to Q: all positions in block_k have second comp = β_k.
  t_Q = (β_0, β_1, β_2).

- If any column maps to R: all positions in block_k have third comp = γ_k.
  t_R = (γ_0, γ_1, γ_2).

So the triples at positions in block_0 are all (α_0, β_0, γ_0).
In block_1: all (α_1, β_1, γ_1).
In block_2: all (α_2, β_2, γ_2).

Hmm, but that means within each block all triples are the same!
And the triple in block k is (α_k, β_k, γ_k) = (t_P[k], t_Q[k], t_R[k]).

So the positions ARE organized into 3 groups (column blocks) of sizes a, b, c.
And the triple matrix is:
M = [[t_P[0], t_P[1], t_P[2]],    = [[α_0, α_1, α_2],
     [t_Q[0], t_Q[1], t_Q[2]],      [β_0, β_1, β_2],
     [t_R[0], t_R[1], t_R[2]]]      [γ_0, γ_1, γ_2]]

Wait, I think I'm confusing blocks. Let me clarify:
Row arrangement: first a rows use P, next b use Q, last c use R.
Column arrangement: we're free to arrange columns in any order.

The key insight: with block σ (for rows), the column pattern at position j is
(t_j[0]^a, t_j[1]^b, t_j[2]^c). For this to equal P, Q, or R,
we showed that all columns are divided into at most 3 groups (by which row type they map to),
and within each group all triples are the same.

But the groups are NOT necessarily aligned with the column blocks!
There's no constraint on which positions j have which triple.

The groups are:
G_P = {j : column j maps to P}
G_Q = {j : column j maps to Q}
G_R = {j : column j maps to R}

Within G_P: all have the same triple t_P.
Within G_Q: all have same triple t_Q.
Within G_R: all have same triple t_R.

|G_P| + |G_Q| + |G_R| = n.

The TRIPLES constrain the first/second/third components:
All positions in block_0 (first a column positions): first comp is determined by whether
the position maps to P, Q, or R:
If j ∈ G_P ∩ block_0: the triple at j is t_P. First comp = t_P[0].
If j ∈ G_Q ∩ block_0: triple = t_Q. First comp = t_Q[0].
If j ∈ G_R ∩ block_0: triple = t_R. First comp = t_R[0].

BUT: the "block_0 first a column positions" is just our labeling. We can arrange the
column positions in ANY order. So block_0 is just positions 0..a-1 with their triples.

For column j to map to P: we need column j = P. This is a constraint on ALL n entries of column j.
Since column j = (t_j[0]^a, t_j[1]^b, t_j[2]^c), and P = (P[0], P[1], ..., P[n-1]):
We need P[i] = t_j[0] for i < a, P[i] = t_j[1] for a<=i<a+b, P[i] = t_j[2] for i>=a+b.

Now P[i] = t_i[0] (first component of the triple at position i).
So for column j mapping to P:
  t_i[0] = t_j[0] for i in {0,...,a-1}  (these are the first a positions)
  t_i[0] = t_j[1] for i in {a,...,a+b-1}
  t_i[0] = t_j[2] for i in {a+b,...,n-1}

This means all positions 0..a-1 have first component = t_j[0].
All positions a..a+b-1 have first component = t_j[1].
All positions a+b..n-1 have first component = t_j[2].

But this constrains ALL positions, not just those mapping to P!
So positions 0..a-1 ALL have first component = t_j[0] = α (some constant).

Wait, but j here is a SPECIFIC position in G_P. If j < a, then j is in the first block
and its first component is also constrained. If j is in a different block...

Actually, j can be ANY position (any column index). The constraint from column j mapping to P
affects ALL rows (i) of that column. The rows are indexed differently from columns.

Let me use different notation. Rows are indexed by r = 0..n-1. Columns by c = 0..n-1.
σ maps rows: first a rows (r < a) use type P, etc.
M[r][c] = types[σ[r]][c].

Column c: M[r][c] for r = 0..n-1.
For r < a: M[r][c] = P[c].
For a <= r < a+b: M[r][c] = Q[c].
For a+b <= r < n: M[r][c] = R[c].

So column c = (P[c], ..., P[c], Q[c], ..., Q[c], R[c], ..., R[c]) = (P[c]^a, Q[c]^b, R[c]^c_val).

Row r (for r < a): M[r][c] for c = 0..n-1 = P = (P[0], P[1], ..., P[n-1]).

Column c = (P[c]^a, Q[c]^b, R[c]^c_val) has length n.
Row P = (P[0], P[1], ..., P[n-1]) has length n.

For column c to equal P:
Entry at position r of column c = entry at position r of P.
For r < a: P[c] = P[r].
For a <= r < a+b: Q[c] = P[r].
For a+b <= r: R[c] = P[r].

So:
P[c] = P[r] for all r < a.  (P is constant on {0,...,a-1} ∪ {c}.)
Q[c] = P[r] for all r ∈ {a,...,a+b-1}.  (Q at position c = P at positions a..a+b-1.)
R[c] = P[r] for all r ∈ {a+b,...,n-1}.  (R at position c = P at positions a+b..n-1.)

This is interesting. If P is NOT constant on {0,...,a-1}, then column c = P requires
P[c] = P[r] for all r < a, meaning P[c] equals the common value of P on [0, a). But
P might not BE constant on [0,a). In that case, no column can equal P!

Wait, we're free to permute columns. Actually no, P, Q, R are defined FIRST, then σ
assigns rows. The column indices are the same for both.

Hmm, actually: we are FREE to choose P, Q, R and σ to minimize complexity.
So we should choose them such that the resulting matrix has small complexity.

The critical insight: we can permute COLUMNS (i.e., choose the order of column indices)
without changing complexity (since complexity depends on the SET of rows and columns,
not their order). So WLOG, we can arrange columns in any order.

Similarly, we can permute ROWS. But σ assigns rows to types, and permuting rows changes σ.
However, we can choose σ to be anything, so permuting rows and adjusting σ accordingly is fine.

So: WLOG, we can arrange columns (and rows, via σ) in any order.

For the block σ case: rows are arranged so first a use P, next b use Q, last c use R.
Columns are arranged in any order. Let's arrange columns by the triple (P[c], Q[c], R[c]).

Group columns by triple:
For each t ∈ {0,1}^3, let m_t = number of columns with triple t.
sum(m_t) = n.

Arrange columns so all columns with the same triple are together.
Then column c has triple t_c, and within each triple group, all columns have the same pattern:
column c = (t_c[0]^a, t_c[1]^b, t_c[2]^{n-a-b}).

For comp <= 3: each column pattern must be one of P, Q, R. Since all columns in the same
triple group have the same column pattern, we need at most 3 distinct column patterns,
and each must equal P, Q, or R.

The column pattern for triple t is C_t = (t[0]^a, t[1]^b, t[2]^c_val).
C_t is a vector of length n.

For C_t to equal P: P must be (t[0]^a, t[1]^b, t[2]^c_val).
But P is determined by the first components of all position triples:
P = (..., t_c'[0], ...) for all column positions c'.
If we arranged columns by triple group, P has value t[0] on all positions in triple group t.

So P = concatenation of (t[0])^{m_t} for each triple t.

For C_t to equal P: (t[0]^a, t[1]^b, t[2]^c_val) must equal this concatenation.

This is a highly constrained system. Let me just enumerate.

Since the column arrangement groups positions by triple, and there are at most 8 triple types,
we have at most 8 "column segments." P's value on each segment is the first component of that
segment's triple. Similarly for Q and R.

For C_t (for each occurring triple t) to be P, Q, or R:
C_t is block-constant with blocks (a, b, c_val).
P, Q, R are block-constant with blocks defined by the column grouping.

For C_t = P: (t[0]^a, t[1]^b, t[2]^c_val) must equal P = (t_1[0]^{m_{t_1}}, t_2[0]^{m_{t_2}}, ...).

The LHS has blocks of sizes (a, b, c_val) with values (t[0], t[1], t[2]).
The RHS has blocks determined by the triple grouping, with values being first components.

For these to be equal, the LHS blocks must align with the RHS blocks.

OK this is getting incredibly involved. Let me just take a completely computational approach.
"""

import itertools


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


def comp3_from_triple_groups(n, max_groups=3):
    """
    Enumerate comp-3 k values using triple-group approach.

    Row block sizes: (a, b, c) with a+b+c = n, a,b,c >= 0.
    Column groups: up to `max_groups` triple groups.
    Each group has a triple t ∈ {0,1}^3 and size m_t.
    sum(m_t) = n.

    Column pattern for triple t: (t[0]^a, t[1]^b, t[2]^c).
    Row pattern P: value on group_t = t[0]. So P = concat of t[0]^m_t for each group.
    Similarly Q and R.

    comp = |{distinct rows used} ∪ {distinct columns used}|.
    We want comp <= 3.
    """
    achievable = set()

    # Enumerate subsets of {0,1}^3 of size up to max_groups
    all_triples = list(itertools.product([0, 1], repeat=3))

    for num_groups in range(1, max_groups + 1):
        for triples in itertools.combinations(all_triples, num_groups):
            # Enumerate sizes m_1, ..., m_g that sum to n, each >= 1
            # Use stars and bars
            for sizes in partitions(n, num_groups):
                # Compute row and column patterns
                # Row P: value on group i = triples[i][0]
                # P = concat of triples[i][0]^sizes[i]
                P = tuple(v for i in range(num_groups) for v in [triples[i][0]] * sizes[i])
                Q = tuple(v for i in range(num_groups) for v in [triples[i][1]] * sizes[i])
                R = tuple(v for i in range(num_groups) for v in [triples[i][2]] * sizes[i])

                for a in range(n + 1):
                    for b in range(n + 1 - a):
                        c = n - a - b

                        # Column patterns for each group
                        patterns = set()
                        if a > 0: patterns.add(P)
                        if b > 0: patterns.add(Q)
                        if c > 0: patterns.add(R)

                        for i in range(num_groups):
                            col = tuple(v for pair in [(triples[i][0], a), (triples[i][1], b), (triples[i][2], c)] for v in [pair[0]] * pair[1])
                            patterns.add(col)

                        if len(patterns) <= 3:
                            k = a * sum(P) + b * sum(Q) + c * sum(R)
                            if 0 < k < n * n:
                                achievable.add(k)

    return achievable


def partitions(n, k):
    """Generate all ways to partition n into k positive parts (ordered)."""
    if k == 1:
        yield (n,)
        return
    for i in range(1, n - k + 2):
        for rest in partitions(n - i, k - 1):
            yield (i,) + rest


# Test for small n
for n in range(2, 15):
    S2 = compute_comp2_set(n)
    S3 = comp3_from_triple_groups(n)
    S_all = S2 | S3
    N_all = n * n - 1
    N4 = N_all - len(S_all)
    C_max3 = 3 * n * n - 1 - len(S2)
    C = C_max3 + N4
    print(f"n={n:2d}: N2={len(S2):3d}, N3new={len(S3-S2):3d}, total={len(S_all):4d}/{N_all:4d}, N4={N4:3d}, C={C:5d}")
