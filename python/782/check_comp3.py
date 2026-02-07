#!/usr/bin/env python3
"""
Check if c(n,k) can exceed 3 for n > 4.

Strategy: for n=5, enumerate all possible (P, Q, R, σ, τ) triples where
{P,Q,R} are the 3 patterns and each row/col is in {P,Q,R}.

Actually, let me check n=5 directly: enumerate all 5x5 binary matrices (2^25 = 33M).
That's feasible if we're smart about it.

Actually, from the brute force for n ≤ 4, max c(n,k) = 3. For n=5, C(5) = 64 matches
my formula. So c(n,k) ≤ 3 for n ≤ 5.

Actually wait, let me reconsider. My formula assumes c(n,k) ∈ {1,2,3} and gets:
C(n) = 2*1 + N2*2 + N3*3 = 2 + 2*(n^2-1-N3) + 3*N3 = 2n^2 + N3

If there are k values with c(n,k) = 4, then:
C(n) = 2 + 2*N2 + 3*N3 + 4*N4 = 2 + 2*(n^2-1-N3-N4) + 3*N3 + 4*N4 = 2n^2 + N3 + 2*N4

So if C(20) = 1150 and my N3 = 344, then:
1150 = 800 + 344 + 2*N4 => N4 = 3.

So there would be 3 k values with c(20,k) = 4.

But I need to check: is it possible for c(n,k) to be 4?

Actually, for ANY k between 1 and n^2-1, I claim c(n,k) ≤ 3.
Proof sketch:
- Take a block structure with a rows of all-ones, then enough rows with
  partial ones, then rows of all-zeros.
- This can hit any k value with at most 3 distinct row patterns.
- The columns will have at most 3 distinct patterns too.
- But the UNION of rows and columns might have up to 6 patterns.

Hmm, that doesn't work directly. Let me think more carefully.

For a "block" matrix (a×b block of 1s, rest 0):
- k = a*b
- Rows: {(1^b, 0^{n-b}), (0^n)} (2 patterns, or 1 if a=0 or a=n or b=0 or b=n)
- Cols: {(1^a, 0^{n-a}), (0^n)} (2 patterns)
- Union: ≤ 3 patterns (the shared (0^n), plus possibly 2 more)

Actually: if a ≠ b and 0 < a < n and 0 < b < n:
Row patterns: (1^b 0^{n-b}), (0^n)
Col patterns: (1^a 0^{n-a}), (0^n)
Union: (0^n) is shared, (1^b 0^{n-b}) ≠ (1^a 0^{n-a}) since a ≠ b.
So union has 3 patterns, complexity = 3.

But this only covers k = a*b (products of two numbers 0..n).
We need all k from 0 to n^2.

For non-product k: let's use a staircase.
E.g., for k = a*b + r where 0 < r < n (extra r ones in row a+1):
Matrix: a rows of (1^b 0^{n-b}), 1 row of (1^{b+r'} 0^{n-b-r'}) for some r',
and n-a-1 rows of (0^n). Hmm wait, this gets complicated.

Let me try: take a rows of (1^b 0^{n-b}) and (n-a) rows of (1^{b'} 0^{n-b'}).
k = a*b + (n-a)*b'. With 0 ≤ a ≤ n, 0 ≤ b,b' ≤ n.

This hits all k of the form a*b + (n-a)*b' = n*b' + a*(b-b').
Fix b' and b-b' = δ: k = n*b' + a*δ.
For δ = 1: k = n*b' + a, so k ranges over all integers 0..n^2 in steps of 1.
Specifically, for any k: b' = k // n, a = k mod n, δ = 1, b = b' + 1.
But we need 0 ≤ b ≤ n, so b' + 1 ≤ n, i.e., b' ≤ n-1, i.e., k ≤ n*(n-1) + n-1 = n^2-1.
And a = k mod n, 0 ≤ a ≤ n. Since a = k mod n, 0 ≤ a ≤ n-1 < n. Fine.

So for ANY k from 0 to n^2-1, we can build a 2-row-type matrix.
For k = n^2: all ones.

Now, what's the complexity of this 2-row-type matrix?
Row type 1: (1^b 0^{n-b}) with b = b'+1
Row type 2: (1^{b'} 0^{n-b'})
a rows of type 1, (n-a) rows of type 2.

Column patterns:
- Cols 0..b'-1: both types have 1, so column = (1,...,1) = all-ones.
- Col b': type 1 has 1, type 2 has 0.
  Column = (1^a, 0^{n-a}) [arranged with a ones where type-1 rows are].
  Wait, the arrangement matters! If the a type-1 rows are the first a rows:
  Column b' = (1,...,1, 0,...,0) with a ones.

  Actually, more precisely: type-1 rows have M[i][b']=1, type-2 rows have M[i][b']=0.
  So column b' has 1s in exactly the positions of type-1 rows.
  If arranged as first a rows: col b' = (1^a, 0^{n-a}).

- Cols b'+1..n-1: both types have 0, so column = all-zeros.

Union of patterns:
- Row type 1: (1^{b'+1}, 0^{n-b'-1})
- Row type 2: (1^{b'}, 0^{n-b'})
- Col all-ones: (1,...,1)  [present if b' > 0]
- Col type: (1^a, 0^{n-a})  [present always since 0 < b'+1 and b' < n]
- Col all-zeros: (0,...,0)  [present if b'+1 < n, i.e., b' < n-1]

Case: b' = 0, b = 1, a = k (since k = n*0 + a*1 = a).
Row type 1: (1, 0,...,0). Row type 2: (0,...,0).
Columns: col 0 = (1^a, 0^{n-a}). Cols 1..n-1 = (0,...,0).
Union: {(1,0,...,0), (0,...,0), (1^a,0^{n-a})}
If a = 1: all three coincide to 2 patterns: (1,0,...,0) and (0,...,0). Complexity 2.
If a > 1: 3 patterns. Complexity 3.

Case: b' > 0, b' < n-1 (the general case):
Possible patterns: row1, row2, all-ones, (1^a,0^{n-a}), all-zeros.
Some may coincide:
- row1 = all-ones iff b'+1 = n
- row2 = all-zeros iff b' = 0
- (1^a,0^{n-a}) = row1 iff a = b'+1
- (1^a,0^{n-a}) = row2 iff a = b'
- (1^a,0^{n-a}) = all-ones iff a = n
- (1^a,0^{n-a}) = all-zeros iff a = 0
- all-ones = row1: iff n = b'+1
- all-zeros = row2: iff b' = 0

In the general case (no coincidences): complexity = 5. That's too high!

So this 2-row-type construction gives complexity up to 5, not 3.
I need a better construction.

Wait -- maybe the right approach is to permute the rows so that column b'
has a "nice" pattern. Since the column depends on which rows are type 1 vs 2,
and we can permute the rows freely.

If we arrange the rows so that the a type-1 rows are at positions that make
column b' match an existing pattern...

Hmm, this is the same constraint as in the complexity-2 analysis!

Let me think about it more carefully: what's the minimum complexity achievable
for each k?

For n=5, my formula gives C(5) = 64, which matches. Let me verify by trying
to show c(5,k) ≤ 3 for all k.
"""

