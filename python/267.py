"""Project Euler Problem 267: Billionaire.

Choose a proportion f and starting with $1, and bet f of your capital
repeatedly N times, winning double your bet with probability 1/2 and
losing your bet otherwise. Find the maximum probability of ending with
at least $C over all f.
"""

from __future__ import annotations

import math


def nCr(n: int, r: int) -> float:
    """Binomial coefficient."""
    if r < 0 or r > n:
        return 0.0
    if r > n // 2:
        r = n - r
    result = 1.0
    for i in range(r):
        result = result * (n - i) / (i + 1)
    return result


def solve() -> float:
    """Solve Problem 267."""
    N = 1000
    C = 10**9

    ans = 0.0
    for w in range(N, -1, -1):
        f = (3.0 * w / N - 1) / 2
        if f <= 0 or f >= 1:
            continue
        e = math.exp((N - w) * math.log(1 - f) + w * math.log(1 + 2 * f))
        if e < C:
            break
        ans += nCr(N, w)

    ans /= 2**N
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.12f}")
    return result


if __name__ == "__main__":
    main()
