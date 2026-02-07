#!/usr/bin/env python3
"""
The complexity-2 k values are:
S = {c^2 : 1 ≤ c ≤ n-1}
  ∪ {n^2 - c^2 : 1 ≤ c ≤ n-1}
  ∪ {x^2 + y^2 : x+y=n, x≥1, y≥1}
  ∪ {2xy : x+y=n, x≥1, y≥1}

Note that {x^2+y^2 : x+y=n} ∪ {2xy : x+y=n} covers all k = x^2+y^2 and k = 2xy.
And x^2+y^2 + 2xy = n^2, so these are complements.

Also, {c^2} ⊆ {x^2+y^2} when y=0... but y ≥ 1 in the third set.
Wait: x^2+y^2 with x+y=n has x ∈ {1,...,n-1}. When x=c, y=n-c:
x^2+y^2 = c^2 + (n-c)^2 = 2c^2 - 2nc + n^2.

The perfect squares c^2 are NOT a subset of {x^2+y^2 : x+y=n} in general.
E.g., n=5, c=2: c^2 = 4. x^2+y^2 with x+y=5: 1+16=17, 4+9=13, 9+4=13, 16+1=17.
4 is not in {13, 17}. Correct, they're different.

And 2xy with x+y=5: 2*1*4=8, 2*2*3=12, 2*3*2=12, 2*4*1=8. So {8, 12}.
n^2-c^2 for c=1..4: 24, 21, 16, 9.

So S = {1,4,9,16} ∪ {24,21,16,9} ∪ {13,17} ∪ {8,12}
     = {1,4,8,9,12,13,16,17,21,24}

Let me verify: my earlier computation for n=5 gave comp2 = {1,4,8,9,12,13,16,17,21,24}.
Yes! 10 values. Matches.

Now N2 = |S| for each n.

Let me compute |S| more efficiently.

S = S1 ∪ S2 ∪ S3 ∪ S4 where:
S1 = {c^2 : 1 ≤ c ≤ n-1}
S2 = {n^2 - c^2 : 1 ≤ c ≤ n-1}
S3 = {c^2 + (n-c)^2 : 1 ≤ c ≤ n-1}
S4 = {2c(n-c) : 1 ≤ c ≤ n-1}

Note: S2 = n^2 - S1 (complement). S4 = n^2 - S3.
Also S3 and S4 are symmetric under c ↔ n-c, so they each have ⌈(n-1)/2⌉ distinct values.
S1 has n-1 values. S2 has n-1 values.

|S1| = n-1 (all c^2 are distinct for 1 ≤ c ≤ n-1).
|S2| = n-1 (all n^2-c^2 are distinct).
|S3| = ⌈(n-1)/2⌉ (since c^2+(n-c)^2 is symmetric around n/2).
|S4| = ⌈(n-1)/2⌉ (since 2c(n-c) is symmetric around n/2).

But |S| accounts for overlaps. The overlaps matter!

Let me compute |S| and N3 for specific n values.

Actually, the key question is: what's the total count of k NOT in S?
N3 = (n^2-1) - |S|.
C(n) = 2n^2 + N3 = 2n^2 + n^2 - 1 - |S| = 3n^2 - 1 - |S|.

For C(20) = 1150: |S| should be 3*400 - 1 - 1150 = 49.
But I computed |S| = 55. So I'm overcounting by 6.

For C(10) = 274: |S| should be 300 - 1 - 274 = 25. I get |S| = 25. ✓

So the formula works for n=10 but not n=20. The issue MUST be that some k values
in S for n=20 actually require complexity > 2. In other words, they're in my formula
set S but can't actually be achieved with 2 patterns.

But I VERIFIED by direct enumeration that my formula set S matches the direct
(P,Q,σ) enumeration for n ≤ 10!

For n > 10, the direct (P,Q,σ) enumeration is too expensive (2^n choices for each).

Hmm, but wait. In the direct enumeration (verify_n8.py), I used an optimized
approach where I computed possible σ values analytically. Let me re-examine that code.
"""

import itertools

