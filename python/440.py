"""Project Euler Problem 440: GCD and Tiling.

Let T(n) be the number of ways to tile a board of length n using 10 distinct
types of 1x1 blocks or 1x2 blocks. Find Σ_{1 ≤ a,b,c ≤ N} GCD(T(c^a), T(c^b)).

First, we can show by induction that T(x) = T(x-y) T(y) + T(x-y-1) T(y-1) for
all (x,y), and GCD(T(x), T(x-1)) = 1 for all x. This means that
GCD(T(x), T(y)) = GCD(T(x-y-1), T(y)).

Defining T'(n) = T(n-1), we have GCD(T'(x+1), T'(y+1)) = GCD(T'((x+1)-(y+1)),
T'(y+1)). This is the same relationship as the normal GCD, so we have
GCD(T'(x+1), T'(y+1)) = T'(GCD(x+1, y+1)), or GCD(T(x), T(y)) = T(GCD(x+1,
y+1) - 1).

Now plug in x = c^a and y = c^b. We need to determine GCD(c^a + 1, c^b + 1).
Assume without loss of generality that a ≥ b. Then:

GCD(c^a + 1, c^b + 1) = GCD(c^a - c^b, c^b + 1)
                      = GCD(c^{a-b} - 1, c^b + 1)
                      = GCD(c^{a-b} + c^b, c^b + 1)
                      = GCD(c^|a-2b| + 1, c^b + 1).

This effectively just replaces a by |a-2b|, so we can repeat until one of
two things happen. If at any point a=b, then both terms are equal to their GCD,
which is c^g + 1 (g = GCD(a,b)). This happens if a/g and b/g are both odd.
Otherwise, if at any point a=2b, then we see that the GCD is at most 2. In fact,
it is clearly 2 if c is odd, and 1 if c is even.

For each g, we can compute the number of pairs (a,b) such that f(a,b) = c^g + 1,
and we can also compute the number of pairs such that f(a,b) = 2 or 1. Then we
can multiply this number by T(f(a,b) - 1), and sum over all c.

To efficiently compute all T(c^g), we note that T(n) is the top left entry of
A^n, where A = [[K, 1], [1, 0]]. So we can start with A and repeatedly raise
the matrix to the c'th power.
"""

from __future__ import annotations

from math import gcd
from typing import List


def pow2x2(matrix: List[int], exp: int, mod: int) -> List[int]:
    """Raise 2x2 matrix to power exp modulo mod.
    
    Matrix is represented as [a, b, c, d] for [[a, b], [c, d]].
    """
    if exp == 0:
        return [1, 0, 0, 1]  # Identity matrix
    
    result = [1, 0, 0, 1]
    base = matrix[:]
    e = exp
    
    while e > 0:
        if e & 1:
            # Multiply result by base
            a1, b1, c1, d1 = result
            a2, b2, c2, d2 = base
            result = [
                (a1 * a2 + b1 * c2) % mod,
                (a1 * b2 + b1 * d2) % mod,
                (c1 * a2 + d1 * c2) % mod,
                (c1 * b2 + d1 * d2) % mod,
            ]
        # Square base
        a, b, c, d = base
        base = [
            (a * a + b * c) % mod,
            (a * b + b * d) % mod,
            (c * a + d * c) % mod,
            (c * b + d * d) % mod,
        ]
        e >>= 1
    
    return result


def solve() -> int:
    """Solve Problem 440."""
    N = 2000
    K = 10
    M = 987898789
    
    mults = [0] * (N + 1)
    for a in range(1, N + 1):
        for b in range(1, N + 1):
            g = gcd(a, b)
            if (a // g) % 2 == 1 and (b // g) % 2 == 1:
                mults[g] += 1
            else:
                mults[0] += 1
    
    ans = 0
    for c in range(1, N + 1):
        ans = (ans + mults[0] * (1 if c % 2 == 0 else K)) % M
        A = [K, 1, 1, 0]
        for g in range(1, N + 1):
            A = pow2x2(A, c, M)
            ans = (ans + mults[g] * A[0]) % M
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
