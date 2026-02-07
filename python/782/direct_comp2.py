#!/usr/bin/env python3
"""
Direct computation of complexity-2 k values for any n.

A matrix has complexity 2 with patterns {P, Q} iff:
1. Each row is P or Q (given by σ)
2. Each column is P or Q

Column j = (P[j] if σ[i] else Q[j] for each i)

The columns only depend on σ and whether P[j] = Q[j].
- If P[j] = Q[j]: column j is constant
- If P[j] ≠ Q[j]: column j is σ or complement(σ)

Requirements:
(a) If P[j]=Q[j]=0 for some j: all-zeros must be P or Q
(b) If P[j]=Q[j]=1 for some j: all-ones must be P or Q
(c) If P[j]=1,Q[j]=0 for some j: σ must be P or Q
(d) If P[j]=0,Q[j]=1 for some j: complement(σ) must be P or Q

P is characterized by (a,b,c,d) = counts of type 00, 11, 10, 01 positions.
weight(P) = b+c, weight(Q) = b+d.

σ ∈ {P, Q} means weight(σ) ∈ {b+c, b+d}.
complement(σ) ∈ {P, Q} means weight(complement(σ)) = n - weight(σ) ∈ {b+c, b+d},
i.e., weight(σ) ∈ {n-b-c, n-b-d} = {a+d, a+c}.

But σ = P or Q also means the SPECIFIC arrangement of 0s and 1s matches, not just weight!

The constraint isn't just about weight -- σ must literally equal P or Q as vectors.

However, we're free to choose which positions are type A, B, C, D. So we can arrange
the column types to make P (and Q) match any desired pattern with the right weight.

Specifically: σ has some weight s. If σ = P, then s = b+c, and we need to assign
column types such that P[j]=1 iff j is in the positions where σ[j]=1.
Since σ is a specific vector with s ones at specific positions, we assign:
- The s positions where σ[j]=1 get types B or C (so P[j]=1 there)
- The n-s positions where σ[j]=0 get types A or D (so P[j]=0 there)

Among the s "one" positions: b of them are type B, c of them type C.
Among the n-s "zero" positions: a of them are type A, d of them type D.

But we also need s = b+c (which is automatic since b+c = s).
And a+d = n-s.

So any partition of the s "one" positions into b type-B and c type-C,
and the n-s "zero" positions into a type-A and d type-D, works.

This means the constraint σ = P is satisfiable for any σ with weight b+c,
as long as the column type counts (a,b,c,d) sum to n appropriately.

THEREFORE my formula IS correct: the only thing that matters is the weight s
of σ, not the specific arrangement. And I've been computing this correctly.

So why is C(20) off?

Wait - maybe c(n,k) CAN be > 3 for some n, k.
My formula assumed c(n,k) ∈ {1, 2, 3}. But maybe for some k, the minimum
complexity is 4, 5, or more.

Actually, can we always achieve complexity 3?
A complexity-3 matrix uses 3 patterns. Let me think about what structures
give complexity 3.

Simple structure: rows use patterns A, B; columns use patterns A, C.
With A shared between rows and columns.

Example: "L-shaped" block.
First a rows have pattern (1^b, 0^{n-b}).
Remaining n-a rows have pattern (0^n).
Then:
Distinct rows: {(1^b, 0^{n-b}), (0^n)} (if a < n)
Distinct cols: first b columns are (1^a, 0^{n-a}), rest are (0^n).
So columns: {(1^a, 0^{n-a}), (0^n)}
Union: {(1^b, 0^{n-b}), (0^n), (1^a, 0^{n-a})}
If a ≠ b: complexity 3.
k = a*b.

So for any k = a*b with 1 ≤ a,b ≤ n-1 and a ≠ b, c(n,k) ≤ 3.
For a = b (so k = a^2), c(n,k) ≤ 2.

What about k values that are NOT products a*b with 0 ≤ a,b ≤ n?
Well, actually every k can be written as a*b for suitable a,b... no.
k=2 for n=3: 2 = 1*2. a=1, b=2, both ≤ 3. So c(3,2) ≤ 3 ✓

What about k=5 for n=3? 5 = 1*5 (b=5 > n=3). Or 5 can be done differently.
Actually for n=3: 5 ones in a 3x3 matrix. Can we make a block? 2*3 = 6 > 5.
So a 2×3 block minus 1 one. That's not a rectangle.

Actually, we don't need rectangles. We could use a more complex structure.
Let me think about this more carefully.

For ANY k from 1 to n^2-1, can we achieve complexity ≤ 3?

Consider this construction:
- Take a block of a rows with pattern P1, and (n-a) rows with pattern P2.
- P1 has b ones in the first b positions.
- P2 has b' ones in the first b' positions.
- k = a*b + (n-a)*b'.
- By varying a, b, b', we can hit many k values.

Complexity:
- Rows: {P1, P2} (if P1 ≠ P2)
- Cols j < min(b,b'): all 1s = (1^n) -> this is a column pattern
- Cols min(b,b') ≤ j < max(b,b'): one of (1^a, 0^{n-a}) or (0^a, 1^{n-a})
- Cols j ≥ max(b,b'): all 0s = (0^n)

Hmm, this can give complexity > 3.

Let me instead think about the general construction for complexity ≤ 3.
"""

