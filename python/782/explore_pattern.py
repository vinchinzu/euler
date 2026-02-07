#!/usr/bin/env python3
"""
Analyze which k values give c(n,k) = 1, 2, or 3.

Clearly:
- c(n,k) = 1 iff k = 0 or k = n^2
- c(n,k) >= 2 for 0 < k < n^2
- c(n,k) = 2 for "nice" k values achievable by block-like matrices
- c(n,k) = 3 for the rest

Key question: for which k is c(n,k) = 2?
"""

import itertools

def complexity(matrix):
    n = len(matrix)
    patterns = set()
    for row in matrix:
        patterns.add(tuple(row))
    for j in range(n):
        col = tuple(matrix[i][j] for i in range(n))
        patterns.add(col)
    return len(patterns)

def brute_force_cnk(n):
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
        c = complexity(matrix)
        if k not in cnk or c < cnk[k]:
            cnk[k] = c
    return cnk

# For n=2,3,4, list the k values where c(n,k) = 3
for n in range(2, 5):
    cnk = brute_force_cnk(n)
    comp3 = [k for k in range(n*n+1) if cnk[k] == 3]
    comp2 = [k for k in range(n*n+1) if cnk[k] == 2]
    comp1 = [k for k in range(n*n+1) if cnk[k] == 1]
    print(f"n={n}: c=1 at {comp1}, c=2 at {comp2}, c=3 at {comp3}")
    print(f"  #c1={len(comp1)}, #c2={len(comp2)}, #c3={len(comp3)}")
    print(f"  C({n}) = {len(comp1)}*1 + {len(comp2)}*2 + {len(comp3)}*3 = {sum(cnk[k] for k in range(n*n+1))}")

    # Analysis: which k values have c=3?
    # These are the k NOT achievable by block-type matrices with 2 patterns
    # Block type: a rows with pattern P, (n-a) rows with pattern Q
    # where P = (x^a, y^{n-a}), Q = (x'^a, y'^{n-a}) with (x,y) != (x',y')
    # Then k = a*(x*a + y*(n-a)) + (n-a)*(x'*a + y'*(n-a))

# Now let's think about what k values are achievable with complexity 2.
# A complexity-2 matrix has exactly 2 patterns in the union of rows and columns.
# Let those patterns be P and Q.
# Every row is P or Q, and every column is P or Q.

# More general approach: enumerate all possible (P, Q, row_assignment, col_structure)
# Actually, the "block structure" analysis above covers the case where rows
# are arranged as a block (first a rows = P, rest = Q). But we should also
# consider permutations of rows.

# Wait -- permuting rows and columns doesn't change the set of distinct patterns.
# If we have a rows of type P and n-a rows of type Q (in any order), the columns
# depend on the ORDER of rows. So the block arrangement is WLOG for row patterns,
# but columns change with permutation.

# Actually, that's not right. Let me think again.
# If rows are in some order, columns are determined. Permuting rows permutes
# the entries within each column. So column patterns change.

# For complexity 2 with patterns {P, Q}:
# Each row is P or Q. Say there are 'a' copies of P and 'n-a' copies of Q.
# The rows can be in any order.
# Each column j takes its entries from row values at position j.
# If rows are σ(0), σ(1), ..., σ(n-1) where σ(i) ∈ {P, Q},
# then column j = (σ(0)[j], σ(1)[j], ..., σ(n-1)[j]).
# For this to be in {P, Q}, we need column j to be either P or Q.

# Key insight: σ is a binary vector of length n (1 if row i = P, 0 if row i = Q).
# Column j = (σ(0)*P[j] + (1-σ(0))*Q[j], ...) = based on σ and position j.
# Actually, column j = (σ(i) ? P[j] : Q[j] for i in 0..n-1)
# Wait, that's wrong. Row i is P if σ(i)=1, Q if σ(i)=0.
# So M[i][j] = P[j] if σ(i)=1, Q[j] if σ(i)=0.
# Column j = (P[j]*σ(0) + Q[j]*(1-σ(0)), ...) NO.
# M[i][j] = P[j] if σ(i)=1 else Q[j].
# So column j[i] = P[j] if σ(i)=1, else Q[j].

# If P[j] = Q[j], then column j[i] = P[j] for all i, so column j is constant.
# If P[j] ≠ Q[j], then column j[i] = σ(i) if P[j]=1,Q[j]=0; or column j[i] = 1-σ(i) if P[j]=0,Q[j]=1.

