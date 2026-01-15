"""Project Euler Problem 499: St. Petersburg Lottery.

A lottery costs $K and pays $ 2^t, where t is the number of coin flips before
a tail appears. If a gambler starts with $N, find the probability that the
gambler never runs out of money.
"""

from __future__ import annotations


def solve() -> float:
    """Solve Problem 499."""
    N = 10**9
    K = 15
    L = 50

    # Binary search for largest root
    low = 0.0
    high = 1.0
    while low + 1e-15 < high:
        mid = (low + high) / 2
        res = 0.0
        for i in range(L):
            res += (mid ** ((1 << i) - K)) / (2 << i)
        if res < 1:
            high = mid
        else:
            low = mid

    ans = 1 - (low ** N)
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.7f}")
    return result


if __name__ == "__main__":
    main()
