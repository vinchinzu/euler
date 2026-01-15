"""Project Euler Problem 708: Twos Are All You Need.

Find Σ_{n=1}^N f(n), where f(n) = 2^{number of prime factors of n}.

Note that f(n) = τ(n) + Σ_d τ(n/d) g(d), where τ(n) is the number of
divisors of n, d ranges over all powerful divisors of n, and g(n) = Π_e
2^{e-2} over all exponents e in the prime factorization of n. This means we
can iterate over all powerful numbers d, and efficiently compute the sum
Σ_{k=1}^{N/d} τ(k) for each d.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def num_divisors(limit: int) -> List[int]:
    """Compute number of divisors for each number up to limit."""
    result = [0] * (limit + 1)
    for i in range(1, limit + 1):
        for j in range(i, limit + 1, i):
            result[j] += 1
    return result


def sum_floor_quotients(n: int) -> int:
    """Sum of τ(k) for k=1 to n."""
    result = 0
    i = 1
    while i <= n:
        q = n // i
        r = n // q
        # Sum τ(k) for k from i to r
        for k in range(i, r + 1):
            # Count divisors of k
            tau_k = 0
            for d in range(1, isqrt(k) + 1):
                if k % d == 0:
                    tau_k += 1
                    if d * d != k:
                        tau_k += 1
            result += tau_k
        i = r + 1
    return result


def solve() -> int:
    """Solve Problem 708."""
    n = 10**14
    l = isqrt(n)

    num_divisors_arr = num_divisors(l)
    sum_floor_quotients_arr = [0] * (l + 1)
    for i in range(1, l + 1):
        sum_floor_quotients_arr[i] = (
            sum_floor_quotients_arr[i - 1] + num_divisors_arr[i]
        )

    primes = sieve(l)
    ans = 0

    def helper(min_index: int, d: int, mult: int) -> None:
        """Recursive helper to iterate over powerful numbers."""
        nonlocal ans
        q = n // d
        if q >= l:
            sum_val = sum_floor_quotients(q)
        else:
            sum_val = sum_floor_quotients_arr[int(q)]
        ans += sum_val * mult

        for index in range(min_index, len(primes)):
            p = primes[index]
            if d * p * p > n:
                break
            new_d = d * p
            e = 2
            while new_d * p <= n:
                new_d *= p
                helper(index + 1, new_d, mult << (e - 2))
                e += 1

    helper(0, 1, 1)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
