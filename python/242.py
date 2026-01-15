"""Project Euler Problem 242: Odd Triplets.

Define f(n, k) to be the number of k-element subsets of {1, 2, ... n} with
an odd sum. Find the number of pairs (n, k) with n ≤ N such that n, k, and
f(n, k) are all odd.
"""

from __future__ import annotations


def ceil_div(a: int, b: int) -> int:
    """Return ceil(a/b)."""
    return (a + b - 1) // b


def pow_int(base: int, exp: int) -> int:
    """Return base^exp."""
    return base**exp


def solve() -> int:
    """Solve Problem 242."""
    N = 10**12

    ans = 0
    n = ceil_div(N, 4)
    e = 0

    while n > 0:
        ans = (n % 2 + 1) * ans + (n % 2) * pow_int(3, e)
        n //= 2
        e += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
