"""Project Euler Problem 645: Every Day is a Holiday.

On a planet with N-day years, each emperor's birthday is declared a holiday
and each date between two holidays also becomes a holiday. Find the expected
number of emperors until every day is a holiday.

Suppose an emperor has the kth unique birthday. There are then an expected
N/(N-k) emperors until one has a new distinct birthday. We can compute the
probability P(k) these k birthdays result in a full year of holidays.
"""

from __future__ import annotations


def nCr(n: int, k: int) -> float:
    """Binomial coefficient."""
    if k < 0 or k > n:
        return 0.0
    result = 1.0
    for i in range(k):
        result = result * (n - i) / (i + 1)
    return result


def solve() -> float:
    """Solve Problem 645."""
    N = 10000

    P = 1.0
    ans = 1.0
    for k in range(N - 1, 0, -1):
        ans += (1 - P) * N / (N - k)
        if k >= 2 and 2 * k - N >= 2:
            P *= nCr(2 * k - N, 2) / nCr(k, 2)
        else:
            P = 0.0
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")
    return result


if __name__ == "__main__":
    main()
