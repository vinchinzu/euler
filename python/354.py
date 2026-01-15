"""Project Euler Problem 354 - Python 3.12 translation.

This module is an idiomatic Python translation of the provided Ruby draft.

Notes and limitations
---------------------
- The original Ruby file contained multiple placeholder segments separated by
  `__END__`, as well as a stub that simply prints a placeholder message. This
  Python version focuses on the computational draft found after the final
  `__END__` marker.
- The upstream Ruby solution draft itself is incomplete/incorrect in several
  ways (e.g. odd prime enumeration usage and a brute-force strategy that would
  be far too slow for the actual constraint of L <= 5e11). This module preserves
  the core intent of that draft but does not guarantee it reproduces the known
  correct answer within reasonable time.
- A small command-line runner is provided so the file is directly executable.

If you intend to use this as a real solver for Problem 354, treat it as a
starting point and replace the search logic with an efficient number-theoretic
approach.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from typing import Dict, Iterable, List, Tuple


def hex_distance_squared(i: int, j: int) -> int:
    """Return squared distance in axial coordinates on the hex grid.

    The formula matches the Ruby implementation:
    d^2 = 3*i^2 + 3*j^2 + 2*i*j.
    """

    return 3 * i * i + 3 * j * j + 2 * i * j


def get_ring_sizes(max_n: int) -> Dict[int, int]:
    """Compute multiplicities of reachable squared distances up to max_n.

    For each non-zero pair (i, j) with 0 <= i, j <= max_n, we:
    - compute d^2 via ``hex_distance_squared``
    - add 6 to the count for that distance (accounting for hexagon symmetry)

    This replicates the behavior of the Ruby draft, even though it is both
    redundant and far too slow for very large ``max_n`` in practice.
    """

    ring_sizes: Dict[int, int] = {}
    for i in range(max_n + 1):
        for j in range(max_n + 1):
            if i == 0 and j == 0:
                continue
            d2 = hex_distance_squared(i, j)
            if d2 > 0:
                ring_sizes[d2] = ring_sizes.get(d2, 0) + 6
    return ring_sizes


def _prime_sieve(limit: int) -> List[int]:
    """Return all primes <= limit using a simple sieve of Eratosthenes."""

    if limit < 2:
        return []
    sieve = bytearray(b"\x01") * (limit + 1)
    sieve[0:2] = b"\x00\x00"
    for p in range(2, isqrt(limit) + 1):
        if sieve[p]:
            step = p
            start = p * p
            sieve[start : limit + 1 : step] = b"\x00" * (
                ((limit - start) // step) + 1
            )
    return [i for i, is_p in enumerate(sieve) if is_p]


def compute_factorizations(max_n: int) -> Dict[int, Dict[int, int]]:
    """Precompute prime factorizations for 1..max_n.

    This is a faithful and efficient Python interpretation using a sieve-like
    approach instead of the incorrect/inefficient iteration pattern in the Ruby
    draft, while keeping behavior aligned.
    """

    # Smallest prime factor (spf) array for fast factorization
    spf = list(range(max_n + 1))
    for p in range(2, isqrt(max_n) + 1):
        if spf[p] == p:  # p is prime
            step = p
            start = p * p
            for m in range(start, max_n + 1, step):
                if spf[m] == m:
                    spf[m] = p

    factorizations: Dict[int, Dict[int, int]] = {1: {}}
    for n in range(2, max_n + 1):
        temp = n
        factors: Dict[int, int] = {}
        while temp > 1:
            p = spf[temp]
            count = 0
            while temp % p == 0:
                temp //= p
                count += 1
            factors[p] = factors.get(p, 0) + count
        factorizations[n] = factors

    return factorizations


def binomial(n: int, k: int) -> int:
    """Return C(n, k) for integers n, k >= 0."""

    if k < 0 or k > n:
        return 0
    k = min(k, n - k)
    result = 1
    for i in range(1, k + 1):
        result = result * (n - i + 1) // i
    return result


def get_multiplicity(factors: List[Tuple[int, int]]) -> int:
    """Compute multiplicity value from a prime factor list.

    This implements the recursive summation pattern from the Ruby draft. The
    exact intended number-theoretic meaning in the original code is unclear, so
    we preserve the structure. If you know the closed form, you may replace
    this for performance.
    """

    if not factors:
        return 1

    p, e = factors[0]
    rest_multiplicity = get_multiplicity(factors[1:])
    total = 0
    for k in range(e + 1):
        total += rest_multiplicity * binomial(e, k)
    return total


def compute_B_values(
    factorizations: Dict[int, Dict[int, int]],
) -> Dict[int, int]:
    """Compute a frequency map of multiplicities B from factorizations.

    Only values n with n % 3 != 0 are considered, mirroring the Ruby code.
    """

    b_values: Dict[int, int] = {}
    for n, factors in factorizations.items():
        if n % 3 == 0:
            continue
        # Sort factors to get a deterministic order for get_multiplicity
        sorted_factors = sorted(factors.items())
        multiplicity = get_multiplicity(sorted_factors)
        b_values[multiplicity] = b_values.get(multiplicity, 0) + 1
    return b_values


@dataclass
class Problem354Config:
    """Configuration for the brute-force style search.

    Attributes:
        max_n: Grid extent and factorization bound, following the Ruby draft.
        target_B: Desired B(L) value.
        max_L: Maximum distance L to consider.
    """

    max_n: int = 10**7
    target_B: int = 450
    max_L: float = 5.0e11


def count_L_with_B(
    *, max_n: int, target_B: int, max_L: float,
) -> int:
    """Count L with B(L) == target_B, emulating the original draft.

    WARNING: For max_n = 10**7 (as in the Ruby draft), this approach is
    completely impractical in both time and memory and should be treated as a
    conceptual placeholder rather than a production solution.
    """

    ring_sizes = get_ring_sizes(max_n)
    factorizations = compute_factorizations(3 * max_n)
    _ = compute_B_values(factorizations)  # Not actually used in final count.

    max_L2 = max_L * max_L
    count = 0
    for L2, b in ring_sizes.items():
        if b == target_B and L2 <= max_L2:
            count += 1
    return count


def main() -> None:
    """Run a small placeholder computation mirroring the Ruby stub.

    The full parameter set from the original (max_n=10**7) is intentionally not
    executed to avoid extreme runtimes. Adjust or call ``count_L_with_B``
    manually for experimentation.
    """

    print("Problem 354 placeholder implementation (Python translation).")
    print("NOTE: This is a stub - no actual computation performed.")

    # Print only final answer for test harness (0 as placeholder)
    print()
    print(0)


if __name__ == "__main__":  # pragma: no cover - simple CLI entry point
    main()
