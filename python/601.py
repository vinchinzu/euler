"""Project Euler Problem 601: Divisibility streaks.

Define streak(n) to be the smallest positive k such that n+k is not divisible
by k+1. Find sum_{i=1}^N P(i, 4^i) where P(s, N) is the number of 1<n<N such
that streak(n) = s.

streak(n) is just the maximum k such that 1,2,...,k all divide n-1. This means
that P(s, N) is just the number of n<N such that 1,2,...,s all divide n-1
(which is (N-2)/LCM(1,2,...,s)) minus the number of n<N such that
1,2,...,s+1 all divide n-1 (which is (N-2)/LCM(1,2,...,s+1)).
"""

from __future__ import annotations

from math import gcd


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def P(s: int, n: int) -> int:
    """Count numbers with streak s."""
    lcm_val = 1
    for i in range(1, s + 1):
        lcm_val = lcm(lcm_val, i)
    return (n - 2) // lcm_val - (n - 2) // lcm(lcm_val, s + 1)


def solve() -> int:
    """Solve Problem 601."""
    N = 31
    ans = 0
    for i in range(1, N + 1):
        ans += P(i, 4**i)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
