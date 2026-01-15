"""Project Euler Problem 493: Under the rainbow.

Find the expected number of distinct colors when selecting N balls from an urn
with C colors of K balls each.
"""

from __future__ import annotations


def nCr(n: int, k: int) -> float:
    """Binomial coefficient as float."""
    if k < 0 or k > n:
        return 0.0
    result = 1.0
    for i in range(min(k, n - k)):
        result = result * (n - i) / (i + 1)
    return result


def solve() -> float:
    """Solve Problem 493."""
    N = 20
    K = 10
    C = 7

    p = nCr((C - 1) * K, N) / nCr(C * K, N)
    ans = C * (1 - p)
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9f}")
    return result


if __name__ == "__main__":
    main()
