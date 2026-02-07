#!/usr/bin/env python3
"""
Careful re-examination of the complexity-2 conditions.

The matrix has patterns P and Q with P != Q.
σ[i] = 1 if row i uses P, else Q.
M[i][j] = P[j] if σ[i]=1 else Q[j].

Column j:
- If P[j] = Q[j]: column is constant, all = P[j].
  The column pattern is (P[j], ..., P[j]).
  If P[j]=0: column = 0^n, must be P or Q.
  If P[j]=1: column = 1^n, must be P or Q.

- If P[j] = 1, Q[j] = 0: column j[i] = σ[i].
  Column = σ, must be P or Q.

- If P[j] = 0, Q[j] = 1: column j[i] = 1 - σ[i].
  Column = comp(σ), must be P or Q.

So: comp(σ) doesn't need to be P or Q independently — it needs to be P or Q
only if there exist type-D positions (P[j]=0, Q[j]=1, i.e., d > 0).

And σ needs to be P or Q only if there exist type-C positions (c > 0).

Now, importantly: if BOTH c > 0 and d > 0, then we need σ ∈ {P,Q} AND comp(σ) ∈ {P,Q}.
This means {σ, comp(σ)} ⊆ {P, Q}. Since σ ≠ comp(σ) (unless all entries are the same,
but then σ would be 0^n or 1^n which means comp(σ) = 1^n or 0^n).
Actually σ = comp(σ) iff σ is impossible for binary (each bit = its complement).
So σ ≠ comp(σ) always.
Therefore {σ, comp(σ)} = {P, Q}. This means Q = comp(P).

If c > 0, d > 0, and Q = comp(P):
P has weight b+c, Q = comp(P) has weight n-(b+c) = a+d.
Also Q has weight b+d.
So a+d = b+d, hence a = b. Also b+c = n-a-d = n-b-d, so b+c+b+d = n, so 2b+c+d = n.
But a+b+c+d = n and a = b, so 2b+c+d = n. This is automatically satisfied.

With Q = comp(P) and a = b:
σ is either P or Q = comp(P).

Case σ = P: weight(σ) = b+c. k = (b+c)*(b+c) + (n-b-c)*(a+d) = (b+c)^2 + (a+d)^2.
But a = b, so k = (b+c)^2 + (b+d)^2.

Case σ = Q = comp(P): weight(σ) = a+d = b+d. k = (b+d)*(b+c) + (a+c)*(b+d).
Wait: k = s*(b+c) + (n-s)*(b+d) where s = b+d.
k = (b+d)(b+c) + (n-b-d)(b+d) = (b+d)(b+c+n-b-d) = (b+d)(n+c-d).
But n = a+b+c+d = 2b+c+d. So n+c-d = 2b+2c. k = (b+d)*2*(b+c).
Hmm, or with σ = P: s = b+c, k = (b+c)^2 + (b+d)*(a+d).
Wait let me recompute: k = s*(b+c) + (n-s)*(b+d).
If σ = P, s = b+c: k = (b+c)^2 + (n-b-c)*(b+d) = (b+c)^2 + (a+d)*(b+d).
Since a = b: k = (b+c)^2 + (b+d)^2. That's correct since a+d = b+d.

Now for the case c > 0, d = 0:
σ must be P or Q (from c > 0). No constraint from d (d = 0).
We also need:
- all-zeros ∈ {P,Q} if a > 0
- all-ones ∈ {P,Q} if b > 0

P = [0]*a + [1]*b + [1]*c + [0]*d = [0]*a + [1]*(b+c) (since d=0)
Q = [0]*a + [1]*b + [0]*c + [1]*d = [0]*(a+c) + [1]*b (since d=0 and rearranging)
Wait, Q = [0]*a + [1]*b + [0]*c. So Q has weight b.

σ = P: weight s = b+c. k = (b+c)^2 + (n-b-c)*b = (b+c)^2 + a*b.
Since d=0: n = a+b+c.

σ = Q: weight s = b. k = b*(b+c) + (n-b)*(b) = b*(b+c) + (a+c)*b = b*(a+2c+b).
Wait: k = b*(b+c) + (a+c)*b. Hmm, let me recompute.
k = s*(b+c) + (n-s)*(b+d) = b*(b+c) + (n-b)*(b+0) = b*(b+c) + (a+c)*b = b*(a+b+2c) = b*(n+c).

And for σ = P: k = (b+c)*(b+c) + a*(b) = (b+c)^2 + ab.

For constraints with a > 0: need all-zeros ∈ {P,Q}.
P = all-zeros iff b+c = 0. Since c > 0, P ≠ all-zeros.
Q = all-zeros iff b = 0.
So if a > 0, need b = 0.

For b > 0: need all-ones ∈ {P,Q}.
P = all-ones iff a = 0 (and d = 0, already true).
Q = all-ones iff a + c = 0 iff a=c=0. But c > 0.
So if b > 0, need a = 0.

So for c > 0, d = 0:
- If a > 0, b = 0: n = a+c. P = [0]*a + [1]*c. Q = [0]*n = all-zeros.
  σ = P: s = c. k = c^2.
  σ = Q: s = 0 (all-zeros). All rows use Q = all-zeros. k = 0. Complexity 1.
  So from σ = P: k = c^2 where c = 1..n-1 (a = n-c > 0).

- If b > 0, a = 0: n = b+c. P = [1]*(b+c) = all-ones. Q = [1]*b + [0]*c.
  σ = P: s = b+c = n. All rows use P = all-ones. k = n^2. Complexity 1.
  σ = Q: s = b. k = b*(b+c) + c*b = b*(b+2c) = b*n + bc. Hmm wait:
  k = b*(n) + (n-b)*b = b*n + c*b = b*(n+c). No wait...
  k = s*(b+c) + (n-s)*(b+d) = b*(b+c) + c*(b+0) = b*(b+c) + c*b = b^2 + 2bc = b^2 + 2bc.
  With n = b+c: k = b^2 + 2bc = b(b+2c) = b(b+2(n-b)) = b(2n-b).

- If a = 0, b = 0: n = c. P = [1]*c. Q = [0]*c. P = all-ones, Q = all-zeros.
  But then {P,Q} = {all-ones, all-zeros}.
  σ = P = all-ones: all rows = P = all-ones. k = n^2. Complexity 1.
  σ = Q = all-zeros: all rows = Q = all-zeros. k = 0. Complexity 1.

So for c > 0, d = 0, the achievable k values are:
1. k = c^2 for c = 1..n-1 (with a = n-c, b = 0)
2. k = b(2n-b) for b = 1..n-1 (with a = 0, c = n-b)

And by symmetry (d > 0, c = 0), the achievable k values are the same
(since swapping P↔Q corresponds to swapping c↔d and complementing σ).

For c > 0, d > 0: Q = comp(P), a = b.
k = (b+c)^2 + (b+d)^2 (for σ = P)
k = (b+d)*(b+c) + (a+c)*(b+d) - wait let me recompute.
σ = P, s = b+c:
k = (b+c)*(b+c) + (n-b-c)*(b+d) = (b+c)^2 + (a+d)*(b+d).
Since a = b: k = (b+c)^2 + (b+d)^2.

σ = Q, s = b+d:
k = (b+d)*(b+c) + (n-b-d)*(b+d) = (b+d)*(b+c+a+c) = (b+d)*(a+b+2c).
Since a = b: k = (b+d)*(2b+2c) = 2*(b+d)*(b+c).

And comp(k): k_comp = n^2 - k.
n = 2b+c+d. n^2 = (2b+c+d)^2.

For σ = P: k = (b+c)^2 + (b+d)^2.
k_comp = n^2 - (b+c)^2 - (b+d)^2 = (2b+c+d)^2 - (b+c)^2 - (b+d)^2.
= 4b^2 + c^2 + d^2 + 4bc + 4bd + 2cd - b^2 - c^2 - 2bc - b^2 - d^2 - 2bd
= 2b^2 + 2bc + 2bd + 2cd = 2(b^2 + bc + bd + cd) = 2(b+c)(b+d).
So k_comp = 2(b+c)(b+d). Which is the same as k for σ = Q!
So the complement gives us the other case. Good, symmetry holds.

SUMMARY OF COMPLEXITY-2 k VALUES:
For an n×n matrix, k has c(n,k) = 2 iff k is in the set:

A) k = c^2, c = 1,...,n-1  [square numbers]
   These come from (a=n-c, b=0, c=c, d=0, σ=P)

B) k = b(2n-b), b = 1,...,n-1
   These come from (a=0, b=b, c=n-b, d=0, σ=Q)
   Note: b(2n-b) = n^2 - (n-b)^2, so these are n^2 - c^2 for c = 1..n-1.
   I.e., complements of case A.

C) k = (b+c)^2 + (b+d)^2, with a=b, a+b+c+d=n, c>0, d>0
   I.e., b ≥ 0, c ≥ 1, d ≥ 1, 2b+c+d = n.
   So k = (b+c)^2 + (b+d)^2 where b+c+b+d = n, b ≥ 0, c ≥ 1, d ≥ 1.
   Let x = b+c, y = b+d. Then x+y = n, x ≥ 1, y ≥ 1, and x-y = c-d (can be anything).
   Also b = x+y-n + b... wait, x = b+c, y = b+d, x+y = 2b+c+d = n. So b = (n-c-d)/2.
   For b ≥ 0: c+d ≤ n. For c ≥ 1, d ≥ 1: c,d ≥ 1.
   Also n-c-d must be even (b integer). So c+d has same parity as n.

   k = x^2 + y^2 where x+y = n, 1 ≤ x ≤ n-1.
   But also need b ≥ 0: b = (n-c-d)/2 = (x-c) = (y-d).
   Since c = x-b and c ≥ 1: x-b ≥ 1, b ≤ x-1.
   Since d = y-b and d ≥ 1: y-b ≥ 1, b ≤ y-1.
   Since b ≥ 0: b ∈ {0, ..., min(x-1, y-1)}.
   This is possible as long as min(x-1, y-1) ≥ 0, i.e., x ≥ 1 and y ≥ 1.

   So for ANY x, y with x+y = n and x,y ≥ 1, and x, y have the same parity as n:
   Wait, we need b integer: b = x - c, and c = x-b, d = y-b.
   n - c - d = n - (x-b) - (y-b) = n - x - y + 2b = 2b.
   This is always even. So b can be any non-negative integer ≤ min(x-1, y-1).
   There's no parity constraint on x, y!

   Actually, let me reconsider. x and y are just two positive integers with x+y = n.
   k = x^2 + y^2.

   For x = 1, y = n-1: k = 1 + (n-1)^2 = n^2 - 2n + 2.
   For x = n-1, y = 1: same k (symmetric).
   ...
   For x = n/2, y = n/2 (if n even): k = 2*(n/2)^2 = n^2/2.

D) k = 2xy where x+y = n, x ≥ 1, y ≥ 1 [complement of case C]
   k = 2xy = 2x(n-x).

   For x = 1: k = 2(n-1).
   For x = n/2 (n even): k = n^2/2.

So the complete set of complexity-2 k values (excluding 0 and n^2) is:

{c^2 : 1 ≤ c ≤ n-1}                          [perfect squares]
∪ {n^2 - c^2 : 1 ≤ c ≤ n-1}                  [complements of squares]
∪ {x^2 + y^2 : x+y = n, x ≥ 1, y ≥ 1}       [sums of squares with x+y=n]
∪ {2xy : x+y = n, x ≥ 1, y ≥ 1}              [complements of above]

Note: x^2+y^2 and 2xy are complementary since x^2+y^2 + 2xy = (x+y)^2 = n^2.

Also note: when c = n (square), k = n^2 (trivial). When c = 0, k = 0.
So perfect squares: k ∈ {1, 4, 9, ..., (n-1)^2}.

And x^2+y^2 with x+y=n: these are n^2 - 2xy for 1 ≤ x ≤ n-1.
2xy = 2x(n-x). These values for x = 1..n-1: 2(n-1), 2*2*(n-2), ..., 2(n-1).

Let me compute these sets explicitly and compare with my analytic formula.
"""

