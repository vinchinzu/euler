"""Project Euler Problem 595: Incremental Random Sort.

An incremental random sort of the numbers from 1 to n involves repeatedly
doing the following: divide up the sequences into consecutive increasing
subsequences (e.g. 4123756 -> 4|123|7|56) and randomly shuffle the
subsequences (e.g. to 123|56|7|4). Find the expected number of shuffles
required to sort the numbers from 1 to N.

Let g(k) be the number of sequences of 1 to n that consist of exactly k
consecutive increasing subsequences. We can compute g(k) using inclusion-
exclusion, then compute the expected value f(n).
"""

from __future__ import annotations

from math import comb, factorial


def solve() -> float:
    """Solve Problem 595."""
    N = 52

    f = [0.0] * (N + 1)

    for n in range(2, N + 1):
        g = [0.0] * (n + 1)

        for k in range(1, n + 1):
            g[k] = comb(n - 1, k - 1) * factorial(k)
            for i in range(1, k):
                g[k] -= comb(n - i, k - i) * g[i]

        f[n] = g[n]
        for j in range(2, n):
            f[n] += g[j] * (1 + f[j])
        f[n] /= factorial(n) - g[n]

    return f[N]


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")


if __name__ == "__main__":
    main()
