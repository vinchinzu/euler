"""Project Euler Problem 694: Cube-full Divisors.

A positive integer n is cube-full if p|n => p³|n. Let s(n) be the number of
cube-full divisors of n. Find sum_{i=1}^n s(i).

A cube-full number must be of the form k = Π (p_i)^(e_i) where e_i ≥ 3, and
we can enumerate over all of them. For each k, the number of positive
integers with k as a divisor is ⌊N/k⌋.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def cb(n: int) -> int:
    """Cube of n."""
    return n * n * n


def solve() -> int:
    """Solve Problem 694."""
    N = 10**18

    max_p = int(N ** (1 / 3)) + 1
    primes = list(primerange(2, max_p + 1))

    def helper(min_index: int, k: int) -> int:
        """Recursive helper."""
        result = N // k
        for index in range(min_index, len(primes)):
            p = primes[index]
            if k * cb(p) > N:
                break
            new_k = k * cb(p)
            while True:
                result += helper(index + 1, new_k)
                if new_k * p > N:
                    break
                new_k *= p
        return result

    return helper(0, 1)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
