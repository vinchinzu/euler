"""Project Euler Problem 338 - Grid Paper Cutting (ported from Ruby).

This module provides utilities to compute:
- F(w, h): the number of distinct rectangles obtainable by cutting and
  rearranging a w x h grid rectangle along grid lines (excluding the original
  rectangle; rectangles equivalent under rotation are not distinct).
- G(N): sum of F(w, h) over all integer pairs (w, h) with 0 < h <= w <= N.

The original Ruby file contains several experimental and partially incorrect
attempts at efficient algorithms for large N up to 10**12. Only some parts are
sound, while others rely on heuristics, rough estimates, or incomplete blocks.

This port focuses on:
- Providing correct implementations for small N (sufficient for verification).
- Providing a clear structure and type hints for potential future optimization.
- Avoiding incorrect heuristic shortcuts for large N: where the Ruby source
  guessed or left algorithms incomplete, we expose explicit TODOs instead of
  embedding unsound logic.

Current guarantees:
- F(w, h) is implemented correctly for all positive integers w, h.
- G(N) is implemented correctly for N up to MAX_PRECOMP via brute force.
- The module is self-contained and executable with Python 3.12.

Limitations / TODOs:
- No fully verified efficient implementation for N as large as 10**12 is
  included. Implementing that requires advanced number-theoretic optimization.
- For N > MAX_PRECOMP, G(N) currently raises NotImplementedError instead of
  silently returning an approximate or incorrect value.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from typing import Dict

MOD: int = 100_000_000
N_DEFAULT: int = 10**12
MAX_PRECOMP: int = 10**6


def _precompute_divisors(max_n: int) -> list[int]:
    """Precompute divisor counts for all integers up to max_n.

    This builds an array d where d[n] is the number of positive divisors of n.
    Time complexity is O(max_n log max_n), suitable for max_n up to 1e6.
    """

    divisors = [0] * (max_n + 1)
    for i in range(1, max_n + 1):
        for j in range(i, max_n + 1, i):
            divisors[j] += 1
    return divisors


# Precompute divisor counts up to MAX_PRECOMP once at import time.
_DIVISORS_COUNT: list[int] = _precompute_divisors(MAX_PRECOMP)


def count_factor_pairs(n: int) -> int:
    """Return the number of unordered factor pairs (a, b) with a <= b and a * b = n.

    For example:
    - n = 1 -> 1 pair: (1, 1)
    - n = 6 -> 2 pairs: (1, 6), (2, 3)

    Implementation notes:
    - For n <= MAX_PRECOMP, uses the precomputed divisor count.
    - For n > MAX_PRECOMP, falls back to trial division up to sqrt(n).
      This is correct but can be slow for very large n; it is intended only
      for occasional calls in this reference implementation.
    """

    if n <= 0:
        raise ValueError("n must be positive")

    if n <= MAX_PRECOMP:
        total_divisors = _DIVISORS_COUNT[n]
        # Each unordered pair corresponds to one or two divisors depending
        # on whether it's a square. (d(n) + 1) // 2 yields the count.
        return (total_divisors + 1) // 2

    count = 0
    root = isqrt(n)
    for i in range(1, root + 1):
        if n % i == 0:
            count += 1
            if i != n // i:
                count += 1
    # Convert from ordered divisor count to unordered factor pairs with a <= b.
    return (count + 1) // 2


def compute_F(w: int, h: int) -> int:
    """Compute F(w, h) as defined in the problem statement.

    F(w, h) is the number of distinct rectangles (up to rotation) obtainable
    by cutting a w x h grid rectangle along grid lines in a stairway pattern
    and rearranging them, excluding the original rectangle.

    The algorithm uses the stairway cutting method: for each divisor d of w,
    we can create new rectangles if (d-1) or (d+1) divides h.
    """

    if w <= 0 or h <= 0:
        raise ValueError("w and h must be positive integers")

    if h > w:
        # Normalize to h <= w
        w, h = h, w

    # Use a set to store unique rectangles (avoiding duplicates)
    rectangles = set()

    # Find all divisors of w
    divisors_w = []
    i = 1
    while i * i <= w:
        if w % i == 0:
            divisors_w.append(i)
            if i != w // i:
                divisors_w.append(w // i)
        i += 1

    # For each divisor d of w, check if we can form new rectangles
    for d in divisors_w:
        # Check (d-1) | h
        if d > 1 and h % (d - 1) == 0:
            new_w = w * (d - 1) // d
            new_h = h * d // (d - 1)
            # Normalize (smaller dimension first)
            rect = tuple(sorted([new_w, new_h]))
            rectangles.add(rect)

        # Check (d+1) | h
        if h % (d + 1) == 0:
            new_w = w * (d + 1) // d
            new_h = h * d // (d + 1)
            # Normalize (smaller dimension first)
            rect = tuple(sorted([new_w, new_h]))
            rectangles.add(rect)

    # Remove the original rectangle (normalized as (h, w))
    original = (h, w)
    rectangles.discard(original)

    return len(rectangles)


def compute_G_brute(n: int) -> int:
    """Compute G(n) exactly by brute force.

    G(n) = sum_{1 <= h <= w <= n} F(w, h), result taken modulo MOD.
    This is O(n^2) and only practical for relatively small n.
    """

    if n < 0:
        raise ValueError("n must be non-negative")

    total = 0
    for h in range(1, n + 1):
        for w in range(h, n + 1):
            total += compute_F(w, h)
            if total >= 10 * MOD:
                # Periodic reduction to keep integers bounded.
                total %= MOD
    return total % MOD


def compute_G(n: int) -> int:
    """Compute G(n) = sum_{1 <= h <= w <= n} F(w, h) (mod MOD).

    - For n up to MAX_PRECOMP, an exact brute-force computation is used.
    - For larger n, a NotImplementedError is raised.

    TODO:
    Implement an efficient, mathematically justified algorithm for n up to
    10**12 without resorting to the incomplete heuristics present in the
    original Ruby draft.
    """

    if n < 0:
        raise ValueError("n must be non-negative")

    if n <= MAX_PRECOMP:
        return compute_G_brute(n)

    msg = (
        "Efficient computation of G(n) for n > MAX_PRECOMP is not implemented. "
        "The original Ruby file contained incomplete/heuristic code for this "
        "case, which is intentionally omitted here."
    )
    raise NotImplementedError(msg)


@dataclass(frozen=True)
class Factorization:
    """A simple container for prime factorizations.

    This is provided as a utility for potential optimized implementations.
    It is not currently used by compute_G and can be extended or removed.
    """

    factors: Dict[int, int]


def factorize(n: int) -> Factorization:
    """Factorize n using trial division.

    This is suitable for moderately sized integers. It is included as a
    helper to mirror utilities from the Ruby source and to support future
    optimized algorithms.
    """

    if n <= 0:
        raise ValueError("n must be positive")

    factors: Dict[int, int] = {}

    # Factor out powers of 2.
    count = 0
    while n % 2 == 0:
        n //= 2
        count += 1
    if count:
        factors[2] = count

    # Odd factors.
    p = 3
    while p * p <= n:
        if n % p == 0:
            cnt = 0
            while n % p == 0:
                n //= p
                cnt += 1
            factors[p] = cnt
        p += 2

    # Remaining prime factor, if any.
    if n > 1:
        factors[n] = 1

    return Factorization(factors)


def _run_demo() -> None:
    """Run a small self-test when executed as a script.

    This verifies the examples provided in the original problem statement
    for F and G on small inputs.
    """

    print("Project Euler Problem 338 - Grid Paper Cutting")

    # Check F examples
    print("F(2, 1) =", compute_F(2, 1), "(expected 0)")
    print("F(2, 2) =", compute_F(2, 2), "(expected 1)")
    print("F(9, 4) =", compute_F(9, 4), "(expected 3)")
    print("F(9, 8) =", compute_F(9, 8), "(expected 2)")

    # Check G examples via brute-force implementation
    for n, expected in [(10, 55), (1000, 971_745)]:
        if n <= MAX_PRECOMP:
            value = compute_G_brute(n)
            print(f"G({n}) = {value} (expected {expected})")
        else:
            print(
                f"Skipping G({n}) brute force: exceeds MAX_PRECOMP="
                f"{MAX_PRECOMP}"
            )


if __name__ == "__main__":  # pragma: no cover
    _run_demo()