# Let me just try to find which k values can NOT achieve complexity ≤ 3 for n=20.
# I'll enumerate complexity-3 matrices (3 patterns) and see what k values are covered.

# For complexity ≤ 3, we have at most 3 patterns, say {P, Q, R}.
# Every row is in {P, Q, R} and every column is in {P, Q, R}.
# This is very general. Let me think about what k values this covers.

# Actually, for a "2-row-type block" structure:
# a rows of type P, (n-a) rows of type Q (where P,Q are binary vectors of length n).
# k = a * weight(P) + (n-a) * weight(Q)
# Complexity = |{P, Q} ∪ {column patterns}|

# Column j: if P[j]=Q[j]=v, column is (v,...,v); if P[j]≠Q[j], column is (P[j]^a, Q[j]^{n-a}).
# So column patterns include:
# - (0,...,0) if some P[j]=Q[j]=0
# - (1,...,1) if some P[j]=Q[j]=1
# - (P[j1],...,P[jn-a]) = (1^a, 0^{n-a}) if some P[j]=1,Q[j]=0
# - (0^a, 1^{n-a}) if some P[j]=0,Q[j]=1

# For complexity ≤ 3 with this structure, |row patterns ∪ column patterns| ≤ 3.

# This is complicated. Let me take a different approach.

# OBSERVATION: For each k, consider using 3 row types with specific structures.
# Row type 1: all ones (weight n) - used by a1 rows
# Row type 2: b ones followed by n-b zeros (weight b) - used by a2 rows
# Row type 3: all zeros (weight 0) - used by a3 rows

# k = a1*n + a2*b + a3*0 = a1*n + a2*b
# with a1 + a2 + a3 = n, a1,a2,a3 ≥ 0.

# So k = a1*n + a2*b where a1+a2 ≤ n, b ≤ n.

# This gives k as any value representable as a1*n + a2*b.
# For fixed n, varying a1 from 0 to n:
# k ∈ {a1*n + a2*b : 0 ≤ a2 ≤ n-a1, 0 ≤ b ≤ n}
# = {a1*n + t : 0 ≤ t ≤ (n-a1)*n}  since a2*b ranges over all of 0...(n-a1)*n? No.

# Actually a2*b: a2 ∈ {0,...,n-a1}, b ∈ {0,...,n}. So a2*b covers many values but not all.

# For a1=0: k = a2*b, 0 ≤ a2 ≤ n, 0 ≤ b ≤ n. So k is any product of two numbers ≤ n.
# Products of two numbers ≤ n don't cover all integers up to n^2.
# E.g., n=3: products = {0,1,2,3,4,6,9}. Missing: 5,7,8.

# For a1=1: k = n + a2*b, 0 ≤ a2 ≤ n-1, 0 ≤ b ≤ n.
# n=3: k = 3 + products of ≤2 and ≤3 = 3 + {0,1,2,3,4,6} = {3,4,5,6,7,9}
# Combined with a1=0: {0,1,2,3,4,5,6,7,9}. Still missing 8.

# For a1=2: k = 6 + a2*b, a2 ≤ 1, b ≤ 3.
# k = 6 + {0,1,2,3} = {6,7,8,9}

# Combined: {0,1,2,3,4,5,6,7,8,9} = all! So for n=3, all k covered.

# But what's the complexity of this construction?
# Row types: all-ones, (1^b,0^{n-b}), all-zeros. But some might coincide.
# Column types depend on the structure.

# This is getting complex. Let me just directly compute:
# For each k, find the minimum complexity by trying various matrix structures.

# Actually, let me try a completely different approach.
# CLAIM: c(n,k) ≤ 3 for all n ≥ 1 and 1 ≤ k ≤ n^2-1.

# If true, then C(n) = 2n^2 + N3 where N3 = #{k : 1 ≤ k ≤ n^2-1, c(n,k) = 3}.
# And C(20) should be 1150 = 800 + N3, so N3 = 350.
# My formula gives N3 = 344, which is 6 less.
# So there must be 6 k values that I incorrectly classify as complexity 2.

# Hmm, but I verified directly up to n=10. Maybe the error only appears for n > 10?
# Let me check n=11..20 directly using the (P,Q,σ) approach.

# Actually, I just realized: in the direct method, σ must literally equal P or Q.
# But when I compute k = s*|P| + (n-s)*|Q|, I'm using the weight of P and Q,
# which depends on (a,b,c,d) but NOT on the specific arrangement.
# And σ = P means σ has 1s exactly where P has 1s.
# Since we can arrange column types freely, σ = P is always achievable if
# weight(σ) = weight(P) = b+c = s.

# BUT WAIT: σ determines which ROWS use P vs Q. The rows are indexed 0..n-1.
# P determines which COLUMNS have certain types. P[j] depends on column type.
# σ = P means: row i uses pattern P iff column i is type B or C.
# This is a SELF-REFERENTIAL constraint: the row assignment at position i
# depends on the column type at position i.