# So column j is either:
# - A constant vector (all P[j]) if P[j]=Q[j]
# - σ (if P[j]=1, Q[j]=0)
# - complement of σ (if P[j]=0, Q[j]=1)

# For column j to be in {P, Q}:
# Case P[j]=Q[j]=0: column = (0,...,0). Must be P or Q.
# Case P[j]=Q[j]=1: column = (1,...,1). Must be P or Q.
# Case P[j]=1,Q[j]=0: column = σ. Must be P or Q.
# Case P[j]=0,Q[j]=1: column = complement(σ). Must be P or Q.

# So σ must be in {P, Q} and complement(σ) must be in {P, Q}.
# Since P ≠ Q, either:
# (a) σ = P and complement(σ) = Q, i.e., Q = complement(P)
# (b) σ = Q and complement(σ) = P, same as (a)
# (c) σ = P and there are no j with P[j]=0,Q[j]=1 (so Q is entry-wise ≤ P or they agree)
# Wait, let me be more careful.

# The constraint is:
# - σ ∈ {P, Q} (if there exists any j with P[j]=1,Q[j]=0)
# - complement(σ) ∈ {P, Q} (if there exists any j with P[j]=0,Q[j]=1)
# - All-zeros ∈ {P, Q} (if there exists any j with P[j]=Q[j]=0)
# - All-ones ∈ {P, Q} (if there exists any j with P[j]=Q[j]=1)

# This is very restrictive!

# Let's categorize positions j:
# Type A: P[j]=0, Q[j]=0
# Type B: P[j]=1, Q[j]=1
# Type C: P[j]=1, Q[j]=0
# Type D: P[j]=0, Q[j]=1

# Let a = |Type A|, b = |Type B|, c = |Type C|, d = |Type D|. a+b+c+d = n.

# Constraints on columns:
# Type A positions: column = all-zeros = (0,...,0). Need (0,...,0) ∈ {P, Q}.
# Type B positions: column = all-ones = (1,...,1). Need (1,...,1) ∈ {P, Q}.
# Type C positions: column = σ. Need σ ∈ {P, Q}.
# Type D positions: column = complement(σ). Need complement(σ) ∈ {P, Q}.

# σ is the indicator of which rows are P. σ has weight 'a_count' (# rows that are P).
# P = (0^a, 1^b, 1^c, 0^d) [grouped by type] = has weight b+c.
# Q = (0^a, 1^b, 0^c, 1^d) = has weight b+d.

# But σ is a binary vector of length n. It's 1 for rows that use P, 0 for Q.
# σ doesn't have a fixed relationship with the type A/B/C/D structure.
# Wait, actually σ is about ROW assignments, while A/B/C/D are about COLUMN positions.

# This is getting very involved. Let me just compute things numerically.
# But the key takeaway: the complexity-2 constraints are very restrictive.

# For now, let me focus on: which k values have c(n,k) = 3?
# And find a formula for C(n) = 2*(n^2+1) - 2 + 3*N3 - N2
# Wait: C(n) = 1*2 + 2*N2 + 3*N3 where N1=2, N2+N3 = n^2-1.
# C(n) = 2 + 2*N2 + 3*N3 = 2 + 2*(n^2-1-N3) + 3*N3 = 2 + 2n^2-2 + N3 = 2n^2 + N3

# So C(n) = 2n^2 + N3, where N3 = number of k in {1,...,n^2-1} with c(n,k) = 3!

print("\n=== KEY FORMULA: C(n) = 2n^2 + N3 ===")
for n in range(1, 5):
    cnk = brute_force_cnk(n)
    N3 = sum(1 for k in range(1, n*n) if cnk[k] == 3)
    Cn = sum(cnk[k] for k in range(n*n+1))
    print(f"n={n}: C({n}) = {Cn}, 2n^2 = {2*n*n}, N3 = {N3}, 2n^2+N3 = {2*n*n+N3}")

# Let me check C(2)=8: 2*4+N3, C(2)=8 so N3=0. But wait, c(2,k) for n=2 has no k with c=3!
# C(3)=22: 2*9+N3=22, N3=4. Matches: k=2,3,6,7 have c=3.
# C(4)=38: 2*16+N3=38, N3=6. Let me verify.
