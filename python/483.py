"""Project Euler Problem 483: Repeated permutation.

Let f(P) be the minimum number of times to apply the permutation P before
restoring the original order. Find the average value of f²(P) over all
permutations of {1, 2, ... N}.
"""

from __future__ import annotations

from math import gcd, lcm
from typing import Dict, List, Tuple


def solve() -> float:
    """Solve Problem 483."""
    N = 350

    def sieve_primes(limit: int) -> List[int]:
        """Sieve of Eratosthenes."""
        is_prime = [True] * (limit + 1)
        is_prime[0] = is_prime[1] = False
        for i in range(2, int(limit**0.5) + 1):
            if is_prime[i]:
                for j in range(i * i, limit + 1, i):
                    is_prime[j] = False
        return [i for i in range(limit + 1) if is_prime[i]]

    primes = sieve_primes(N)
    largest_prime_factor = [0] * N
    for p in primes:
        for i in range(p, N, p):
            largest_prime_factor[i] = p

    cache: Dict[Tuple[int, int, int, int], float] = {}

    def sum_f2(max_index: int, min_k: int, n: int, lcm_val: int) -> float:
        """Sum of f² over all permutations."""
        key = (max_index, min_k, n, lcm_val)
        if key in cache:
            return cache[key]

        if n == 0:
            return lcm_val * lcm_val

        if max_index < 0:
            return 0.0

        p = primes[max_index]
        result = 0.0

        # Try all cycle lengths
        for k in range(min_k, n // p + 1):
            cycle_len = k * p
            if cycle_len > n:
                break

            # Number of ways to choose cycle
            ways = 1
            for i in range(cycle_len):
                ways *= n - i
            ways //= cycle_len

            new_lcm = lcm(lcm_val, cycle_len)
            result += ways * sum_f2(max_index - 1, 1, n - cycle_len, new_lcm)

        result += sum_f2(max_index - 1, min_k, n, lcm_val)
        cache[key] = result
        return result

    total = sum_f2(len(primes) - 1, 1, N, 1)
    # Average = total / N!
    fact = 1
    for i in range(1, N + 1):
        fact *= i
    return total / fact


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9e}")
    return result


if __name__ == "__main__":
    main()
