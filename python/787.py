"""Project Euler Problem 787: Bezout's Game.

In Bezout's game, there are two piles of stones with sizes (a,b), and two
players alternate turns where they take c stones from pile 1 and d stones from
pile 2 such that ad-bc=±1, and the player who empties a pile wins. Find the
number of starting pile sizes (a,b), where a,b>0, a+b≤N, and GCD(a,b)=1,
such that the first player can guarantee a win.

We can see by induction that (a,b) is a winning configuration if min(a,b) is
odd. For the base case, if the pile sizes are (k,1) then c=k-1, d=1 is a
valid move that empties the pile of one stone. If the pile sizes are (a,b),
then by Bezout's identity there are only two solutions, one where ad-bc=+1 and
the other where ad-bc=-1. These two solutions (c1,d1) and (c2,d2) satisfy
c1+c2=a and d1+d2=b. We must have c≤d, so the smaller pile will always remain
the smaller pile. So if the smaller pile is odd, there is at least one move
where the smaller pile becomes even (a losing configuration for the next
player). If the smaller pile is even, both moves must result in it becoming
odd (otherwise ad-bc will be even), hence a winning configuration for the next
player.

This means the problem is equivalent to counting (a,b) such that a,b>0,
a+b≤N, GCD(a,b)=1, and min(a,b) is odd. We use the standard approach to remove
the GCD condition; for the condition that a+b≤N/g, the conditions clearly
result in a polynomial expression, which we can determine from small values is
⌊n(n+1)/4⌋. And since the GCD must be odd, the answer is
Σ_{g=1}^N µ(g) ⌊(N/g)(N/g+1)/4⌋.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 787."""
    N = 10**9
    L = isqrt(N)

    mobius = pre_mobius(N)

    ans = 0

    # Direct computation for odd g
    for g in range(1, N // L + 1, 2):
        ans += mobius[g] * (tr(N // g) // 2)

    # For ranges with same t=N/g, use Mertens function (simplified)
    for t in range(1, L):
        mertens_t = sum(mobius[i] for i in range(1, t + 1))
        mertens_t_plus_1 = sum(mobius[i] for i in range(1, t + 2))
        sum_even_t = sum(mobius[i] for i in range(1, t + 1) if i % 2 == 0)
        sum_even_t_plus_1 = sum(mobius[i] for i in range(1, t + 2) if i % 2 == 0)

        ans += (
            ((mertens_t - mertens_t_plus_1) - (sum_even_t - sum_even_t_plus_1))
            * (tr(t) // 2)
        )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