def find_comp2_direct_careful(n):
    """Find k values achievable with complexity exactly 2 by careful construction."""
    achievable = set()
    all_vecs = list(itertools.product([0, 1], repeat=n))

    for P in all_vecs:
        for Q in all_vecs:
            if P >= Q:
                continue

            # For each column j, determine column type
            has_10 = False  # P[j]=1, Q[j]=0
            has_01 = False  # P[j]=0, Q[j]=1
            needs_zero = False  # P[j]=Q[j]=0
            needs_one = False   # P[j]=Q[j]=1

            for j in range(n):
                if P[j] == 1 and Q[j] == 0:
                    has_10 = True
                elif P[j] == 0 and Q[j] == 1:
                    has_01 = True
                elif P[j] == 0 and Q[j] == 0:
                    needs_zero = True
                elif P[j] == 1 and Q[j] == 1:
                    needs_one = True

            # Check constant column constraints
            if needs_zero:
                zero_vec = tuple([0]*n)
                if zero_vec != P and zero_vec != Q:
                    continue
            if needs_one:
                one_vec = tuple([1]*n)
                if one_vec != P and one_vec != Q:
                    continue

            # Determine valid σ values
            possible_sigmas = set()

            if has_10 and has_01:
                # σ ∈ {P, Q} and comp(σ) ∈ {P, Q}
                # Need Q = comp(P)
                comp_P = tuple(1-x for x in P)
                if comp_P == Q:
                    possible_sigmas.add(P)
                    possible_sigmas.add(Q)
            elif has_10:
                # σ ∈ {P, Q}
                possible_sigmas.add(P)
                possible_sigmas.add(Q)
            elif has_01:
                # comp(σ) ∈ {P, Q}, so σ ∈ {comp(P), comp(Q)}
                possible_sigmas.add(tuple(1-x for x in P))
                possible_sigmas.add(tuple(1-x for x in Q))
            else:
                # No type C or D: P = Q. But P < Q, contradiction.
                continue

            for sigma in possible_sigmas:
                # Verify: build matrix and check complexity
                s = sum(sigma)
                # k = sum_j [s*P[j] + (n-s)*Q[j]]
                k = s * sum(P) + (n - s) * sum(Q)

                # Actually verify by building the matrix
                matrix = []
                for i in range(n):
                    if sigma[i]:
                        matrix.append(P)
                    else:
                        matrix.append(Q)

                patterns = set()
                for row in matrix:
                    patterns.add(row)
                for j in range(n):
                    col = tuple(matrix[i][j] for i in range(n))
                    patterns.add(col)

                comp = len(patterns)
                if comp <= 2 and 0 < k < n*n:
                    achievable.add(k)

    return achievable


# Test for n = 2..10
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


for n in range(2, 11):
    direct = find_comp2_direct_careful(n)
    formula = compute_comp2_formula(n)

    if direct != formula:
        print(f"n={n}: DIFFER!")
        print(f"  In direct not formula: {sorted(direct - formula)}")
        print(f"  In formula not direct: {sorted(formula - direct)}")
    else:
        print(f"n={n}: MATCH ({len(formula)} values)")

# Now, for n=11..20, I can't do direct enumeration efficiently.
# But let me check: for the formula values, can I always build a valid matrix?
# The construction requires σ to be a specific vector (P or Q).
# This requires that σ[i] = P[i] (or Q[i]) for all i.
# Since σ is the row assignment (1 if row i uses P), and P[j] is the column pattern,
# σ[i] = P[i] means: row i uses P iff column i is type B or C (P[i]=1).

# This is always constructible: assign column types first, then set σ accordingly.
# So the formula should always be correct!

# BUT WAIT: I need to double-check something. When σ = P, the matrix is:
# M[i][j] = P[j] if P[i]=1 else Q[j].
# Column j: M[i][j] for each i depends on P[i] and P[j]/Q[j].
# If P[j]=Q[j]: M[i][j] = P[j] for all i. Column = constant. OK.
# If P[j]≠Q[j] (say P[j]=1,Q[j]=0): M[i][j] = 1 if P[i]=1 else 0 = P[i].
#   So column j = P (as a vector).
# If P[j]=0,Q[j]=1: M[i][j] = 0 if P[i]=1 else 1 = 1-P[i] = comp(P)[i].
#   So column j = comp(P).

# Column j = P ∈ {P,Q}: always ✓.
# Column j = comp(P). For this to be in {P,Q}: need comp(P) = P or comp(P) = Q.
#   comp(P) = P: impossible (P has at least one 0 and one 1 since P < Q).
#   comp(P) = Q: this is the has_10 and has_01 case.

# So if has_01 = True (there exist j with P[j]=0, Q[j]=1):
# Column j for such j = comp(P). This must be P or Q.
# comp(P) = Q requires the has_10 and has_01 condition.

# Similarly for σ = Q:
# If P[j]=1,Q[j]=0: column j[i] = 1 if Q[i]=1 else 0 = Q[i]. Column = Q ∈ {P,Q} ✓.
# If P[j]=0,Q[j]=1: column j[i] = 0 if Q[i]=1 else 1 = comp(Q)[i].
#   Column = comp(Q). Need comp(Q) ∈ {P,Q}.
#   comp(Q) = P requires Q = comp(P), same condition.
#   comp(Q) = Q: impossible.

# Hmm wait. If has_01 = True and σ = Q:
# Column j (type D) = comp(Q). Need comp(Q) ∈ {P,Q}.
# If comp(Q) = P, this works. Requires P = comp(Q), same as Q = comp(P).

# But what if has_01 = True and has_10 = False? Then P[j] ≤ Q[j] for all j.
# Since P < Q, at least one position has P[j] < Q[j], so has_01 = True.
# has_10 = False means P[j] ≤ Q[j] for all j. So P is componentwise ≤ Q.
# Then types are only A (00), B (11), and D (01). No type C (10).

