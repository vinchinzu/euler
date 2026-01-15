"""Project Euler Problem 688: Piles of Plates.

Let f(n, k) be the maximum possible size of the smallest pile if n plates
are split into k piles of distinct sizes. Find sum_{n,k} f(n, k) for all
n ≤ N.

To compute f(n, k), we first put 1 plate in the second pile, 2 plates in
the third pile, etc., which uses nCr(k, 2) plates, and then split the
remaining plates evenly into k piles. So f(n, k) = ⌊(n - nCr(k, 2)) / k⌋.
"""

from __future__ import annotations


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 688."""
    N = 10**16
    M = 10**9 + 7

    n = N
    ans = 0
    k = 1
    while n > 0:
        limit = (n // k) % M
        term1 = k * limit % (2 * M) * (limit - 1) // 2
        term2 = (n % k + 1) * limit
        ans = (ans + term1 + term2) % M
        n -= k
        k += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
