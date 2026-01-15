"""Project Euler Problem 381: Sum of S(p) for primes.

This module provides an efficient Python implementation for computing

    S(p) = sum((p - k)!) mod p for 1 <= k <= 5

for prime p, and summing S(p) over a given prime range.

Key ideas:
- Uses Wilson's theorem to express (p - k)! modulo p in terms of a small
  factorial and modular inverses, giving O(1) work per prime.
- Includes a small-range self-check against the value given in the
  original problem statement.

The main entry point is `main()`, which can be invoked directly.
"""

from __future__ import annotations

import sys
import time
from typing import Iterable, List


LIMIT: int = 100_000_000
MIN_P: int = 5
SMALL_FACTS: List[int] = [1, 1, 2, 6, 24]  # 0! to 4!
VALIDATION_LIMIT: int = 100
EXPECTED_SMALL_SUM: int = 480  # Given in problem statement


def mod_inverse(a: int, p: int) -> int | None:
    """Return modular inverse of ``a`` modulo prime ``p`` using the
    extended Euclidean algorithm.

    Returns ``None`` if ``a`` is 0 or if no inverse exists.
    """

    if a == 0:
        return None

    t, new_t = 0, 1
    r, new_r = p, a % p

    while new_r != 0:
        quotient = r // new_r
        t, new_t = new_t, t - quotient * new_t
        r, new_r = new_r, r - quotient * new_r

    if r > 1:
        return None

    return t % p


def compute_s_p(p: int) -> int:
    """Compute S(p) for a prime ``p``.

    Uses Wilson's theorem to derive the simplified formula:
    S(p) = (p - 3) × inverse(8, p) mod p

    For p < 5, returns 0 (these are excluded in the problem constraints).
    """

    if p < 5:
        return 0

    inv8 = mod_inverse(8, p)
    if inv8 is None:
        # This shouldn't happen for primes p >= 5
        return 0

    return ((p - 3) * inv8) % p


def _primes_upto(limit: int) -> Iterable[int]:
    """Generate primes up to ``limit`` using a memory-efficient sieve.

    Only odd numbers are stored to reduce memory footprint.
    """

    if limit >= 2:
        yield 2

    if limit < 3:
        return

    size = (limit - 1) // 2
    sieve = bytearray(b"\x01") * size

    from math import isqrt

    for i in range(isqrt(limit) // 2 + 1):
        if sieve[i]:
            step = 2 * i + 3
            start = (step * step - 3) // 2
            sieve[start::step] = b"\x00" * ((size - start - 1) // step + 1)

    for i, is_prime in enumerate(sieve):
        if is_prime:
            yield 2 * i + 3


def validate_small_sum() -> None:
    """Validate S(p) on a small range of primes.

    Checks that sum S(p) for 5 <= p < VALIDATION_LIMIT matches
    EXPECTED_SMALL_SUM. Raises ValueError if validation fails.
    """

    small_primes = [p for p in _primes_upto(VALIDATION_LIMIT) if p >= MIN_P]
    computed_sum = sum(compute_s_p(p) for p in small_primes)

    if computed_sum == EXPECTED_SMALL_SUM:
        print(
            " Validation passed: Sum for"
            f" {MIN_P} <= p < {VALIDATION_LIMIT} = {computed_sum}"
            f" (expected {EXPECTED_SMALL_SUM})"
        )
    else:
        raise ValueError(
            "Validation failed: Computed"
            f" {computed_sum}, expected {EXPECTED_SMALL_SUM}"
        )


def compute_total_sum(limit: int = LIMIT, min_p: int = MIN_P) -> int:
    """Compute sum of S(p) for primes in ``[min_p, limit)``.

    This is the core computation used by :func:`main` and can be reused.
    """

    total_sum = 0

    for p in _primes_upto(limit - 1):
        if p < min_p:
            continue
        total_sum += compute_s_p(p)

    return total_sum


def main(argv: list[str] | None = None) -> int:
    """CLI entry point - prints only the final numeric answer.

    Optionally accepts one argument: the upper limit (exclusive).
    Defaults to ``LIMIT`` if not provided.
    """

    if argv is None:
        argv = sys.argv[1:]

    try:
        limit = int(argv[0]) if argv else LIMIT
    except ValueError:
        print("Invalid limit; expected integer.", file=sys.stderr)
        return 1

    if limit < MIN_P:
        print(f"Limit must be >= {MIN_P}.", file=sys.stderr)
        return 1

    total_sum = compute_total_sum(limit, MIN_P)
    print(total_sum)

    return 0


if __name__ == "__main__":  # pragma: no cover - CLI guard
    raise SystemExit(main())