# In this case:
# σ ∈ {comp(P), comp(Q)} (from has_01, since comp(σ) ∈ {P,Q}).
# Column j type D: comp(σ). If σ = comp(P), comp(σ) = P ∈ {P,Q} ✓.
# Column j type A: all-zeros, needs to be P or Q. Only if P = 0^n (i.e., P has no 1s).
# Column j type B: all-ones, needs to be P or Q. Only if Q = 1^n.

# σ = comp(P): σ[i] = 1 - P[i]. Weight = n - weight(P).
# But σ is the ROW assignment, and comp(P) is a specific vector.
# Row i uses P if comp(P)[i] = 1, i.e., P[i] = 0.
# So rows where P[i]=0 use P, and rows where P[i]=1 use Q.

# The matrix: M[i][j] = P[j] if P[i]=0, else Q[j].
# This is a specific matrix determined by P and Q!

# Let me verify this works for a specific case.
# n=5, P=(0,1,0,0,0), Q=(0,1,1,1,1).
# P < Q lexicographically: (0,1,0,0,0) < (0,1,1,1,1) ✓
# Types: j=0: 00 (A), j=1: 11 (B), j=2: 01 (D), j=3: 01 (D), j=4: 01 (D)
# a=1, b=1, c=0, d=3.
# has_10 = False, has_01 = True.
# needs_zero: a=1 > 0, so all-zeros must be P or Q.
#   P = (0,1,0,0,0), Q = (0,1,1,1,1). Neither is all-zeros. CONSTRAINT VIOLATED!
# So this (P,Q) pair is rejected. Correct.

# Let me try: P = (0,0,0,0,0) = 0^n, Q = (0,0,1,1,1). Weight Q = 3.
# Types: j=0,1: 00 (A), j=2,3,4: 01 (D). a=2, d=3, b=c=0.
# has_10 = False, has_01 = True.
# needs_zero: a=2 > 0. 0^n = P ✓.
# needs_one: b=0, so no.
# σ ∈ {comp(P), comp(Q)} = {1^n, comp(Q)}.
# comp(P) = (1,1,1,1,1), weight = 5.
# comp(Q) = (1,1,0,0,0), weight = 2.

# σ = 1^n (all rows use P = 0^n): k = 0. Trivial.
# σ = comp(Q) = (1,1,0,0,0), weight = 2:
# k = 2 * 0 + 3 * 3 = 9. (2 rows use P=0^n, 3 rows use Q=(0,0,1,1,1))
# Matrix:
# Row 0 (σ=1, P): (0,0,0,0,0)
# Row 1 (σ=1, P): (0,0,0,0,0)
# Row 2 (σ=0, Q): (0,0,1,1,1)
# Row 3 (σ=0, Q): (0,0,1,1,1)
# Row 4 (σ=0, Q): (0,0,1,1,1)
# Columns:
# Col 0: (0,0,0,0,0) = P = 0^n ✓
# Col 1: (0,0,0,0,0) = P ✓
# Col 2: (0,0,1,1,1) = Q ✓
# Col 3: (0,0,1,1,1) = Q ✓
# Col 4: (0,0,1,1,1) = Q ✓
# Complexity = |{P, Q}| = 2. k = 9 = 3^2. ✓

# Great, this works. So the formula gives correct results for explicit constructions.
# I need to find why C(20) doesn't match.

# Let me try a completely fresh approach: compute N3 as the count of k in [1, n^2-1]
# that are NOT representable as:
# (i) c^2 for some 1 ≤ c ≤ n-1, or
# (ii) n^2 - c^2, or
# (iii) x^2 + (n-x)^2 for some 1 ≤ x ≤ n-1, or
# (iv) 2x(n-x)

# And double-check the count for n=20.
print("\n\nDetailed computation for n=20:")
n = 20
S1 = {c*c for c in range(1, n)}
S2 = {n*n - c*c for c in range(1, n)}
S3 = {x*x + (n-x)*(n-x) for x in range(1, n)}
S4 = {2*x*(n-x) for x in range(1, n)}
S = S1 | S2 | S3 | S4

print(f"S1 (c^2): {sorted(S1)}")
print(f"S2 (n^2-c^2): {sorted(S2)}")
print(f"S3 (x^2+y^2, x+y=n): {sorted(S3)}")
print(f"S4 (2xy, x+y=n): {sorted(S4)}")
print(f"|S1|={len(S1)}, |S2|={len(S2)}, |S3|={len(S3)}, |S4|={len(S4)}")
print(f"|S| = {len(S)}")
print(f"N3 = {n*n-1} - {len(S)} = {n*n-1-len(S)}")
print(f"C(20) = 2*{n*n} + {n*n-1-len(S)} = {2*n*n + n*n-1-len(S)}")
print(f"Expected C(20) = 1150")
print(f"Difference = {1150 - (2*n*n + n*n-1-len(S))}")
