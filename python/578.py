"""Project Euler Problem 578: Integers with Decreasing Prime Powers.

Find the number of integers Π (p_i)^(e_i) up to N such that e_i ≥ e_j if
p_i ≤ p_j.

We can write n = Π (p_i)^(e_i) * Π q_i where all e_i ≥ 2. First we recurse
to find all powerful numbers (where e_i ≥ 2). For each one, we use Inclusion
Exclusion to find the number of Π q_i, i.e. that do not contain any factors
up to the largest p_i, and do not contain two factors of any other prime q_i.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def solve() -> int:
    """Solve Problem 578."""
    N = 10**13

    primes = sieve(isqrt(N))
    ans = 0

    def parity(n: int) -> int:
        """Return (-1)^n."""
        return 1 if n % 2 == 0 else -1

    def find_remaining(
        min_index: int, n: int, count: int, threshold: int, primes_list: List[int]
    ) -> None:
        """Find remaining factors using inclusion-exclusion."""
        nonlocal ans
        ans += (N // n) * parity(count)
        for index in range(min_index, len(primes_list)):
            p = primes_list[index]
            if index >= threshold:
                if n * p * p > N:
                    break
                find_remaining(
                    index + 1, n * p * p, count + 1, threshold, primes_list
                )
            else:
                if n * p > N:
                    break
                find_remaining(index + 1, n * p, count + 1, threshold, primes_list)

    def find_powerfuls(
        min_index: int, n: int, prev_e: int, primes_list: List[int]
    ) -> None:
        """Find all powerful numbers recursively."""
        find_remaining(0, n, 0, min_index, primes_list)
        for index in range(min_index, len(primes_list)):
            p = primes_list[index]
            if n * p * p > N:
                break
            nn = n * p
            for e in range(2, prev_e + 1):
                nn *= p
                if nn * p > N:
                    break
                find_powerfuls(index + 1, nn * p, e, primes_list)

    find_powerfuls(0, 1, float("inf"), primes)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
