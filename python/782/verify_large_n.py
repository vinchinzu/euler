#!/usr/bin/env python3
"""
For n=11..20, try to verify C(n) by computing complexity-2 k values
using the direct (P,Q,σ) enumeration with CAREFUL verification.

For n up to ~12, we can enumerate all P, Q ∈ {0,1}^n (2^n * 2^n / 2 pairs).
For each pair, analytically determine the valid σ values.
Build the actual matrix and verify.

For n=12: 2^12 = 4096, so 4096^2/2 ≈ 8M pairs. For each pair, O(1) work. Feasible.
For n=20: 2^20 ≈ 1M, so 1M^2/2 = 500B pairs. TOO MUCH.

But we can be smarter: for each P, Q, the valid σ is determined analytically.
We just need to check if Q = comp(P) when both type C and D exist,
and if the zero/ones constraints are met.

Actually, the possible (P, Q) pairs are very constrained. Let me enumerate
by the type structure (a, b, c, d) which has O(n^3) possibilities, not O(2^{2n}).

For each (a,b,c,d) with appropriate constraints, there are at most a few valid σ types,
and each gives a unique k value.

Actually, I already did this in verify_each_n20.py and got 55 verified values.
The issue is that C(20) should be 1150 but I get 1144.

Let me take yet another approach: maybe the formula c(n,k) ≤ 3 is WRONG for large n,
and some k values actually have c(n,k) = 4.

For n=20, this would mean:
1150 = 2*400 + N3 + 2*N4  (where N4 = # of k with c(n,k)=4)

With N3 + N4 = n^2 - 1 - N2 = 399 - 55 = 344.
So 1150 = 800 + N3 + 2*N4.
N3 = 344 - N4.
1150 = 800 + 344 - N4 + 2*N4 = 1144 + N4.
So N4 = 6.

OK! So there are exactly 6 k values for n=20 where c(n,k) = 4 (not 3).

Now I need to figure out WHICH k values have c(n,k) = 4.

Hmm, but for n ≤ 5 (and probably n ≤ 10), all k values achieve c ≤ 3 with
3-row-type matrices. So what changes for n=20?

Wait, I only verified c(n,k) ≤ 3 for n=5 (using 3-row-type enumeration).
I didn't verify for n=6,...,10. Let me check if there are any k values
with c(n,k) > 3 for intermediate n values.

Actually, let me compute C(n) assuming c(n,k) ≤ 3 (my formula) and compare
with the CORRECT C(n) to find the first n where N4 > 0.

I know C(n) = 1144 for n=20, and the correct answer is 1150.
If c(n,k) ≤ 3 for all n ≤ 10 (verified), then N4 = 0 for n ≤ 10.
For n=11..19: I need to check.

But I don't have known values for n=11..19.

Instead, let me think about WHICH k values could have c(n,k) = 4.

For c(n,k) = 4, k can't be achieved with ≤ 3 patterns.
For c(n,k) = 3, we need 3 patterns covering all rows and columns.

The question is: for what k values can we NOT find a matrix with 3 patterns?

With 3 patterns {A, B, C} and rows ∈ {A,B,C} and cols ∈ {A,B,C}:
The matrix has row types with multiplicities (r_A, r_B, r_C) where r_A+r_B+r_C = n.
Similarly, column types are determined.

This is very general. Let me think about what k values are achievable
with 3 patterns.

Actually, let me approach it from the other direction. I showed that with
2 row types P, Q and any σ, the achievable k values (with comp ≤ 3) cover
a lot of values. The k values with comp > 3 using 2 row types are the ones
that need 3+ row types to achieve comp ≤ 3.

So the question reduces to: for which k values (that have comp > 3 with 2 types)
can we find a 3-type matrix with comp ≤ 3?

And if for some k values, even 3 types give comp > 3, those have c(n,k) = 4.

Hmm, let me just try to enumerate all k values achievable with comp ≤ 3
using various constructions.
"""