# Let me verify C(5) by a different method: for each k, find a matrix with
# complexity ≤ 3.

def find_min_complexity(n, k):
    """Find a matrix with n rows, n cols, k ones, and minimum complexity.
    Use construction: try all 2-row-type arrangements.
    """
    if k == 0 or k == n*n:
        return 1

    min_comp = n + n  # worst case

    # Try 2-row-type construction with various arrangements
    for b1 in range(n+1):  # weight of row type 1
        for b2 in range(n+1):  # weight of row type 2
            for a in range(n+1):  # number of type-1 rows
                kk = a * b1 + (n - a) * b2
                if kk != k:
                    continue

                # Row types: P has 1s in positions 0..b1-1, Q has 1s in positions 0..b2-1.
                # We need to choose which rows are type 1 vs 2.
                # Try block arrangement: first a rows are type 1.
                P = tuple([1]*b1 + [0]*(n-b1))
                Q = tuple([1]*b2 + [0]*(n-b2))

                # Build matrix with first a rows = P
                matrix = [list(P)] * a + [list(Q)] * (n - a)

                # Compute complexity
                patterns = set()
                for row in matrix:
                    patterns.add(tuple(row))
                for j in range(n):
                    col = tuple(matrix[i][j] for i in range(n))
                    patterns.add(col)
                comp = len(patterns)
                min_comp = min(min_comp, comp)

                # Also try other row arrangements
                # Specifically, arrange rows so some column matches a row pattern
                # This is where it gets tricky. Let me try placing type-1 rows
                # at specific positions.

                # What if we place type-1 rows at positions 0..a-1?
                # Then column j:
                # j < min(b1,b2): col = all-ones
                # j in range only type 1: col = (1^a, 0^{n-a})
                # j in range only type 2: col = (0^a, 1^{n-a})
                # j >= max(b1,b2): col = all-zeros

                # What if we place type-1 rows to match some column pattern?
                # E.g., place type-1 rows at positions 0..b1-1 (matching P's 1 positions)
                if a == b1 and a <= n:
                    sigma = [1]*a + [0]*(n-a)
                    matrix2 = []
                    for i in range(n):
                        if sigma[i]:
                            matrix2.append(list(P))
                        else:
                            matrix2.append(list(Q))
                    patterns2 = set()
                    for row in matrix2:
                        patterns2.add(tuple(row))
                    for j in range(n):
                        col = tuple(matrix2[i][j] for i in range(n))
                        patterns2.add(col)
                    min_comp = min(min_comp, len(patterns2))

    return min_comp

print("Checking c(n,k) for n=5:")
total = 0
for k in range(26):
    c = find_min_complexity(5, k)
    total += c
    print(f"  c(5,{k}) <= {c}")
print(f"C(5) <= {total}")

print("\nChecking c(n,k) for n=6:")
total = 0
for k in range(37):
    c = find_min_complexity(6, k)
    total += c
    print(f"  c(6,{k}) <= {c}")
print(f"C(6) <= {total}")
