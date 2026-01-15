"""Project Euler Problem 347 - Python 3.12 implementation.

This module computes S(N) as defined in the problem statement:
- For two distinct primes p and q, M(p, q, N) is the largest positive integer
  <= N that is divisible only by p and q (and uses both), or 0 if none exists.
- S(N) is the sum of all distinct M(p, q, N) over all distinct prime pairs.

Public API:
- build_spf
- get_prime_factors
- only_primes_p_and_q
- compute_m
- compute_s

The implementation is designed to be executable directly and also importable as a
module. When run as a script, it performs a small self-check.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, Iterable, List, Set


def build_spf(limit: int) -> List[int]:
    """Build the smallest prime factor (SPF) array up to ``limit``.

    Index ``i`` of the returned list holds the smallest prime factor of ``i``.
    Values for 0 and 1 are set to 0.
    """

    if limit < 2:
        return [0] * (limit + 1)

    spf = list(range(limit + 1))
    spf[0] = 0
    spf[1] = 0

    for i in range(2, isqrt(limit) + 1):
        if spf[i] != i:
            # Not a prime (already marked).
            continue

        step_start = i * i
        for j in range(step_start, limit + 1, i):
            if spf[j] == j:
                spf[j] = i

    return spf


def get_prime_factors(n: int, spf: List[int]) -> List[int]:
    """Return the distinct prime factors of ``n`` using a precomputed SPF table."""

    if n <= 1:
        return []

    factors: Set[int] = set()
    while n > 1:
        p = spf[n]
        if p == 0:
            # Happens only for invalid SPF tables or n out of range; we stop.
            break
        factors.add(p)
        while n % p == 0:
            n //= p

    return list(factors)


def only_primes_p_and_q(n: int, p: int, q: int, spf: List[int]) -> bool:
    """Return True if ``n`` has exactly the primes ``p`` and ``q`` as its
    distinct prime factors.
    """

    if n <= 0:
        return False

    factors = get_prime_factors(n, spf)
    return sorted(factors) == sorted((p, q))


def compute_m(p: int, q: int, limit: int, spf: List[int]) -> int:
    """Compute M(p, q, limit).

    M(p, q, N) is the largest integer ``<= limit`` divisible only by primes
    ``p`` and ``q`` (and by both of them). Returns 0 if no such integer exists.
    Assumes ``p`` and ``q`` are distinct primes.
    """

    lcm_pq = p * q
    if lcm_pq > limit:
        return 0

    max_k = limit // lcm_pq
    for k in range(max_k, 0, -1):
        candidate = k * lcm_pq
        if only_primes_p_and_q(candidate, p, q, spf):
            return candidate

    return 0


def _sieve_primes(limit: int) -> List[int]:
    """Return a list of all primes up to ``limit`` using a simple sieve."""

    if limit < 2:
        return []

    sieve = bytearray(b"\x01") * (limit + 1)
    sieve[0:2] = b"\x00\x00"
    for i in range(2, isqrt(limit) + 1):
        if sieve[i]:
            step_start = i * i
            sieve[step_start : limit + 1 : i] = b"\x00" * (
                ((limit - step_start) // i) + 1
            )

    return [i for i, is_prime in enumerate(sieve) if is_prime]


def compute_s(limit: int, *, verbose: bool = False) -> int:
    """Compute S(limit) as defined in the problem.

    When ``verbose`` is True, progress information is printed.
    """

    if limit < 2:
        return 0

    if verbose:
        print(f"Computing S({limit})...")
        print(f"Building SPF sieve up to {limit}...")

    spf = build_spf(limit)

    if verbose:
        print("Generating primes...")

    primes = _sieve_primes(limit)

    if verbose:
        print(f"Found {len(primes)} primes")
        print("Processing all pairs of distinct primes...")

    seen: Set[int] = set()
    total_sum = 0

    # Iterate over pairs (p, q) with p < q.
    num_primes = len(primes)
    for i in range(num_primes):
        p = primes[i]
        for j in range(i + 1, num_primes):
            q = primes[j]
            if p * q > limit:
                # Further q will only be larger.
                break
            m_val = compute_m(p, q, limit, spf)
            if m_val and m_val not in seen:
                seen.add(m_val)
                total_sum += m_val

    if verbose:
        print(
            f"Completed: found {len(seen)} unique M(p, q, {limit}) values;",
            f"S({limit}) = {total_sum}",
        )

    return total_sum


def _self_test() -> None:
    """Run basic self-tests when executed as a script."""

    expected_100 = 2262
    result_100 = compute_s(100, verbose=False)
    if result_100 != expected_100:
        raise RuntimeError(f"Self-test failed: S(100) = {result_100}, expected {expected_100}")


if __name__ == "__main__":  # pragma: no cover - manual execution entrypoint
    _self_test()
    result = compute_s(10_000_000, verbose=False)
    print(result)