def achievable_with_block(n):
    """
    k values achievable with complexity ≤ 3 using block-like structures.

    Structure 1: a×b block of 1s (top-left), rest 0.
    k = a*b, complexity depends on a, b.

    Structure 2: Two blocks.
    Structure 3: L-shaped regions.
    """
    achievable = set()

    # Structure: a1 rows of weight w1, a2 rows of weight w2 (a1+a2 = n)
    # Rows of weight w: first w positions are 1, rest 0.
    # k = a1*w1 + a2*w2.
    # Complexity depends on column structure.

    for a1 in range(n+1):
        a2 = n - a1
        for w1 in range(n+1):
            for w2 in range(n+1):
                k = a1 * w1 + a2 * w2

                if k < 0 or k > n*n:
                    continue

                # Build matrix:
                # First a1 rows: (1^w1, 0^{n-w1})
                # Last a2 rows: (1^w2, 0^{n-w2})
                P = tuple([1]*w1 + [0]*(n-w1))
                Q = tuple([1]*w2 + [0]*(n-w2))

                # Determine column patterns
                # Columns 0..min(w1,w2)-1: all 1s from both row types -> (1,...,1) if a1>0 and a2>0
                # etc.

                # Just build and compute directly
                matrix = [list(P)] * a1 + [list(Q)] * a2

                patterns = set()
                for row in matrix:
                    patterns.add(tuple(row))
                for j in range(n):
                    col = tuple(matrix[i][j] for i in range(n))
                    patterns.add(col)

                comp = len(patterns)
                if comp <= 3 and 0 < k < n*n:
                    achievable.add(k)

    # Also try permuted rows (not just block arrangement)
    # Key: for 2 row types, we can try σ = various vectors
    # For 3-pattern complexity, we can try σ such that σ is one of the
    # column patterns.

    # Try: σ has a1 ones at specific positions that create nice column patterns
    # E.g., σ has ones at positions 0..a1-1 (block) - already done.
    # Or σ has ones at positions matching Q's 1s.

    for a1 in range(n+1):
        a2 = n - a1
        for w1 in range(n+1):
            for w2 in range(n+1):
                k = a1 * w1 + a2 * w2
                if k <= 0 or k >= n*n:
                    continue
                if k in achievable:
                    continue

                P = [1]*w1 + [0]*(n-w1)
                Q = [1]*w2 + [0]*(n-w2)

                # Try σ = Q (σ[i] = Q[i] for each i)
                sigma = Q[:]
                if sum(sigma) != a1:
                    continue

                matrix = []
                for i in range(n):
                    if sigma[i]:
                        matrix.append(P[:])
                    else:
                        matrix.append(Q[:])

                patterns = set()
                for row in matrix:
                    patterns.add(tuple(row))
                for j in range(n):
                    col = tuple(matrix[i][j] for i in range(n))
                    patterns.add(col)

                comp = len(patterns)
                if comp <= 3:
                    achievable.add(k)

    return achievable

# For n=20
n = 20
block_achievable = achievable_with_block(n)
print(f"n={n}: {len(block_achievable)} k values achievable with comp ≤ 3 (block structures)")

# Compare with comp-2 set
S = set()
for c in range(1, n):
    S.add(c * c)
    S.add(n * n - c * c)
for x in range(1, n):
    y = n - x
    S.add(x * x + y * y)
    S.add(2 * x * y)
S.discard(0)
S.discard(n * n)

# k values with comp = 3 (not in comp-2 set)
comp3 = block_achievable - S
print(f"k values with comp = 3 (block structures): {len(comp3)}")
print(f"Remaining: {n*n - 1 - len(S) - len(comp3)} k values unaccounted for (potentially comp ≥ 4)")

# Check what k values are not covered
all_k = set(range(1, n*n))
not_covered = all_k - S - block_achievable
print(f"Not covered (may need comp > 3): {len(not_covered)}")
if len(not_covered) < 50:
    print(f"  Values: {sorted(not_covered)}")
