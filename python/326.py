"""Project Euler Problem 326 - Python 3.12 implementation.

This module provides tools to compute the function f(N, M) based on the
recursively defined sequence a_n:

    a_1 = 1
    a_n = (sum_{k=1}^{n-1} k * a_k) mod n

It includes small-input reference logic and a larger-input approach that
attempts to apply ideas analogous to the original Ruby code.

Note:
- The original Ruby used Prime.prime_division and a partially implemented
  "CRT-style" approach that is mathematically unclear and appears heuristic.
- This translation keeps the structure but does not claim a proven-fast
  solution for N=10**12, M=10**6. The main algorithm is written to match the
  Ruby logic as closely and clearly as possible.
- All public functions include docstrings and use type hints.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd, isqrt
from typing import Dict, Iterable, List, Sequence, Tuple

# ---------------------------------------------------------------------------
# Utility: integer factorization and Euler's totient using only stdlib
# ---------------------------------------------------------------------------


def prime_factorization(n: int) -> List[Tuple[int, int]]:
    """Return the prime factorization of n as a list of (prime, exponent).

    Uses trial division with small optimizations; sufficient for this context
    without external dependencies.
    """

    if n <= 1:
        return []

    factors: List[Tuple[int, int]] = []
    count = 0

    # Factor out 2s.
    while n % 2 == 0:
        n //= 2
        count += 1
    if count:
        factors.append((2, count))

    # Odd factors.
    p = 3
    limit = isqrt(n)
    while p <= limit and n > 1:
        if n % p == 0:
            count = 0
            while n % p == 0:
                n //= p
                count += 1
            factors.append((p, count))
            limit = isqrt(n)
        p += 2

    # Remaining prime.
    if n > 1:
        factors.append((n, 1))

    return factors


def euler_totient(n: int) -> int:
    """Compute Euler's totient function phi(n).

    For n <= 1, this returns 1 to match the Ruby behavior in the source.
    """

    if n <= 1:
        return 1

    phi = n
    for p, _ in prime_factorization(n):
        phi = phi // p * (p - 1)
    return phi


# ---------------------------------------------------------------------------
# Core sequence and small f(N, M)
# ---------------------------------------------------------------------------


def compute_a_sequence(limit: int) -> List[int]:
    """Compute the sequence a_n up to index ``limit`` (1-based, index 0 unused).

    a_1 = 1
    a_n = (sum_{k=1}^{n-1} k * a_k) mod n
    """

    if limit < 1:
        return []

    a: List[int] = [0] * (limit + 1)
    a[1] = 1
    running_weighted_sum = 0

    for n in range(2, limit + 1):
        running_weighted_sum += (n - 1) * a[n - 1]
        a[n] = running_weighted_sum % n

    return a


def compute_f_small(n: int, m: int) -> int:
    """Brute-force computation of f(n, m) using prefix sums.

    f(N, M) counts pairs (p, q) with 1 <= p <= q <= N such that
    sum_{i=p}^q a_i ≡ 0 (mod M).

    This implementation is for relatively small N and M.
    """

    if n < 1:
        return 0

    if m == 1:
        return n * (n + 1) // 2

    a = compute_a_sequence(n)

    prefix_sums: List[int] = [0] * (n + 1)
    for k in range(1, n + 1):
        prefix_sums[k] = prefix_sums[k - 1] + a[k]

    counts: List[int] = [0] * m
    for k in range(0, n + 1):
        remainder = prefix_sums[k] % m
        counts[remainder] += 1

    result = 0
    for count in counts:
        result += count * (count - 1) // 2

    return int(result)


# ---------------------------------------------------------------------------
# Matrix helpers (present in Ruby source but unused in its main logic)
# ---------------------------------------------------------------------------


def matrix_multiply(a: Sequence[Sequence[int]],
                    b: Sequence[Sequence[int]],
                    mod: int) -> List[List[int]]:
    """Multiply two square matrices ``a`` and ``b`` modulo ``mod``.

    Provided because the Ruby source defined it. Currently unused by main.
    """

    n = len(a)
    result: List[List[int]] = [[0] * n for _ in range(n)]

    for i in range(n):
        for k in range(n):
            aik = a[i][k]
            if aik == 0:
                continue
            for j in range(n):
                result[i][j] = (result[i][j] + aik * b[k][j]) % mod

    return result


def matrix_power(matrix: Sequence[Sequence[int]],
                 exponent: int,
                 mod: int) -> List[List[int]]:
    """Fast exponentiation of a square matrix modulo ``mod``.

    Included as a translation of the Ruby helper. Not used by main.
    """

    n = len(matrix)
    # Identity matrix
    result: List[List[int]] = [[0] * n for _ in range(n)]
    for i in range(n):
        result[i][i] = 1

    base = [list(row) for row in matrix]
    e = exponent

    while e > 0:
        if e & 1:
            result = matrix_multiply(result, base, mod)
        base = matrix_multiply(base, base, mod)
        e >>= 1

    return result


def get_state_transition(m: int) -> int:
    """Placeholder for state transition logic.

    The original Ruby function simply returned ``m`` and did not implement
    any transition matrix or similar structure. If a more sophisticated
    Markov/automaton approach is desired, implement and return a transition
    description here.
    """

    return m


# ---------------------------------------------------------------------------
# Heuristic/CRT-based contribution computation
# ---------------------------------------------------------------------------


def compute_contribution_for_modulus(n: int, m: int, max_a: int) -> int:
    """Compute contribution for a given modulus.

    This mirrors the structure of the Ruby implementation, which mixes
    small-range exact computation with a heuristic extension using an assumed
    periodic behavior of the sequence modulo m.

    Note:
    - The logic is preserved for fidelity, but the mathematical justification
      is not documented in the original source and may not be fully correct
      for very large N.
    """

    if n <= max_a:
        return compute_f_small(n, m)

    base_result = compute_f_small(max_a, m)
    remaining = n - max_a

    # The Ruby code sets ``states = m`` but does not use it; omitted here.

    # Approximate idea retained but unused; kept for clarity of translation.
    # approx_contribution = remaining * (remaining + 1) / 2 * (1.0 / m)

    period = euler_totient(m)
    full_periods = remaining // period
    remainder_terms = remaining % period

    # Build sequence up to max_a + period to probe periodic behavior.
    period_a = compute_a_sequence(max_a + period)
    period_prefix: List[int] = [0] * (max_a + period + 1)
    for k in range(1, max_a + period + 1):
        period_prefix[k] = (period_prefix[k - 1] + period_a[k]) % m

    # Count prefix remainders within one "period" segment beyond max_a.
    period_counts: List[int] = [0] * m
    for k in range(max_a + 1, max_a + period + 1):
        period_counts[period_prefix[k]] += 1

    period_contribution = 0
    for count in period_counts:
        period_contribution += count * (count - 1) // 2
    period_contribution *= full_periods

    # Handle remaining terms after full periods.
    remainder_contribution = 0
    current_counts = period_counts.copy()

    # The Ruby code constructs new_prefix using a somewhat opaque formula.
    # We replicate its behavior literally, though its correctness is unclear.
    for k in range(1, remainder_terms + 1):
        new_prefix = (
            period_prefix[max_a + k] + period_prefix[max_a + k - 1]
        ) % m
        remainder_contribution += current_counts[new_prefix]
        current_counts[new_prefix] += 1

    total = base_result + period_contribution + remainder_contribution
    return int(total)


def compute_f_crt(n: int, m: int) -> int:
    """Attempt to compute f(n, m) via factorization and combined contributions.

    This function follows the structure of the Ruby version, which:
    - Factors m into prime powers.
    - Computes a partial contribution for each prime power via
      ``compute_contribution_for_modulus``.
    - Combines those contributions with a heuristic adjustment.

    Important:
    - The combination method in the original Ruby code is not a standard
      Chinese Remainder Theorem reconstruction for counts, and its
      mathematical validity is uncertain.
    - This translation preserves that behavior for compatibility. Users
      needing a provably correct and efficient solution for very large n
      should replace this with a rigorously justified algorithm.
    """

    if m == 1:
        return n * (n + 1) // 2

    factors = prime_factorization(m)
    prime_powers = [p ** e for p, e in factors]

    total_contributions: List[int] = [
        compute_contribution_for_modulus(n, pp, m) for pp in prime_powers
    ]

    max_contribution = max(total_contributions)
    # Ruby: adjustment = sum - max * (len - 1)
    adjustment = sum(total_contributions) - max_contribution * (
        len(prime_powers) - 1
    )

    # Ruby: result = max_contribution + adjustment / len
    result = max_contribution + adjustment // len(prime_powers)
    return int(result)


# ---------------------------------------------------------------------------
# Main entry point (with basic validation as in the Ruby script)
# ---------------------------------------------------------------------------


@dataclass
class Problem326Config:
    """Configuration for the main Problem 326 computation."""

    n: int = 10**12
    m: int = 10**6
    max_a: int = 10**6


def run_validation_tests() -> None:
    """Run small sanity checks mirroring the Ruby script's tests."""

    test1 = compute_f_small(1, 1)
    if test1 != 1:
        raise AssertionError(f"Test 1 failed: got {test1}, expected 1")

    test2 = compute_f_small(10, 10)
    if test2 != 4:
        raise AssertionError(f"Test 2 failed: got {test2}, expected 4")

    test3 = compute_f_small(10**4, 10**3)
    if test3 != 97_158:
        raise AssertionError(f"Test 3 failed: got {test3}, expected 97_158")


def main(config: Problem326Config | None = None) -> int:
    """Compute f(N, M) using the translated heuristic approach.

    Includes validation tests before the main computation.
    """

    if config is None:
        config = Problem326Config()

    run_validation_tests()

    n, m, max_a = config.n, config.m, config.max_a
    result = compute_f_crt(n, m)
    print(f"f({n}, {m}) = {result}")

    # Print only final answer for test harness
    print()
    print(result)

    return result


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    main()
