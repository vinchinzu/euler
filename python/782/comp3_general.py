#!/usr/bin/env python3
"""
General 3-type construction (not just block).

For 3 row types A, B, C with general assignment σ ∈ {0,1,2}^n:
Each column j: col_j = (types[σ[i]][j] for i in range(n)).

For comp <= 3: all column patterns ∈ {A, B, C}.

For BLOCK arrangement (σ = (0^a, 1^b, 2^c)):
Column j depends on (A[j], B[j], C[j]):
This is what we did above.

For GENERAL σ:
Column j depends on (A[j], B[j], C[j]) and σ.
Given (A[j], B[j], C[j]) = (p,q,r), column j = (x_i) where x_i = p if σ[i]=0, q if σ[i]=1, r if σ[i]=2.

For (p,q,r) = (0,0,0): col = 0^n
For (p,q,r) = (1,1,1): col = 1^n
For (p,q,r) = (1,0,0): col[i] = 1 iff σ[i]=0. col = indicator(σ=0).
For (p,q,r) = (0,1,0): col = indicator(σ=1).
For (p,q,r) = (0,0,1): col = indicator(σ=2).
For (p,q,r) = (1,1,0): col = indicator(σ∈{0,1}).
For (p,q,r) = (1,0,1): col = indicator(σ∈{0,2}).
For (p,q,r) = (0,1,1): col = indicator(σ∈{1,2}).

So the possible column patterns are:
0^n, 1^n, I_0, I_1, I_2, I_{01}, I_{02}, I_{12}
where I_S[i] = 1 iff σ[i] ∈ S.

Note: I_{01} = complement of I_2, etc.
I_0 + I_1 + I_2 = 1^n (pointwise).

For comp <= 3 with row types {A, B, C}:
All appearing column patterns must be in {A, B, C}.

The column patterns that appear depend on which (p,q,r) triples occur in positions.
Let T be the set of triples (A[j], B[j], C[j]) for j = 0..n-1.

Each triple generates a specific column pattern (from the 8 listed above).
All generated patterns must be in {A, B, C}.

Row patterns are:
A = (A[0], ..., A[n-1])
B = (B[0], ..., B[n-1])
C = (C[0], ..., C[n-1])

Column patterns are determined by σ and the position-triples.

BLOCK σ: σ = (0^a, 1^b, 2^c).
Then I_0 = (1^a, 0^{b+c}), I_1 = (0^a, 1^b, 0^c), I_2 = (0^{a+b}, 1^c).
I_{01} = (1^{a+b}, 0^c), I_{02} = (1^a, 0^b, 1^c), I_{12} = (0^a, 1^{b+c}).

GENERAL σ: the indicators I_S depend on σ.

KEY INSIGHT: For comp <= 3, we need {col patterns} ⊆ {A, B, C}.
The column patterns are some subset of {0^n, 1^n, I_0, I_1, I_2, I_{01}, I_{02}, I_{12}}.
Each of these must be one of A, B, or C.

Since I_0 + I_1 + I_2 = 1^n (pointwise), and I_{01} = 1^n - I_2, etc.,
these patterns are highly constrained.

If all of I_0, I_1, I_2 appear as columns, then:
I_0 = α, I_1 = β, I_2 = γ where {α, β, γ} ⊆ {A, B, C}.
And α + β + γ = 1^n.

If I_0 = A, I_1 = B, I_2 = C: then A + B + C = 1^n (pointwise).
This means for each position j: A[j] + B[j] + C[j] = 1, so exactly one of A[j], B[j], C[j] is 1.

This is a very clean structure! Each position has exactly one 1 among A, B, C.
Then the position triples (A[j], B[j], C[j]) are all in {(1,0,0), (0,1,0), (0,0,1)}.
Column patterns: I_0 = A, I_1 = B, I_2 = C. No 0^n or 1^n columns.
Complexity = 3 (since A, B, C are distinct and all column patterns are among {A, B, C}).

k = sum over (i,j) of M[i][j]
  = a * wt(A) + b * wt(B) + c * wt(C)
  where a = |{i : σ[i]=0}|, b = |{i : σ[i]=1}|, c = |{i : σ[i]=2}|.

But under the constraint A + B + C = 1^n: wt(A) + wt(B) + wt(C) = n.
Let wA = wt(A), wB = wt(B), wC = wt(C). Then wA + wB + wC = n.
k = a*wA + b*wB + c*wC with a+b+c = n.

This is the set of all values a*wA + b*wB + c*wC where:
- a + b + c = n, a,b,c >= 0
- wA + wB + wC = n, wA, wB, wC >= 1 (at least one position of each type)
  Actually, we need each of A, B, C to have at least 1 position (for the triples to include all 3 types).
  Wait no: we need the triples to generate I_0, I_1, I_2 as column patterns, which happens iff
  there exists at least one position of each type (1,0,0), (0,1,0), (0,0,1).
  Since those are the ONLY possible triples, wA >= 1, wB >= 1, wC >= 1.

  But we also need a >= 1, b >= 1, c >= 1 for all 3 row types to appear.
  Well, if a = 0, then I_0 doesn't appear as a column (no position has σ[i]=0 determining the column value).
  Wait, I_0[i] = 1 iff σ[i] = 0. If a = 0, I_0 = 0^n. So column type (1,0,0) gives column = 0^n.
  Then 0^n must be one of {A, B, C}. But A has at least one 1 (wA >= 1), etc.
  Unless wA = 0, but we said wA >= 1.

  So if a = 0: column from (1,0,0)-type position = 0^n, not in {A,B,C}. So comp > 3.
  Unless there are no (1,0,0) positions, i.e., wA = 0. But then A = 0^n...

  Bottom line: for the clean A+B+C=1^n structure, we need a,b,c >= 1 AND wA,wB,wC >= 1.

So k values from this construction:
{a*wA + b*wB + c*wC : a+b+c=n, a,b,c>=1, wA+wB+wC=n, wA,wB,wC>=1}

But σ is general (not just block), so a,b,c are the frequencies of 0,1,2 in σ.
The actual row types and column patterns don't depend on the specific σ, only on the
frequencies a,b,c (in the block case).

Wait, that's not true for general σ! For general σ, the column pattern I_0 = indicator(σ=0)
depends on the specific arrangement of σ, not just the frequencies.

But the column patterns need to be A, B, C. We said I_0 = A, which means:
indicator(σ=0) = A. So σ[i] = 0 iff A[i] = 1. Similarly σ[i] = 1 iff B[i] = 1, σ[i] = 2 iff C[i] = 1.

Since A + B + C = 1^n (exactly one 1 per position), σ is completely determined by A, B, C!
σ[i] = 0 if A[i]=1, 1 if B[i]=1, 2 if C[i]=1.

So a = wA, b = wB, c = wC. And k = wA^2 + wB^2 + wC^2.

Similarly, we could have I_0 = B, I_1 = C, I_2 = A (any permutation).
I_0 = π(A), I_1 = π(B), I_2 = π(C) for permutation π of {A,B,C}.

If I_0 = B: σ[i]=0 iff B[i]=1. So a = wB. k = wB*wA + wC*wB + wA*wC = wA*wB + wB*wC + wA*wC.
Wait, let me recompute. Row i has type 0 (= A) if σ[i]=0, type 1 (= B) if σ[i]=1, type 2 (= C) if σ[i]=2.

If I_0 = B: σ[i] = 0 iff B[i] = 1. Similarly I_1 = C means σ[i] = 1 iff C[i] = 1.
And I_2 = A means σ[i] = 2 iff A[i] = 1.

So: a = |{i: σ[i]=0}| = wB, b = |{i: σ[i]=1}| = wC, c = |{i: σ[i]=2}| = wA.
k = a*wA + b*wB + c*wC = wB*wA + wC*wB + wA*wC.

Hmm interesting. So for A+B+C=1^n with different σ assignments:

Permutation (I_0=A, I_1=B, I_2=C): σ[i]=0 iff A[i]=1, etc.
  a=wA, b=wB, c=wC. k = wA^2 + wB^2 + wC^2.

Permutation (I_0=B, I_1=C, I_2=A): σ[i]=0 iff B[i]=1, etc.
  a=wB, b=wC, c=wA. k = wB*wA + wC*wB + wA*wC = wA*wB + wB*wC + wC*wA.

Permutation (I_0=C, I_1=A, I_2=B):
  a=wC, b=wA, c=wB. k = wC*wA + wA*wB + wB*wC = same as above.

Actually any permutation that's a derangement of (A,B,C) gives k = wA*wB + wB*wC + wC*wA.
And the identity gives k = wA^2 + wB^2 + wC^2.

Note: wA^2 + wB^2 + wC^2 + wA*wB + wB*wC + wC*wA = n^2 ... no wait.
(wA + wB + wC)^2 = wA^2 + wB^2 + wC^2 + 2(wA*wB + wB*wC + wC*wA) = n^2.
So wA^2 + wB^2 + wC^2 = n^2 - 2(wA*wB + wB*wC + wC*wA).
And the two k values are complementary: they sum to n^2.

So from the A+B+C=1^n construction, we get:
- k = wA^2 + wB^2 + wC^2 (with wA+wB+wC=n, each >= 1)
- k = wA*wB + wB*wC + wC*wA (complement)

What about other constructions where columns are in {A,B,C} but A+B+C ≠ 1^n?

Let me think about what other structures are possible.

Case: exactly 2 of the 8 column indicator types appear, and both are in {A,B,C}.
For example, columns of type I_0 and I_1 only. Then no position has (A[j],B[j],C[j]) = (0,0,1).
Possible triples: a subset of {(1,0,0), (0,1,0), (0,0,0), (1,1,0), (1,0,1), (0,1,1), (1,1,1)}.
But we only get I_0, I_1 columns. So only triples (1,0,0) and (0,1,0) can appear.
Wait, triple (0,0,0) gives column 0^n, and (1,1,0) gives I_{01}, etc.
For only I_0 and I_1 to appear: only triples (1,0,0) and (0,1,0).
So A[j]+B[j]+C[j] <= 1 for all j, and C[j] = 0 for all j. C = 0^n.
Then A+B <= 1^n.

Row types: A (some 1s), B (some 1s), C = 0^n. All distinct.
Column patterns: I_0 (wherever A[j]=1), I_1 (wherever B[j]=1), and if there are
positions with A[j]=B[j]=C[j]=0: column = 0^n = C.

So column types: I_0, I_1, 0^n=C. These must be in {A, B, C}.
I_0 = A (or B or C), I_1 = A (or B or C).
If I_0 = A and I_1 = B: σ determines A and B.
indicator(σ=0) = A. σ[i]=0 iff A[i]=1.
indicator(σ=1) = B. σ[i]=1 iff B[i]=1.
For positions where A[i]=B[i]=0: σ[i]=2 (assigns to C=0^n).
a = wA, b = wB, c = n - wA - wB.
k = wA*wA + wB*wB + 0 = wA^2 + wB^2.

This is a subset of the comp-2 construction! (x^2 + y^2 with x+y <= n).
Wait, this is more general - wA + wB can be less than n.

For comp-2: x^2+y^2 with x+y = n. Here wA+wB can be < n.

So k = wA^2 + wB^2 with 1 <= wA, wB and wA+wB <= n. This is comp <= 3.

NEW k VALUES! wA^2 + wB^2 where wA + wB < n. These are NOT in S2!

For example: n=9, wA=1, wB=1: k = 2. Is 2 in S2(9)?
S2(9) = {c^2: 1<=c<=8} ∪ {81-c^2} ∪ {x^2+y^2: x+y=9} ∪ {2xy: x+y=9}
c^2 values: 1,4,9,16,25,36,49,64
81-c^2: 80,77,72,65,56,45,32,17
x^2+y^2 (x+y=9): 1+64=65, 4+49=53, 9+36=45, 16+25=41, ...
2xy: 2*8=16, 2*2*7=28, 2*3*6=36, 2*4*5=40, ...
2 is NOT in S2(9)!

So k=2 has c(9,k) <= 3 from this construction, but it's not in S2.
And I missed it before!

Let me be more systematic and enumerate ALL comp-3 k values.
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


def compute_comp3_general(n):
    """Find all k values achievable with comp <= 3.

    Construction types for comp <= 3:
    1. k ∈ {0, n^2}: comp = 1.
    2. k ∈ S2: comp = 2.
    3. Various comp = 3 constructions.

    Comp-3 constructions from "A+B+C = 1^n" type:
    a) k = wA^2 + wB^2 + wC^2, wA+wB+wC = n, all >= 1
    b) k = wA*wB + wB*wC + wC*wA (complement of a)

    From "2 of 3 rows used, third is 0^n or 1^n":
    c) k = wA^2 + wB^2, 1 <= wA, 1 <= wB, wA + wB <= n (rows: A, B, 0^n)
    d) k = n^2 - wA^2 - wB^2 (complement)

    From "1 row type, constant":
    e) k = n*w for w = 1..n-1 (comp = 3 with rows {R, 0^n, 1^n})
       But these are multiples of n. Some may be in S2.

    More generally, from the block 3x3 matrix approach:
    f) Various k = polynomial in (a, b, c, M entries)

    Let me try a cleaner enumeration.
    """
    achievable = set()

    # Construction 1: A + B + C = 1^n (partition into 3 groups)
    # k = wA^2 + wB^2 + wC^2
    for wA in range(1, n):
        for wB in range(1, n - wA):
            wC = n - wA - wB
            if wC < 1:
                continue
            achievable.add(wA**2 + wB**2 + wC**2)
            achievable.add(wA*wB + wB*wC + wC*wA)

    # Construction 2: A + B <= 1^n, C = 0^n (2 groups + zeros)
    # k = wA^2 + wB^2
    for wA in range(1, n):
        for wB in range(1, n - wA + 1):
            achievable.add(wA**2 + wB**2)
            achievable.add(n**2 - wA**2 - wB**2)

    # Construction 3: A + B >= 1^n in some sense... Let me think.
    # What about: positions where A[j]+B[j]+C[j] = 2?
    # Triples (1,1,0), (1,0,1), (0,1,1): col = I_{01}, I_{02}, I_{12}.
    # I_{01} = complement of I_2, I_{02} = complement of I_1, I_{12} = complement of I_0.

    # If A + B + C = 2*1^n (each position has exactly two 1s):
    # Then triples are exactly {(1,1,0), (1,0,1), (0,1,1)}.
    # Column patterns: I_{01}, I_{02}, I_{12}.
    # These are complements of I_2, I_1, I_0.
    # For comp 3: {I_{01}, I_{02}, I_{12}} ⊆ {A, B, C}.
    # If I_{01} = A: A[i] = 1 iff σ[i] ∈ {0,1}.
    # But A[i] = 1 at positions of type (1,*,*).
    # Hmm, this is getting circular. Let me use complementation.
    # If M has A+B+C = 2, then M' = J-M (complement) has A'+B'+C' = 1.
    # The complement matrix has k' = n^2 - k, and complexity is the same.
    # So this gives the complement k values, already covered.

    # Construction 4: Mixed triples.
    # Positions have some triples with sum 0, 1, 2, or 3.
    # The column patterns must all be in {A, B, C}.

    # Let me enumerate what other 3-type configurations give comp=3.
    # Using the block construction approach:
    # 3 row types, block arrangement, and check column ⊆ row.

    # Let me also try: 2 active row types + 1 type that coincides with a column pattern.
    # E.g., 3 row types but only 2 are "real" (the third has 0 rows).
    # No, that's just 2 row types.

    # What about: 3 row types with arbitrary (non-block) σ,
    # but the position triples are restricted?

    # Let me enumerate more carefully.
    # Possible sets of position triples that give column patterns in {A,B,C}:

    # Each triple (p,q,r) generates a column pattern.
    # The column pattern depends on σ.
    # For BLOCK σ = (0^a, 1^b, 2^c):
    #   (0,0,0) -> 0^n
    #   (1,0,0) -> (1^a, 0^{b+c})
    #   (0,1,0) -> (0^a, 1^b, 0^c)
    #   (0,0,1) -> (0^{a+b}, 1^c)
    #   (1,1,0) -> (1^{a+b}, 0^c)
    #   (1,0,1) -> (1^a, 0^b, 1^c)
    #   (0,1,1) -> (0^a, 1^{b+c})
    #   (1,1,1) -> 1^n

    # For these to all be in {A, B, C}: use the 3x3 matrix approach.

    # For GENERAL σ, the analysis is different.
    # (1,0,0) -> I_0, (0,1,0) -> I_1, (0,0,1) -> I_2
    # etc.
    # For non-block σ, I_0, I_1, I_2 are general binary vectors.

    # The constraint is: all column patterns ∈ {A, B, C}.
    # Since A, B, C ARE the row types, we need to choose σ and A,B,C
    # such that all columns are rows.

    # For the A+B+C = 1^n case with non-block σ:
    # σ is determined by A,B,C. I already computed k = wA^2+wB^2+wC^2.
    # But we could also use block σ with the same A,B,C.
    # In that case, columns would be different (block columns).

    # Let me focus on what k values each construction gives.

    # Construction 5: Block 3-type with "staircase"
    # 3 row types: (1^w1, 0^{n-w1}), (1^w2, 0^{n-w2}), (1^w3, 0^{n-w3})
    # Block: a rows of type 1, b of type 2, c of type 3.
    for w1 in range(n+1):
        for w2 in range(n+1):
            for w3 in range(n+1):
                for a in range(n+1):
                    for b in range(n+1-a):
                        c = n - a - b
                        k = a*w1 + b*w2 + c*w3
                        if k <= 0 or k >= n*n:
                            continue

                        R1 = (1,)*w1 + (0,)*(n-w1)
                        R2 = (1,)*w2 + (0,)*(n-w2)
                        R3 = (1,)*w3 + (0,)*(n-w3)

                        patterns = set()
                        if a > 0: patterns.add(R1)
                        if b > 0: patterns.add(R2)
                        if c > 0: patterns.add(R3)

                        rows = [R1]*a + [R2]*b + [R3]*c
                        for j in range(n):
                            col = tuple(rows[i][j] for i in range(n))
                            patterns.add(col)

                        if len(patterns) <= 3:
                            achievable.add(k)

    achievable.discard(0)
    achievable.discard(n*n)
    return achievable


# Test
print("Testing comp-3 constructions:")
for n in range(2, 25):
    S2 = compute_comp2_set(n)
    S3 = compute_comp3_general(n)
    S_all = S2 | S3
    N_all = n*n - 1
    N_achieved = len(S_all)
    N4 = N_all - N_achieved
    C = 3*n*n - 1 - len(S2) + N4
    print(f"n={n:2d}: N2={len(S2):3d}, N3new={len(S3-S2):3d}, total={N_achieved:4d}/{N_all:4d}, N4={N4:3d}, C={C:5d}")

print()
print("Expected: C(2)=8, C(5)=64, C(10)=274, C(20)=1150")
for n in [2, 5, 10, 20]:
    S2 = compute_comp2_set(n)
    S3 = compute_comp3_general(n)
    S_all = S2 | S3
    N4 = n*n - 1 - len(S_all)
    C = 3*n*n - 1 - len(S2) + N4
    print(f"  C({n}) = {C}")
