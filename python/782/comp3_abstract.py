#!/usr/bin/env python3
"""
Abstract approach to comp-3 k values.

For comp <= 3 with 3 row types A, B, C and assignment σ:
All columns must be in {A, B, C}.

Each position j has a "type triple" (A[j], B[j], C[j]) ∈ {0,1}^3.
The column at position j depends on σ and this triple.

For block arrangement σ = (0^a, 1^b, 2^c):
Column j = (A[j]^a, B[j]^b, C[j]^c).

For this to be one of A, B, C:
(A[j]^a, B[j]^b, C[j]^c) = A means: A[i] = A[j] for i<a, A[i] = B[j] for a<=i<a+b, A[i] = C[j] for a+b<=i<n.
This means A is block-constant: constant on [0,a), on [a,a+b), and on [a+b,n).

So A = (u^a, v^b, w^c) for some u,v,w ∈ {0,1}. This gives 8 choices.
Similarly B = (p^a, q^b, r^c) and C = (s^a, t^b, x^c).

M_3x3 = [[u,v,w],[p,q,r],[s,t,x]]
Column j in block i: M_3x3[type_of_row(j)][block_of_position(j)]... no.

Wait. The "block" is the ROW arrangement, not column arrangement.
A, B, C are the ROW types (length n vectors).
If A is block-constant: A = (u^a, v^b, w^c) means A has value u for first a positions,
v for next b positions, w for next c positions.

Then column at position j in [0,a):
(A[j], B[j], C[j]) = (u, p, s) [using the block-constant forms].
Column pattern = (u^a, p^b, s^c) ... no, the column depends on σ.

Column at position j (j-th column) = (row_0[j], row_1[j], ..., row_{n-1}[j]).
With block σ: first a rows are A, next b are B, next c are C.
So column j = (A[j], ..., A[j], B[j], ..., B[j], C[j], ..., C[j])
            = (A[j]^a, B[j]^b, C[j]^c).

If A = (u^a, v^b, w^c), then A[j] depends on which block j falls in:
j in [0,a): A[j] = u.
j in [a,a+b): A[j] = v.
j in [a+b,n): A[j] = w.

Similarly for B, C.

So column j in [0,a): = (A[j]^a, B[j]^b, C[j]^c) = (u^a, p^b, s^c).
Column j in [a,a+b): = (v^a, q^b, t^c).
Column j in [a+b,n): = (w^a, r^b, x^c).

For each to be one of {A, B, C}:
(u^a, p^b, s^c) ∈ {(u,v,w) row, (p,q,r) row, (s,t,x) row} [in block form].
Actually (u^a, p^b, s^c) must equal A = (u^a, v^b, w^c) or B = (p^a, q^b, r^c) or C = (s^a, t^b, x^c).

Col from block 0 = (u^a, p^b, s^c):
Matches A = (u^a, v^b, w^c) iff p = v and s = w.
Matches B = (p^a, q^b, r^c) iff u = p and s = r.
Matches C = (s^a, t^b, x^c) iff u = s and p = t.

Col from block 1 = (v^a, q^b, t^c):
Matches A iff v = u and q = v and t = w -> v = u and q = v and t = w
Matches B iff v = p and q = q (always) and t = r -> v = p and t = r
Matches C iff v = s and q = t and t = x -> v = s and q = t and t = x

Col from block 2 = (w^a, r^b, x^c):
Matches A iff w = u and r = v and x = w
Matches B iff w = p and r = q and x = r
Matches C iff w = s and r = t and x = x (always) -> w = s and r = t

This is exactly: M_3x3 has columns that are also rows.
Let me verify: M_3x3 rows are (u,v,w), (p,q,r), (s,t,x).
Columns of M_3x3: (u,p,s), (v,q,t), (w,r,x).
Each column must be one of the rows. This is exactly the 3x3 self-contained condition!

So the block construction is: find 3x3 binary matrices M where
each column of M is also a row of M. Then for any (a,b,c) with a+b+c=n:
- Row types: rows of M, "stretched" to length n by repeating each entry
- k = a*(weight of M_row1)*a_stretch + ...
  Actually k = a*(row1 weight in full vector) + b*(row2 weight) + c*(row3 weight)
  where row_i weight = M[i][0]*a + M[i][1]*b + M[i][2]*c.
  So k = a*(M[0][0]*a + M[0][1]*b + M[0][2]*c) + b*(M[1][0]*a + M[1][1]*b + M[1][2]*c) + c*(M[2][0]*a + M[2][1]*b + M[2][2]*c)

But this is ONLY the block arrangement. Non-block σ gives different constructions!

For the "partition" construction (A+B+C = 1^n), σ is NOT block form.
σ is determined by the positions: σ[i] = 0 if A[i]=1, 1 if B[i]=1, 2 if C[i]=1.
The row assignment matches the column structure.

So I need to consider GENERAL σ, not just block σ.

Let me think about this more carefully for general σ.

For 3 row types A, B, C and arbitrary σ:
Column j = tuple(types[σ[i]][j] for i in range(n)).

Let t_j = (A[j], B[j], C[j]) be the position-triple for position j.

Column j is: for each row i, value = t_j[σ[i]].
So col_j[i] = t_j[σ[i]].

The column pattern depends on σ and t_j only through the "selection function" σ -> t_j.

Specifically, if we define φ_{t_j}: i -> t_j[σ[i]], then col_j = (φ_{t_j}(0), ..., φ_{t_j}(n-1)).

Now, t_j ∈ {0,1}^3. There are 8 possible triples.
For each triple t = (a,b,c):
col = (t[σ[i]] for i in range(n)).

The resulting column is completely determined by:
- Which positions have σ[i] = 0, 1, or 2
- The triple t

Since σ is fixed, the "indicator vectors" I_0, I_1, I_2 are fixed.
col for triple (1,0,0) = I_0
col for triple (0,1,0) = I_1
col for triple (0,0,1) = I_2
col for triple (1,1,0) = I_0 + I_1 (pointwise, since 0+0=0, 1+0=1, 0+1=1... but what about both 1? Can't happen since σ[i] is exactly one of 0,1,2.)
Ah right, I_0[i] + I_1[i] + I_2[i] = 1 for all i (since σ[i] is exactly one value).
So I_0 + I_1 = 1 - I_2 = complement of I_2.

col for triple (1,1,0) = I_{01} = complement(I_2)
col for triple (1,0,1) = I_{02} = complement(I_1)
col for triple (0,1,1) = I_{12} = complement(I_0)
col for triple (0,0,0) = 0^n
col for triple (1,1,1) = 1^n

The 8 possible column patterns are:
0^n, I_0, I_1, I_2, comp(I_2), comp(I_1), comp(I_0), 1^n
= 0^n, I_0, I_1, I_2, I_{01}, I_{02}, I_{12}, 1^n

But I_{01} = comp(I_2), etc.

Note: I_0, I_1, I_2 are determined by σ and they partition {0,...,n-1} into 3 groups.
As binary vectors: I_0 + I_1 + I_2 = 1^n and I_x . I_y = 0 for x ≠ y.

Now, for comp <= 3 with row types {A, B, C}:
All appearing column patterns must be in {A, B, C}.

The appearing column patterns are determined by which position-triples t_j occur.
Let T = {t_j : j = 0,...,n-1} = the set of occurring triples.

Each t ∈ T generates a column pattern. All must be in {A, B, C}.

Row type A = (A[0], ..., A[n-1]).
A[j] = the first component of t_j.

So A = (t_0[0], t_1[0], ..., t_{n-1}[0]).
B = (t_0[1], t_1[1], ..., t_{n-1}[1]).
C = (t_0[2], t_1[2], ..., t_{n-1}[2]).

And the column patterns generated by T are: {col(t) : t ∈ T}
where col(t)[i] = t[σ[i]].

We need {col(t) : t ∈ T} ⊆ {A, B, C}.

Now A[i] = t_i[0], B[i] = t_i[1], C[i] = t_i[2].
So A[i] depends on what triple is at position i.

This is getting complex. Let me just enumerate computationally.

The key insight: for 3 row types with general σ, the construction is
parameterized by:
1. A partition of {0,...,n-1} into groups G_0, G_1, G_2 (possibly empty)
   where G_k = {i : σ[i] = k}.
   |G_0| = a, |G_1| = b, |G_2| = c.

2. A set of position-triples T ⊆ {0,1}^3.
   For each j, t_j ∈ T.
   The number of positions with each triple value.

3. The constraint: for each t ∈ T, col(t) ∈ {A, B, C}.

Since col(t)[i] = t[σ[i]], and A[i] = t_i[0]:
A = column generated by triple (1,0,0) for positions... no.
A[i] = t_i[0]. So A depends on which triple each position has.

Let me parameterize differently.

Let n_t = number of positions with triple t, for each t ∈ {0,1}^3.
Then sum of n_t = n.

And k = sum over all positions j of sum over all rows i of M[i][j]
     = sum over rows i of (weight of row_type(i))
     = a * wt(A) + b * wt(B) + c * wt(C)
where wt(A) = sum of A[j] = sum over t: n_t * t[0]
      wt(B) = sum of B[j] = sum over t: n_t * t[1]
      wt(C) = sum of C[j] = sum over t: n_t * t[2]

So:
wt(A) = n_{100} + n_{110} + n_{101} + n_{111}
wt(B) = n_{010} + n_{110} + n_{011} + n_{111}
wt(C) = n_{001} + n_{101} + n_{011} + n_{111}

k = a*wt(A) + b*wt(B) + c*wt(C)

Now the constraint: for each triple t with n_t > 0, col(t) ∈ {A, B, C}.
col(t) is a vector of length n.
col(t)[i] = t[σ[i]].

If we order positions by σ value: first a positions with σ=0, then b with σ=1, then c with σ=2.
(This is equivalent to block arrangement by relabeling.)

col(t) in block arrangement = (t[0]^a, t[1]^b, t[2]^c).

And A, B, C in this arrangement depend on the position triples:
A = (t_0[0], t_1[0], ..., t_{n-1}[0]) where the ordering puts σ=0 positions first.

But among the σ=0 positions, each has some triple t, and A at that position = t[0].
Similarly among σ=1 positions: A at that position = t[0] of the triple at that position.
And σ=2 positions: A at that position = t[0] of the triple there.

So A is NOT block-constant unless all positions in the same σ-group have the same triple.
But we're free to ARRANGE the positions within each σ-group arbitrarily.
The ROW TYPE assignments are determined by σ.
The POSITION triples are determined by A, B, C (which are the row types).

Actually, wait. Let me re-derive.

We have n positions (columns) and n row indices.
Row types: A, B, C (binary vectors of length n).
σ assigns each row index i to a type (0, 1, or 2).
M[i][j] = types[σ[i]][j].

Column j = (types[σ[i]][j] for i in range(n)) = (A[j], B[j], or C[j] depending on σ[i]).

The key: we can relabel the row indices so that σ = (0^a, 1^b, 2^c).
Then column j = (A[j]^a, B[j]^b, C[j]^c)... NO!
Column j = (A[j] for first a rows, B[j] for next b rows, C[j] for last c rows).
= (A[j], ..., A[j], B[j], ..., B[j], C[j], ..., C[j])
with a copies of A[j], b copies of B[j], c copies of C[j].

For this to equal A (the full vector), we need:
A[i] for i in σ=0 group: A[j] (constant within group, = A[j])
A[i] for i in σ=1 group: B[j]
A[i] for i in σ=2 group: C[j]

So column j = A iff for each row i:
  If σ[i]=0: A[j] must equal A[i]
  If σ[i]=1: B[j] must equal A[i]
  If σ[i]=2: C[j] must equal A[i]

This means A is constant on each σ-group, with:
  Value on σ=0 group: A[j]
  Value on σ=1 group: B[j]
  Value on σ=2 group: C[j]

But A is a specific vector. A[i] depends on i, not on σ[i].
Unless we choose σ such that all rows in the same group have the same A-value.

OK, this is the fundamental insight: the relabeling trick (σ = block) changes the
COLUMN structure but not the row structure. With general σ, the column patterns
depend on A, B, C AND σ in a more complex way.

Let me just abandon the analytical approach and enumerate computationally
for each n, using the weight-based parametrization.

For given n and (a, b, c) with a+b+c = n:
The column at position j (for block σ) is (A[j]^a, B[j]^b, C[j]^c).
This depends on the triple (A[j], B[j], C[j]).

We can choose any multiset of n triples from {0,1}^3.
Let n_t = count of triple t. Sum = n.

Each triple t with n_t > 0 generates a column pattern: (t[0]^a, t[1]^b, t[2]^c).
These are also in {0,1}^n (block-constant vectors).

Row types:
A = vector where A[j] = t_j[0]. In block arrangement of COLUMNS by triple type:
A[j] = the first component of whichever triple position j has.
A is not necessarily block-constant unless positions are grouped by triple.

Hmm, A is a general vector; it has value 1 at positions where triple starts with 1.

But the COLUMN pattern (t[0]^a, t[1]^b, t[2]^c) IS block-constant (because the ROW
arrangement is block).

The ROW type A is the actual A vector, which is NOT block-constant unless
positions are arranged by triple type.

So column j and row A are both vectors of length n.
Column j = (t_j[0]^a, t_j[1]^b, t_j[2]^c) -- block constant in ROW blocks.
Row A = (t_j[0] for each position j) -- depends on position arrangement.

For column j to equal row A: column j must be A.
(t_j[0]^a, t_j[1]^b, t_j[2]^c) = (t_0[0], t_1[0], ..., t_{n-1}[0]).

The LHS is block-constant: first a entries = t_j[0], next b = t_j[1], last c = t_j[2].
The RHS: first a entries are t_0[0], ..., t_{a-1}[0] (the first components of the first a position triples).

For these to be equal: ALL of the first a positions must have first component = t_j[0].
ALL of the next b positions must have first component = t_j[1].
ALL of the last c positions must have first component = t_j[2].

So A is block-constant iff positions are arranged so that within each column block,
all triples have the same first component.

This is getting very complicated. Let me take a completely different approach.

The problem reduces to:
For n×n binary matrices, find for each k the minimum complexity.
Complexity = |{distinct rows} ∪ {distinct columns}|.

Rather than enumerating all possible matrix structures, let me think about what
determines k values where c(n,k) >= 4.

OBSERVATION: The block construction covers all cases where row types AND column
types are both "block-constant" (i.e., each is of the form (v1^a1, v2^a2, ...)).

For NON-block constructions, the key is that σ can be GENERAL.

Let me think about the "partition" construction more carefully.

PARTITION CONSTRUCTION:
Choose a partition of [n] into 3 groups of sizes p, q, r (p+q+r = n, p,q,r >= 0).
Row type A = indicator of group 1 (weight p).
Row type B = indicator of group 2 (weight q).
Row type C = indicator of group 3 (weight r).
A + B + C = 1^n.

σ[i] is determined by the column structure:
For col j to be one of {A, B, C}, we need specific σ assignments.

Actually, the partition determines A, B, C but σ is a SEPARATE choice.
Let me just enumerate: for each way to assign each of n rows to one of 3 types,
and each way to have A+B+C = 1^n (partition of positions):
- Columns are automatically in {A, B, C} (as shown earlier).
- σ can be ANY assignment (not just matching the partition).
- a, b, c = counts of rows using A, B, C.
- k = a*p + b*q + c*r.

So the achievable k values from this construction are:
{a*p + b*q + c*r : p+q+r = n, p,q,r >= 1, a+b+c = n, a,b,c >= 0,
 but also each σ-type used (a>0, b>0, or c>0) doesn't introduce new patterns}

Wait, I need to check that ALL columns are in {A, B, C}.

Column j: depends on the triple (A[j], B[j], C[j]).
If j is in group 1: A[j]=1, B[j]=0, C[j]=0. Triple = (1,0,0).
If j is in group 2: triple = (0,1,0).
If j is in group 3: triple = (0,0,1).

Column with triple (1,0,0): col[i] = 1 if σ[i]=0 (type A), else 0.
= indicator(σ = 0) = I_0.
For this to be in {A, B, C}:
I_0 = A iff σ matches A's pattern. σ[i]=0 iff A[i]=1. But A[i]=1 iff i is in group 1 (of positions).
Wait, σ assigns ROWS, A describes POSITIONS (columns).
I_0[i] = 1 iff σ[i] = 0. This is a vector indexed by ROW indices.
A = indicator of position group 1, indexed by COLUMN indices.

For an n×n matrix, rows and columns are both indexed by {0,...,n-1}.
I_0 is an n-length vector describing which rows use type A.
A is an n-length vector describing which columns are in group 1.

For column j in group 1 (A[j]=1):
Column j = (types[σ[i]][j])_{i=0}^{n-1} = (A[j] if σ[i]=0, B[j] if σ[i]=1, C[j] if σ[i]=2)_{i}
= (1 if σ[i]=0, 0 if σ[i]=1, 0 if σ[i]=2)_{i}
= I_0.

For this to be A: need I_0 = A. Meaning σ[i]=0 iff A[i]=1 iff i is in group 1 (of positions).
If i is in position group 1: A[i]=1, and we need σ[i]=0.
If i is not in position group 1: A[i]=0, and we need σ[i] ≠ 0.

So for I_0 = A: the a rows that use type A are exactly the rows at positions in group 1.
a = p (size of group 1).

Similarly I_1 = B requires b = q, and I_2 = C requires c = r.
So σ = A (as a mapping), meaning the row assignment mirrors the position partition.
a = p, b = q, c = r.
k = p^2 + q^2 + r^2.

For I_0 = B: I_0[i] = 1 iff σ[i]=0 iff B[i]=1 iff i in group 2.
So rows in group 2 use type A. a = q.
Then I_1 must be C or A (for columns in group 2):
Column in group 2: triple (0,1,0), col = I_1.
If I_1 = C: σ[i]=1 iff C[i]=1 iff i in group 3. b = r.
Then I_2 = indicator(σ=2). σ[i]=2 for remaining rows (group 1). c = p.
Column in group 3: col = I_2 = indicator of group 1 = A. ✓ (A is one of our types).
k = a*p + b*q + c*r = q*p + r*q + p*r = pq + qr + pr.

So with the partition construction, we can get:
- k = p^2 + q^2 + r^2 (identity permutation of groups)
- k = pq + qr + pr (derangement)
- And possibly other permutations, but all non-identity perms of 3 elements give k = pq+qr+pr.

Wait, what about I_0 = B, I_1 = A, I_2 = C?
σ[i]=0 iff B[i]=1 iff i ∈ group 2. a = q.
σ[i]=1 iff A[i]=1 iff i ∈ group 1. b = p.
σ[i]=2 iff C[i]=1 iff i ∈ group 3. c = r.
Column in group 1: triple (1,0,0), col = I_0 = B. ✓
Column in group 2: triple (0,1,0), col = I_1 = A. ✓
Column in group 3: triple (0,0,1), col = I_2 = C. ✓
k = q*p + p*q + r*r = 2pq + r^2.

That's different! So we get k = 2pq + r^2 from this permutation.

Let me enumerate all 6 permutations of {A,B,C} -> {I_0, I_1, I_2}:

Perm (A,B,C): I_0=A, I_1=B, I_2=C. a=p, b=q, c=r. k = p^2 + q^2 + r^2.
Perm (A,C,B): I_0=A, I_1=C, I_2=B. a=p, b=r, c=q. k = p^2 + rq + qr = p^2 + 2qr.
Perm (B,A,C): I_0=B, I_1=A, I_2=C. a=q, b=p, c=r. k = qp + pq + r^2 = 2pq + r^2.
Perm (B,C,A): I_0=B, I_1=C, I_2=A. a=q, b=r, c=p. k = qp + rq + pr = pq+qr+pr.
Perm (C,A,B): I_0=C, I_1=A, I_2=B. a=r, b=p, c=q. k = rp + pq + qr = pq+qr+pr.
Perm (C,B,A): I_0=C, I_1=B, I_2=A. a=r, b=q, c=p. k = rp + q^2 + pr = q^2 + 2pr.

So from partition (p,q,r) we get k values:
- p^2 + q^2 + r^2
- p^2 + 2qr
- q^2 + 2pr
- r^2 + 2pq
- pq + qr + pr (appears twice, from 2 perms)

Note: p^2 + q^2 + r^2 + 2(pq + qr + pr) = (p+q+r)^2 = n^2.
And p^2 + 2qr + q^2 + 2pr + r^2 + 2pq = (p+q+r)^2 = n^2.
So {p^2+2qr, q^2+2pr, r^2+2pq} and {p^2+q^2+r^2} together...
Actually p^2 + 2qr = p^2 + 2qr, and this + q^2+2pr + r^2+2pq = p^2+q^2+r^2+2pq+2qr+2pr = n^2.
Hmm, each of the 4 k values is just one of the partition statistics.

And there's also the construction with a=0 (or b=0 or c=0), where only 2 row types are used.
Those give comp <= 2 or comp = 3 with the partition construction using only 2 groups.

Now what about FEWER than all 3 groups? If r = 0 (group 3 empty):
p + q = n, r = 0. A + B = 1^n, C = 0^n.
Triples: only (1,0,0) and (0,1,0). Plus C[j]=0 for all j.
But C = 0^n is still a row type (the third type). We need col patterns ∈ {A, B, C}.
Columns of triples (1,0,0) give I_0, (0,1,0) give I_1.
Need I_0 ∈ {A, B, 0^n} and I_1 ∈ {A, B, 0^n}.
I_0 = A: a = p. I_1 = B: b = q. Then c = n-p-q = 0.
k = p^2 + q^2 + 0 = p^2 + q^2. But since c=0, all rows are A or B, comp = 2. Already in S2.

What if c > 0? Then some rows use C = 0^n.
I_0 must be A, B, or 0^n. I_1 must be A, B, or 0^n.
If I_0 = A and I_1 = B: a = p, b = q, c = n-p-q.
But this requires checking I_2 = indicator(σ=2).
I_2[i] = 1 iff σ[i] = 2 iff row i uses C = 0^n.
But there's no position with triple (0,0,1) (since r=0, C=0^n).
So I_2 doesn't appear as any column pattern. ✓
k = p^2 + q^2 + 0 = p^2 + q^2, with a = p, b = q, c = n-p-q.
Wait, k = a*wA + b*wB + c*wC = p*p + q*q + c*0 = p^2 + q^2.
Same k regardless of c!

But p + q <= n (since c >= 0).
So k = p^2 + q^2 where 1 <= p, 1 <= q, p + q <= n.

This is EXACTLY construction 4 from above! And it's new relative to S2 when p+q < n.

Similarly, using different permutation assignments:
I_0 = B (a = q), I_1 = A (b = p), c = n-p-q.
k = q*p + p*q + 0 = 2pq.
Same as some S2 values but now p+q can be < n.
k = 2pq with 1 <= p, q, p+q <= n. Some of these are not in S2 (which requires p+q=n).

So we also get: k = 2pq for 1 <= p, 1 <= q, p+q <= n.
And complements: k = n^2 - 2pq, k = n^2 - p^2 - q^2.

Let me now compute the COMPLETE set of comp-3 achievable k values.
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


def compute_all_comp_le3(n):
    """All k values with c(n,k) <= 3."""
    achievable = set()

    # Comp 2: from S2
    achievable |= compute_comp2_set(n)

    # Comp 3 from partition (p,q,r) with p+q+r = n, p,q,r >= 1:
    # k ∈ {p^2+q^2+r^2, p^2+2qr, q^2+2pr, r^2+2pq, pq+qr+pr}
    for p in range(1, n):
        for q in range(1, n - p):
            r = n - p - q
            if r < 1:
                continue
            achievable.add(p*p + q*q + r*r)
            achievable.add(p*p + 2*q*r)
            achievable.add(q*q + 2*p*r)
            achievable.add(r*r + 2*p*q)
            achievable.add(p*q + q*r + p*r)

    # Comp 3 from "2 groups + zeros": k = p^2 + q^2, p+q <= n, p,q >= 1
    for p in range(1, n):
        for q in range(1, n - p + 1):
            achievable.add(p*p + q*q)
            achievable.add(n*n - p*p - q*q)

    # Comp 3 from "2 groups + zeros, derangement": k = 2pq, p+q <= n
    for p in range(1, n):
        for q in range(1, n - p + 1):
            achievable.add(2*p*q)
            achievable.add(n*n - 2*p*q)

    # Comp 3 from identical rows: k = n*w, w = 1..n-1
    for w in range(1, n):
        achievable.add(n * w)

    # What about 3 groups with one group having 0 positions but > 0 rows?
    # E.g., partition into 2 groups (p, q) with p+q=n, but 3 row types used.
    # This was already covered above with "2 groups + zeros/ones".

    # What about partition with r=0 and "derangement" permutations?
    # I_0 = A, I_1 = 0^n: but 0^n is not I_1 unless q = 0.
    # Hmm, let me think about this differently.

    # With 2 position groups (p,q) and 3 row types (A, B, C=0^n):
    # Triples at positions: (1,0,0) for group 1, (0,1,0) for group 2.
    # Col patterns: I_0 (from triple (1,0,0)), I_1 (from triple (0,1,0)).
    # Must be in {A, B, 0^n}.

    # Possible assignments:
    # I_0 = A, I_1 = B: a=p, b=q, c=n-p-q. k = p^2 + q^2.
    # I_0 = A, I_1 = 0^n: need I_1 = 0^n, i.e., no row uses type B (b=0).
    #   Then a = p, c = n-p. k = p^2. But comp = 2 (only A and C=0^n used).
    # I_0 = B, I_1 = A: a=q, b=p, c=n-p-q. k = q*p + p*q = 2pq.
    # I_0 = 0^n, I_1 = A: need I_0 = 0^n, i.e., a=0.
    #   Then b = p or q... this gets complicated.

    # I think the constructions above already cover the main cases.
    # Let me also try: 2 position groups (p,q,r=0) with C=1^n.
    # Triples: (1,0,1) and (0,1,1) for the 2 groups.
    # Col from (1,0,1) = I_{02} = comp(I_1).
    # Col from (0,1,1) = I_{12} = comp(I_0).
    # Need these in {A, B, 1^n}.

    # A has 1s at group-1 positions AND wherever C[j]=1 (C=1^n, so everywhere).
    # Wait, A+B+C: positions in group 1 have A[j]=1,B[j]=0,C[j]=1 so triple=(1,0,1).
    # Positions in group 2: A[j]=0,B[j]=1,C[j]=1 so triple=(0,1,1).
    # A = (1s at group 1, 0s at group 2). Weight p.
    # B = (0s at group 1, 1s at group 2). Weight q.
    # C = 1^n.

    # Col from (1,0,1): I_{02}[i] = 1 iff σ[i] ∈ {0,2}.
    # Col from (0,1,1): I_{12}[i] = 1 iff σ[i] ∈ {1,2}.
    # Need I_{02} ∈ {A, B, 1^n} and I_{12} ∈ {A, B, 1^n}.

    # If I_{02} = 1^n: all rows use type 0 or 2, none use type 1 (b=0).
    #   Then I_{12} = complement(I_0). For I_{12} ∈ {A, B, 1^n}:
    #   comp(I_0) = A: I_0 = comp(A). I_0[i]=1 iff A[i]=0 (group 2 positions). a = q.
    #   c = n-q. k = q*p + c*n = qp + (n-q)*n.
    #   comp(I_0) = B: I_0 = comp(B). I_0[i]=1 iff B[i]=0 (group 1). a = p. c = n-p.
    #   k = p*p + (n-p)*n.
    #   comp(I_0) = 1^n: I_0 = 0^n, a = 0. Then c = n. k = n*n.

    # This gives: k = qp + (n-q)*n = qp + n^2 - qn = n^2 - q(n-p) = n^2 - q^2 (since p+q=n... wait, p+q ≤ n).
    # If p+q < n: there are positions with no group. But we said groups cover all positions.
    # Hmm, with C=1^n and groups 1,2: every position is in group 1 or 2. p+q = n.
    # So k = n^2 - q(n-p) = n^2 - q*q. Already in S2 (comp 2 from c^2 complement).

    # OK. What about partial groups with C = 1^n and some "empty" positions?
    # If p + q < n: some positions have triple (0,0,1) (A=0,B=0,C=1).
    # Col from (0,0,1) = I_2.
    # Need I_2 ∈ {A, B, 1^n}.
    # I_2 = A: σ[i]=2 iff A[i]=1 iff i in group 1. c = p.
    #   I_0 = indicator(group... this is getting very involved.

    # Let me just add the staircase construction for smaller n and check.

    achievable.discard(0)
    achievable.discard(n*n)
    return achievable


# Test
print("=== Complete comp-3 characterization ===")
for n in range(1, 31):
    S = compute_all_comp_le3(n)
    N_all = n*n - 1
    N_achieved = len(S)
    N4 = N_all - N_achieved
    S2 = compute_comp2_set(n)
    C = 3*n*n - 1 - len(S2) + N4
    marker = ""
    if n in [2, 5, 10, 20]:
        expected = {2: 8, 5: 64, 10: 274, 20: 1150}
        marker = f"  (expected {expected[n]})" + (" ✓" if C == expected[n] else " ✗")
    print(f"n={n:2d}: achieved={N_achieved:4d}/{N_all:4d}, N4={N4:3d}, C={C:5d}{marker}")
