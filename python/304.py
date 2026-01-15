"""Project Euler Problem 304 solver in Python 3.12.

This module computes

    sum_{n=1..100000} Fibonacci(a(n)) mod 1234567891011

where a(1) is the smallest prime greater than 10**14 and a(n) is the next
prime after a(n-1). Fibonacci(k) is the standard sequence with F(0) = 0,
F(1) = 1.

The implementation is standalone (standard library only) and optimized via

- Miller-Rabin primality testing with deterministic bases suitable for the
  searched range.
- Fast doubling method for Fibonacci modulo a given integer.

Public API:
- solve() -> int: compute and return the requested sum.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, List, Tuple
import math
import numpy as np

MOD: int = 1_234_567_891_011
N: int = 100_000
START: int = 10**14
MAX_SEARCH_LIMIT: int = 10_000  # For fallback, but not used now

# Deterministic witnesses for Miller-Rabin up to at least 2^64.
# This comfortably covers the range we care about (values around 1e14).
MILLER_RABIN_WITNESSES: Tuple[int, ...] = (
    2,
    3,
    5,
    7,
    11,
    13,
    17,
    19,
    23,
    29,
    31,
    37,
)

SMALL_PRIME_LIMIT: int = 10**7 + 1000
small_primes: List[int] = []

def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes with NumPy."""
    if limit < 2:
        return []
    is_prime = np.ones(limit + 1, dtype=bool)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(np.sqrt(limit)) + 1):
        if is_prime[i]:
            is_prime[i * i::i] = False
    return np.nonzero(is_prime)[0].tolist()

def segmented_sieve(low: int, high: int, small_primes: List[int]) -> List[int]:
    """Generate primes in [low, high) using segmented sieve with NumPy vectorization."""
    size = high - low
    if size <= 0:
        return []
    sieve = np.ones(size, dtype=bool)
    for p in small_primes:
        if p * p > high:
            break
        remainder = low % p
        start_idx = (p - remainder) % p
        sieve[start_idx::p] = False
    # Handle the case where low might be marked if low == 0 or 1, but low large
    return [low + i for i in np.nonzero(sieve)[0]]


@dataclass(frozen=True)
class FibModResult:
    """Container for Fibonacci computation results.

    Attributes:
        value: Fibonacci(n) modulo `mod`.
        index: The index `n` used for the computation.
    """

    value: int
    index: int


def mod_pow(base: int, exponent: int, mod: int) -> int:
    """Efficient modular exponentiation.

    Computes (base ** exponent) % mod using binary exponentiation.
    """

    result = 1
    base %= mod
    while exponent > 0:
        if exponent & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exponent >>= 1
    return result


def _decompose_n_minus_one(n: int) -> Tuple[int, int]:
    """Write n-1 as d * 2**s with d odd.

    Returns (d, s).
    """

    d = n - 1
    s = 0
    while d % 2 == 0:
        d //= 2
        s += 1
    return d, s


def is_probable_prime(n: int) -> bool:
    """Return True if n is prime using deterministic Miller-Rabin for 64-bit ints.

    For n < 2, returns False. For our search range (around 1e14) and below
    2**64, this is deterministic with the chosen witnesses.
    """

    if n < 2:
        return False
    if n in (2, 3):
        return True
    if n % 2 == 0:
        return False

    d, s = _decompose_n_minus_one(n)

    for a in MILLER_RABIN_WITNESSES:
        if a >= n:
            continue

        x = mod_pow(a, d, n)
        if x == 1 or x == n - 1:
            continue

        for _ in range(s - 1):
            x = (x * x) % n
            if x == n - 1:
                break
        else:
            return False

    return True


def next_prime_after(n: int) -> int:
    """Return the smallest prime strictly greater than n.

    The search is limited by MAX_SEARCH_LIMIT * 2 steps (odd candidates only).
    Raises RuntimeError if no prime is found within that window to help
    surface potential logic issues or unexpectedly large prime gaps.
    """

    candidate = n + 1
    if candidate % 2 == 0 and candidate > 2:
        candidate += 1

    # We try at most MAX_SEARCH_LIMIT odd candidates.
    for _ in range(MAX_SEARCH_LIMIT):
        if is_probable_prime(candidate):
            return candidate
        candidate += 2

    msg = (
        f"Could not find next prime after {n} within {MAX_SEARCH_LIMIT} "
        "odd candidates. This may indicate a bug or an unusually large "
        "prime gap for the configured limit."
    )
    raise RuntimeError(msg)


def fib_mod_fast_doubling(n: int, mod: int) -> int:
    """Compute Fibonacci(n) modulo mod using fast doubling.

    This is significantly faster than matrix exponentiation for very large n.
    """

    if n < 0:
        raise ValueError("Fibonacci index must be non-negative")

    def _fib_pair(k: int) -> Tuple[int, int]:
        if k == 0:
            return 0, 1
        a, b = _fib_pair(k >> 1)
        c = (a * ((b * 2 - a) % mod)) % mod
        d = (a * a + b * b) % mod
        if k & 1:
            return d, (c + d) % mod
        return c, d

    return _fib_pair(n)[0] % mod


def generate_primes_after(start: int, count: int) -> List[int]:
    """Generate a list of `count` consecutive primes greater than `start`."""
    # Find the first prime > start using Miller-Rabin
    candidate = start + 1 if start % 2 == 0 else start + 2
    while not is_probable_prime(candidate):
        candidate += 2
    first_prime = candidate

    # Estimate range needed
    ln_p = math.log(first_prime)
    delta = int(count * ln_p * 1.2)  # Safety margin
    high = first_prime + delta

    # Generate small primes if not already
    global small_primes
    if not small_primes:
        small_primes = sieve(SMALL_PRIME_LIMIT)

    segment_primes = segmented_sieve(first_prime, high, small_primes)
    if len(segment_primes) < count:
        # Increase delta if needed (rare)
        raise ValueError("Increase delta or check implementation")
    return segment_primes[:count]


def compute_sum_fib_on_primes(
    primes: Iterable[int], mod: int = MOD
) -> int:
    """Return sum(Fibonacci(p) for p in primes) modulo mod."""

    total = 0
    for p in primes:
        total = (total + fib_mod_fast_doubling(p, mod)) % mod
    return total


def solve() -> int:
    """Compute and return the Project Euler 304 answer modulo MOD.

    This function is the main public entry point for programmatic use.
    """

    primes = generate_primes_after(START, N)
    return compute_sum_fib_on_primes(primes, MOD)


def _main() -> None:
    """CLI entry point mirroring the original Ruby script's behavior."""

    try:
        result = solve()
    except Exception as exc:  # pragma: no cover - defensive CLI handling
        print("Error:", exc)
        raise
    else:
        print(result)


if __name__ == "__main__":  # pragma: no cover - manual execution guard
    _main()
