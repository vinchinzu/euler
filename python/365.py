"""Project Euler Problem 365 (translated from Ruby).

This module computes:

    sum(M(10**18, 10**9, p * q * r))

where 1000 < p < q < r < 5000 and p, q, r are primes, and
M(n, k, m) is the binomial coefficient C(n, k) modulo m.

The implementation is based on Lucas' theorem and the Chinese Remainder
Theorem (CRT). It is written to be an executable, self-contained module
compatible with Python 3.12.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from time import perf_counter
from typing import Dict, List

import sys
import numba

N: int = 10**18
K: int = 10**9
MIN_PRIME: int = 1000
MAX_PRIME: int = 5000


@numba.jit(nopython=True, cache=True)
def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Return (g, x, y) such that a*x + b*y = g = gcd(a, b)."""
    if a == 0:
        return b, 0, 1
    g, y, x = extended_gcd(b % a, a)
    return g, x - (b // a) * y, y


@numba.jit(nopython=True, cache=True)
def mod_inverse(a: int, m: int) -> int:
    """Return modular inverse of a modulo m.

    Raises ValueError if inverse does not exist.
    """
    g, x, _ = extended_gcd(a, m)
    if g != 1:
        return -1  # Changed to avoid raising exception in numba
    return (x % m + m) % m


@dataclass(slots=True)
class FactorialCache:
    """Cache factorial and inverse factorial tables modulo primes.

    For each prime p, we store:
    - fact[p][i]     == i! mod p
    - inv_fact[p][i] == (i!)^{-1} mod p
    for 0 <= i < p.
    """

    fact: Dict[int, List[int]] = field(default_factory=dict)
    inv_fact: Dict[int, List[int]] = field(default_factory=dict)

    def compute_for_prime(self, p: int) -> None:
        """Precompute factorial tables for a prime p if not already present."""
        if p in self.fact:
            return

        fact = [1] * p
        for i in range(1, p):
            fact[i] = (fact[i - 1] * i) % p

        inv_fact = [0] * p
        inv_fact[p - 1] = mod_inverse(fact[p - 1], p)
        for i in range(p - 2, -1, -1):
            inv_fact[i] = (inv_fact[i + 1] * (i + 1)) % p

        self.fact[p] = fact
        self.inv_fact[p] = inv_fact

    def binomial_small(self, n: int, k: int, p: int) -> int:
        """Compute C(n, k) mod prime p for 0 <= n, k < p.

        Assumes factorial tables for p have been precomputed.
        """
        if k < 0 or k > n:
            return 0
        if k == 0 or k == n:
            return 1

        if n >= p:
            # In our usage via Lucas' theorem, this should not occur; digits are < p.
            # Returning 0 mirrors behavior of the source, without asserting.
            if k > 0:
                return 0
            return 1

        k = min(k, n - k)
        fact_p = self.fact[p]
        inv_fact_p = self.inv_fact[p]
        return (
            fact_p[n]
            * inv_fact_p[k]
            % p
            * inv_fact_p[n - k]
            % p
        )


@numba.jit(nopython=True, cache=True)
def crt_three(a1: int, m1: int, a2: int, m2: int, a3: int, m3: int) -> int:
    """Combine three congruences using the Chinese Remainder Theorem.

    Solve for x such that:
      x = a1 (mod m1)
      x = a2 (mod m2)
      x = a3 (mod m3)

    Assumes moduli are pairwise coprime.
    """

    m12 = m1 * m2
    m = m12 * m3

    # Combine first two congruences.
    inv1 = mod_inverse(m1, m2)
    x12 = (a1 + m1 * ((a2 - a1) * inv1 % m2)) % m12

    # Combine result with third.
    inv12 = mod_inverse(m12, m3)
    x = (x12 + m12 * ((a3 - x12) * inv12 % m3)) % m
    return x


def lucas_mod(n: int, k: int, p: int, fact_cache: FactorialCache) -> int:
    """Compute C(n, k) modulo prime p via Lucas' theorem."""
    if k < 0 or k > n:
        return 0
    if k == 0:
        return 1

    n_digits: List[int] = []
    k_digits: List[int] = []

    nd = n
    kd = k
    while nd > 0 or kd > 0:
        n_digits.append(nd % p)
        k_digits.append(kd % p)
        nd //= p
        kd //= p

    max_len = max(len(n_digits), len(k_digits))
    n_digits.extend([0] * (max_len - len(n_digits)))
    k_digits.extend([0] * (max_len - len(k_digits)))

    result = 1
    for nd_digit, kd_digit in zip(n_digits, k_digits):
        if kd_digit > nd_digit:
            return 0
        c = fact_cache.binomial_small(nd_digit, kd_digit, p)
        result = (result * c) % p

    return result


def binomial_mod_n_k_mod_product(
    n: int,
    k: int,
    p: int,
    q: int,
    r: int,
    fact_cache: FactorialCache,
) -> int:
    """Compute C(n, k) modulo p*q*r using Lucas' theorem and CRT."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1

    ap = lucas_mod(n, k, p, fact_cache)
    aq = lucas_mod(n, k, q, fact_cache)
    ar = lucas_mod(n, k, r, fact_cache)

    return crt_three(ap, p, aq, q, ar, r)


def _simple_primes_up_to(limit: int) -> List[int]:
    """Return list of primes <= limit using a simple sieve.

    Standard-library-only replacement for Ruby's Prime.each.
    """

    if limit < 2:
        return []

    sieve = bytearray(b"\x01") * (limit + 1)
    sieve[0:2] = b"\x00\x00"

    p = 2
    while p * p <= limit:
        if sieve[p]:
            step = p
            start = p * p
            sieve[start : limit + 1 : step] = b"\x00" * (
                ((limit - start) // step) + 1
            )
        p += 1

    return [i for i in range(2, limit + 1) if sieve[i]]


def main() -> int:
    """Execute the Project Euler 365 computation and print progress info.

    Returns the final sum as an integer.
    """

    primes = [p for p in _simple_primes_up_to(MAX_PRIME) if p > MIN_PRIME]
    print(
        f"Generated {len(primes)} primes between {MIN_PRIME} and {MAX_PRIME}",
        flush=True,
    )

    fact_cache = FactorialCache()
    for p in primes:
        fact_cache.compute_for_prime(p)
    print("Precomputed factorials for all primes", flush=True)

    # KEY OPTIMIZATION: Precompute lucas_mod(N, K, p) for each prime once
    print("Precomputing lucas_mod values for all primes...", flush=True)
    lucas_values = {}
    for p in primes:
        lucas_values[p] = lucas_mod(N, K, p, fact_cache)
    print("Precomputation complete", flush=True)

    total_sum = 0
    count = 0
    start_time = perf_counter()

    # Triple nested loops with index management to ensure p < q < r.
    plen = len(primes)
    for i, p in enumerate(primes[:-2]):
        ap = lucas_values[p]
        for j in range(i + 1, plen - 1):
            q = primes[j]
            aq = lucas_values[q]
            for k_idx in range(j + 1, plen):
                r = primes[k_idx]
                ar = lucas_values[r]

                # Use precomputed lucas values with CRT
                result = crt_three(ap, p, aq, q, ar, r)
                total_sum += result
                count += 1

                if count % 100_000 == 0:
                    elapsed = perf_counter() - start_time
                    rate = count / elapsed if elapsed > 0 else float("inf")
                    print(
                        "Processed" f" {count} triplets, sum = {total_sum}, "
                        f"rate = {int(rate)}/sec",
                        flush=True,
                    )

    elapsed = perf_counter() - start_time
    avg_rate = count / elapsed if elapsed > 0 else float("inf")

    print(f"\nProcessed {count} triplets in {elapsed:.2f} seconds", flush=True)
    print(f"Average rate: {avg_rate:.0f} triplets/second", flush=True)
    print(f"Final result: {total_sum}", flush=True)

    return total_sum


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    result_ = main()
    print(f"\nThe final answer is: {result_}")