# Is this always satisfiable? Let's see:
# We have s = b+c rows using P and n-s using Q.
# We have column types: a type A, b type B, c type C, d type D.
# σ = P means: σ[i] = 1 iff i is a type-B or type-C column position.
# So the s positions where we use row P are exactly the b+c column positions of types B,C.
# This is possible as long as s = b+c (which we ensure).

# But there's a subtlety: which rows use P determines σ, and σ = P means
# σ has 1s at positions of B and C columns. This is a consistent assignment:
# - Position i is a B or C column: row i uses P (σ[i]=1).
# - Position i is an A or D column: row i uses Q (σ[i]=0).

# This is perfectly consistent! There's no circular dependency.
# We're free to choose which positions are which column type.
# We just need b+c positions to be column types B,C (where P[j]=1)
# and the other a+d positions to be types A,D (where P[j]=0).
# We then set σ = P, meaning row i uses P iff column i has type B or C.

# So my formula should be correct. Let me debug more carefully.

# Maybe the complement trick is wrong?
# When I add k_comp = n^2 - k, am I justified?

# Complementing a matrix: swap all 0s and 1s.
# If original has k ones, complement has n^2-k.
# Complexity: row patterns are complemented, col patterns are complemented.
# |{complement(r)} ∪ {complement(c)}| = |complement of (rows ∪ cols)| = |rows ∪ cols|.
# So complexity is preserved. YES, c(n,k) = c(n,n^2-k).

# So the complement trick is valid.

# Let me check: for n=20, which k values are in my comp2 set?
comp2_20 = sorted(get_comp2_k_values(20))
print(f"n=20: N2={len(comp2_20)}")
print(f"comp2 k values: {comp2_20}")
print()

# To find 1150: N3_needed = 350, N2_needed = 399 - 350 = 49.
# I have N2 = 55, so I have 6 extra k values that shouldn't be comp2.
# Let me check if any of my "comp2" k values actually require complexity > 2.

# For each k in comp2_20, verify that a valid complexity-2 matrix exists.
# Build the actual matrix and check.

def verify_comp2_k(n, k):
    """Check if there exists a complexity-2 n×n matrix with exactly k ones."""
    for a in range(n+1):
        for b in range(n+1-a):
            for c in range(n+1-a-b):
                d = n - a - b - c
                if c == 0 and d == 0:
                    continue
                zero_is_P = (b + c == 0)
                zero_is_Q = (b + d == 0)
                ones_is_P = (a + d == 0)
                ones_is_Q = (a + c == 0)
                if a > 0 and not (zero_is_P or zero_is_Q):
                    continue
                if b > 0 and not (ones_is_P or ones_is_Q):
                    continue
                possible_s = set()
                if c > 0 and d > 0:
                    for sc in ['P', 'Q']:
                        for cc in ['P', 'Q']:
                            s1 = (b + c) if sc == 'P' else (b + d)
                            s2 = (a + d) if cc == 'P' else (a + c)
                            if s1 == s2:
                                possible_s.add(s1)
                elif c > 0:
                    possible_s.add(b + c)
                    possible_s.add(b + d)
                elif d > 0:
                    possible_s.add(a + d)
                    possible_s.add(a + c)
                for s in possible_s:
                    if 0 <= s <= n:
                        kk = s * (b + c) + (n - s) * (b + d)
                        if kk == k:
                            # Found! Let's verify by building the matrix.
                            # Arrange: positions 0..b+c-1 are σ=1 (row uses P)
                            # positions b+c..n-1 are σ=0 (row uses Q)
                            # Wait, σ has weight s = b+c or b+d etc.

                            # Column types: first a positions type A, next b type B,
                            # next c type C, next d type D.
                            # P[j]: A->0, B->1, C->1, D->0
                            # Q[j]: A->0, B->1, C->0, D->1
                            P_vec = [0]*a + [1]*b + [1]*c + [0]*d
                            Q_vec = [0]*a + [1]*b + [0]*c + [1]*d

                            # σ = P means σ[i] = P_vec[i]
                            # So row i uses P if P_vec[i]=1, else Q.
                            sigma = P_vec[:]

                            # Build matrix
                            matrix = []
                            for i in range(n):
                                if sigma[i]:
                                    matrix.append(P_vec[:])
                                else:
                                    matrix.append(Q_vec[:])

                            # Check complexity
                            patterns = set()
                            for row in matrix:
                                patterns.add(tuple(row))
                            for j in range(n):
                                col = tuple(matrix[i][j] for i in range(n))
                                patterns.add(col)
                            comp = len(patterns)

                            actual_k = sum(sum(row) for row in matrix)
                            if comp <= 2 and actual_k == k:
                                return True, (a,b,c,d,s)
                            # Try other arrangements...
    return False, None

# Check all comp2_20 values
print("Verifying comp2 k values for n=20:")
bad = []
for k in comp2_20:
    ok, params = verify_comp2_k(20, k)
    if not ok:
        print(f"  k={k}: NOT VERIFIED!")
        bad.append(k)
    # else:
    #     print(f"  k={k}: OK {params}")
print(f"Bad k values: {bad}")
print(f"Number of bad: {len(bad)}")
