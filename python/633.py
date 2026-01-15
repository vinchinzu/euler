"""Project Euler Problem 633: Square prime factors II.

Let C_k(N) be the number of integers between 1 and N inclusive that are
divisible by p² for exactly k primes p. Find the limit of C_K(N)/N as N
approaches infinity.

Similar to p632, we use dynamic programming by letting c_k be the fraction
of integers that are divisible by p² for exactly k primes p, when only
considering the first t primes.
"""

from __future__ import annotations

from sympy import primerange


def solve() -> float:
    """Solve Problem 633."""
    K = 7

    ans = 0.0
    num_primes = K
    while True:
        c = [0.0] * (K + 2)
        c[0] = 1.0
        primes = list(primerange(2, num_primes + 1))
        for p in primes:
            p_sq = float(p * p)
            for k in range(K, -1, -1):
                c[k + 1] += c[k] / p_sq
                c[k] *= 1 - 1 / p_sq

        if abs(ans / c[K] - 1) < 1e-5 if c[K] != 0 else False:
            break
        ans = c[K]
        num_primes *= 2

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.4e}")
    return result


if __name__ == "__main__":
    main()
