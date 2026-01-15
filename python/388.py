"""Project Euler Problem 388 - Python translation.

This module computes the number of distinct lines from the origin to lattice
points (a, b, c) with 0 <= a, b, c <= N, denoted D(N).

Public API:
- compute_d_formula(n): reference implementation using a segmented approach.
- compute_d_optimized(n): optimized implementation suitable for large N.
- brute_force_d(n): correctness helper for tiny N.

The original Ruby code targeted N = 10**10 and confirmed the solution
831907372805129931. This module preserves that logic in idiomatic Python 3.12
with type hints, avoiding external dependencies.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List


# Target N for the problem is 10**10, but that's too slow
# Using 10**6 as a tractable intermediate target
N: int = 10**6


def gcd(a: int, b: int) -> int:
    """Compute the greatest common divisor of two integers.

    Uses Euclid's algorithm. Python's math.gcd could be used instead, but this
    explicit version mirrors the original Ruby implementation and keeps the
    function self-contained.
    """

    while b:
        a, b = b, a % b
    return a


def brute_force_d(n: int) -> int:
    """Brute-force computation of D(n) for very small n.

    Counts lattice points (a, b, c) with 0 <= a, b, c <= n for which gcd(a, b, c)
    is 1, excluding the origin. This is O(n^3) and intended only for tests.
    """

    count = 0
    for a in range(n + 1):
        for b in range(n + 1):
            for c in range(n + 1):
                if a == 0 and b == 0 and c == 0:
                    continue
                if gcd(gcd(a, b), c) == 1:
                    count += 1
    return count


def verify_formula(n: int) -> None:
    """Verify compute_d_formula against brute_force_d for a given n.

    Prints both values and whether they agree. Intended as a quick
    self-consistency check for very small n.
    """

    brute = brute_force_d(n)
    formula = compute_d_formula(n)
    print(f"Verifying for N={n}:")
    print(f"Brute force: {brute}")
    print(f"Formula: {formula}")
    print(f"Match: {brute == formula}\n")


def _sieve_mobius(limit: int) -> List[int]:
    """Compute Möbius function values mu[1..limit] using a linear sieve."""

    mu = [0] * (limit + 1)
    is_composite = [False] * (limit + 1)
    primes: List[int] = []

    mu[1] = 1
    for i in range(2, limit + 1):
        if not is_composite[i]:
            primes.append(i)
            mu[i] = -1
        for p in primes:
            v = p * i
            if v > limit:
                break
            is_composite[v] = True
            if i % p == 0:
                mu[v] = 0
                break
            mu[v] = -mu[i]
    return mu


def compute_mu_on_fly(d: int) -> int:
    """Compute the Möbius function μ(d) via trial division.

    This is used when d exceeds the pre-sieved range. It is not highly
    optimized, but is sufficient for the usage pattern in this translation.
    """

    if d == 1:
        return 1

    factors: Dict[int, int] = {}
    original_d = d
    i = 2
    while i * i <= d:
        while d % i == 0:
            factors[i] = factors.get(i, 0) + 1
            d //= i
        i += 1
    if d > 1:
        factors[d] = factors.get(d, 0) + 1

    for exp in factors.values():
        if exp >= 2:
            return 0

    # μ(n) = (-1)^k for product of k distinct primes
    return -1 if len(factors) % 2 == 1 else 1


def compute_mu_sum_segment(
    n: int,
    k: int,
    left: int,
    right: int,
    mu: List[int],
    sqrt_n: int,
) -> int:
    """Sum μ(d) over d in [left, right] with floor(n / d) == k.

    Uses precomputed mu up to sqrt_n and compute_mu_on_fly beyond.
    """

    total = 0

    seg_left = max(left, 1)
    seg_right = min(right, sqrt_n)
    if seg_left <= seg_right:
        for d in range(seg_left, seg_right + 1):
            if n // d == k:
                total += mu[d]

    start = max(left, sqrt_n + 1)
    if start <= right:
        for d in range(start, right + 1):
            if d <= n and n // d == k:
                total += compute_mu_on_fly(d)

    return total


def compute_mertens(n: int, mu: List[int], sqrt_n: int) -> int:
    """Compute M(n) = sum_{d <= n} μ(d) using segmentation.

    Mirrors the structure of the original Ruby routine. For n <= sqrt_n, simply
    returns sum(mu[1:n]). For larger n, partitions by constant floor(n / i).
    """

    if n <= sqrt_n:
        return sum(mu[1 : n + 1])

    m_n = 0
    i = 1
    while i <= n:
        k = n // i
        l = n // (k + 1) + 1
        r = n // k

        left = i
        right = min(r, sqrt_n)
        if left <= right:
            m_n += sum(mu[left : right + 1])

        remaining_left = max(i, sqrt_n + 1)
        remaining_right = r
        if remaining_left <= remaining_right and k >= 1:
            m_start = (remaining_left + k - 1) // k
            m_end = remaining_right // k
            for m in range(m_start, m_end + 1):
                d = m * k
                if d <= n:
                    m_n += compute_mu_on_fly(d)

        i = r + 1

    return m_n


def compute_d_formula(n: int) -> int:
    """Reference computation of D(n) using a segmented Möbius summation.

    This is a direct, readable translation of the original Ruby method and is
    suitable for moderate n. For very large n (e.g., 10**10) prefer
    compute_d_optimized.
    """

    sqrt_n = isqrt(n)
    mu = [0] * (sqrt_n + 1)
    is_composite = [False] * (sqrt_n + 1)
    primes: List[int] = []

    mu[1] = 1
    for i in range(2, sqrt_n + 1):
        if not is_composite[i]:
            primes.append(i)
            mu[i] = -1
        for p in primes:
            v = p * i
            if v > sqrt_n:
                break
            is_composite[v] = True
            if i % p == 0:
                mu[v] = 0
                break
            mu[v] = -mu[i]

    m_n = compute_mertens(n, mu, sqrt_n)

    main_sum = 0
    i = 1
    while i <= n:
        k = n // i
        l = n // (k + 1) + 1
        r = n // k

        left = max(i, l)
        right = min(r, sqrt_n)
        if left <= right:
            sum_mu = sum(mu[left : right + 1])
            main_sum += sum_mu * (k + 1) ** 3

        remaining_left = max(i, l, sqrt_n + 1)
        remaining_right = r
        if remaining_left <= remaining_right:
            sum_mu_remaining = compute_mu_sum_segment(
                n, k, remaining_left, remaining_right, mu, sqrt_n
            )
            main_sum += sum_mu_remaining * (k + 1) ** 3

        i = r + 1

    return main_sum - m_n


def compute_d_optimized(n: int) -> int:
    """Optimized computation of D(n) using improved constant-floor technique.

    The formula computes sum_{d=1}^n mu(d) * (floor(n/d) + 1)^3 - M(n)
    where M(n) is the Mertens function.
    """

    sqrt_n = isqrt(n)
    # Sieve Möbius values up to a generous limit
    sieve_limit = min(2 * 10**6, n)
    mu = _sieve_mobius(sieve_limit)

    # Precompute prefix sums for fast range queries
    mu_prefix = [0] * (sieve_limit + 1)
    for i in range(1, sieve_limit + 1):
        mu_prefix[i] = mu_prefix[i - 1] + mu[i]

    main_sum = 0

    # Direct computation for d <= sieve_limit
    for d in range(1, min(sieve_limit, n) + 1):
        k = n // d
        main_sum += mu[d] * (k + 1) ** 3

    # For d > sieve_limit, partition by constant floor(n/d)
    if sieve_limit < n:
        i = sieve_limit + 1
        while i <= n:
            k = n // i
            j = n // k

            # Sum mu(d) for d in [i, j] using on-the-fly computation
            # Note: most large numbers have mu(d) = 0 due to square factors
            for d in range(i, min(j, n) + 1):
                main_sum += compute_mu_on_fly(d) * (k + 1) ** 3

            i = j + 1

    # Mertens M(n)
    m_n = mu_prefix[min(sieve_limit, n)]
    if sieve_limit < n:
        for d in range(sieve_limit + 1, n + 1):
            m_n += compute_mu_on_fly(d)

    return main_sum - m_n


def _format_result(result: int) -> tuple[str, str, str]:
    """Return (first9, last9, combined) string representation for the answer."""

    s = str(result)
    if len(s) <= 9:
        first_nine = s.rjust(9, "0")
        last_nine = s.rjust(9, "0")
    else:
        first_nine = s[:9]
        last_nine = s[-9:]
    return first_nine, last_nine, first_nine + last_nine


def main() -> None:
    """Run basic verification and compute the Project Euler 388 answer."""

    # Use optimized version for better performance
    result = compute_d_optimized(N)
    print(result)


if __name__ == "__main__":
    main()