def compute_comp2_formula(n):
    """Compute complexity-2 k values using the derived formula."""
    s = set()

    # Perfect squares: c^2 for c = 1..n-1
    for c in range(1, n):
        s.add(c * c)

    # n^2 - c^2 for c = 1..n-1
    for c in range(1, n):
        s.add(n * n - c * c)

    # x^2 + y^2 with x+y=n, x>=1, y>=1
    for x in range(1, n):
        y = n - x
        s.add(x * x + y * y)

    # 2xy with x+y=n, x>=1, y>=1
    for x in range(1, n):
        y = n - x
        s.add(2 * x * y)

    # Remove 0 and n^2
    s.discard(0)
    s.discard(n * n)

    return s


def get_comp2_k_values_original(n):
    """Original analytic formula."""
    achievable = set()
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
                        k = s * (b + c) + (n - s) * (b + d)
                        if 0 < k < n * n:
                            achievable.add(k)
                        k_comp = n * n - k
                        if 0 < k_comp < n * n:
                            achievable.add(k_comp)
    return achievable


# Compare
for n in range(2, 25):
    formula = compute_comp2_formula(n)
    original = get_comp2_k_values_original(n)

    if formula != original:
        print(f"n={n}: DIFFER")
        print(f"  Formula: {sorted(formula)}")
        print(f"  Original: {sorted(original)}")
        print(f"  In formula not original: {sorted(formula - original)}")
        print(f"  In original not formula: {sorted(original - formula)}")
    else:
        N2 = len(formula)
        N3 = (n*n - 1) - N2
        Cn = 2*n*n + N3
        print(f"n={n}: MATCH, N2={N2}, C={Cn}")
