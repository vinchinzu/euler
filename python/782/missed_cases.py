#!/usr/bin/env python3
"""
Re-examine: did I miss any complexity-2 cases?

My formula produces the set:
{c^2 : 1 ≤ c ≤ n-1}
∪ {n^2 - c^2 : 1 ≤ c ≤ n-1}
∪ {x^2 + y^2 : x+y = n, x ≥ 1, y ≥ 1}
∪ {2xy : x+y = n, x ≥ 1, y ≥ 1}

This was derived from 3 cases:
1. a > 0, b = 0, c > 0, d = 0: gives k = c^2
2. a = 0, b > 0, c > 0, d = 0: gives k = b(2n-b) = n^2-(n-b)^2
3. a = b > 0, c > 0, d > 0: gives k = x^2+y^2 or 2xy

But what about:
4. a > 0, b = 0, c = 0, d > 0: (by symmetry with case 1 swapping P↔Q)
   P = [0]*a + [0]*c + [0]*d = all zeros... wait, c=0 so P = [0]*a + [0]*0 + [0]*d = [0]*(a+d) = all zeros.
   Q = [0]*a + [0]*0 + [1]*d = [0]*a + [1]*d. Weight = d.
   Since b=c=0: P = all-zeros, Q = [0]*a + [1]*d.

   Constraint: a > 0 requires all-zeros ∈ {P,Q}. P = all-zeros. ✓
   d > 0 requires comp(σ) ∈ {P,Q}.
   comp(σ) = P = all-zeros means σ = all-ones, s = n.
   comp(σ) = Q means σ = comp(Q) = [1]*a + [0]*d, s = a.

   For σ = all-ones (s=n): all rows use P = all-zeros. k = 0. Trivial.
   For s = a: k = a*(b+c) + (n-a)*(b+d) = a*0 + d*d = d^2.

   So this gives k = d^2 for d = 1..n-1 (with a = n-d). Same as case 1!

5. a = 0, b = 0, c > 0, d > 0: Q = comp(P), and a=b=0.
   n = c+d. P = [1]*c + [0]*d. Q = [0]*c + [1]*d.
   Q = comp(P). σ ∈ {P, Q} and comp(σ) ∈ {P, Q}.
   Since Q = comp(P), σ = P means comp(σ) = Q. ✓

   σ = P: s = c. k = c*c + d*d = c^2 + d^2 where c+d = n.
   σ = Q: s = d. k = d*c + c*d = 2cd where c+d = n.

   These are included in case 3 (with b=0).

6. a > 0, b > 0, c > 0, d = 0:
   Constraint: a > 0 requires all-zeros ∈ {P,Q}.
   P = [0]*a + [1]*b + [1]*c. Weight = b+c.
   Q = [0]*a + [1]*b. Weight = b.
   all-zeros ∈ {P,Q}: P = all-zeros iff b+c=0 (no, c>0). Q = all-zeros iff b=0 (no, b>0).
   So this case is IMPOSSIBLE. ✓ (My formula correctly excludes it.)

7. a > 0, b > 0, c = 0, d > 0: (symmetric to case 6)
   P = [0]*a + [1]*b. Weight = b.
   Q = [0]*a + [1]*b + [1]*d. Weight = b+d.
   all-zeros ∈ {P,Q}: P = all-zeros iff b=0 (no). Q = all-zeros iff b+d=0 (no).
   IMPOSSIBLE. ✓

8. a > 0, b > 0, c > 0, d > 0:
   Need all-zeros ∈ {P,Q}: P weight = b+c, all-zeros iff b+c=0 (no). Q weight = b+d, all-zeros iff b+d=0 (no).
   Need all-ones ∈ {P,Q}: P = all-ones iff a+d=0 (no). Q = all-ones iff a+c=0 (no).
   IMPOSSIBLE. ✓

9. a = 0, b > 0, c = 0, d > 0:
   P = [1]*b + [0]*d. Weight = b.
   Q = [1]*b + [1]*d. Weight = b+d = n.
   Q = all-ones.
   Constraint: b > 0 requires all-ones ∈ {P,Q}. Q = all-ones. ✓
   d > 0 requires comp(σ) ∈ {P,Q}.

   comp(σ) = P: σ = comp(P) = [0]*b + [1]*d. s = d.
   comp(σ) = Q = all-ones: σ = all-zeros. s = 0.

   For s = 0: all rows use Q = all-ones. k = n^2. Trivial.
   For s = d: k = d*b + b*(b+d) = db + b^2 + bd = b^2 + 2bd = b(b+2d).
   With n = b+d: k = b(b + 2d) = b(b+2(n-b)) = b(2n-b) = n^2 - (n-b)^2.

   This is the same as case 2! (b plays the role of b, and d = n-b.)

10. a = 0, b > 0, c > 0, d > 0:
    P = [1]*b + [1]*c + [0]*d = [1]*(b+c) + [0]*d. Weight = b+c.
    Q = [1]*b + [0]*c + [1]*d. Weight = b+d.
    Constraint: b > 0 requires all-ones ∈ {P,Q}. P = all-ones iff d=0 (no). Q = all-ones iff c=0 (no).
    IMPOSSIBLE if b > 0 and c > 0 and d > 0!

    Wait, that can't be right. Let me recheck.
    all-ones has weight n. P has weight b+c, Q has weight b+d.
    all-ones = P iff b+c = n iff a+d = 0. Since a=0: d=0. But d > 0. So no.
    all-ones = Q iff b+d = n iff a+c = 0. Since a=0: c=0. But c > 0. So no.
    IMPOSSIBLE. ✓

11. a > 0, b = 0, c > 0, d > 0:
    P = [0]*a + [1]*c + [0]*d. Weight = c.
    Q = [0]*a + [0]*c + [1]*d = [0]*(a+c) + [1]*d. Weight = d.
    Constraint: a > 0 requires all-zeros ∈ {P,Q}. P weight = c ≥ 1, Q weight = d ≥ 1. So neither is all-zeros.
    IMPOSSIBLE. ✓

So cases 4, 5, 9 are already captured by cases 1, 2, 3.
And cases 6, 7, 8, 10, 11 are impossible.

This means my formula is complete for the 2-pattern case.

Hmm, but then C(20) should be 1144, not 1150.

Let me double-check the problem values by looking for the OEIS sequence.
"""

# Let me look at the sequence C(1), C(2), C(3), ...
# 2, 8, 22, 38, 64, 94, 130, 170, 222, 274, ...

# OEIS search
print("C(n) for n=1..30:")
def C_formula(n):
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
    N2 = len(s)
    N3 = (n * n - 1) - N2
    return 2 * n * n + N3

for n in range(1, 31):
    print(f"C({n}) = {C_formula(n)}")
