"""Project Euler Problem 337 - Python translation.

This module computes S(N) as defined in the problem statement:

- a_1 = 6
- For all 1 <= i < n:
    phi(a_i) < phi(a_{i + 1}) < a_i < a_{i + 1}
- S(N) is the number of such sequences with a_n <= N.

The algorithm:
- Precompute Euler's totient function phi(k) for 1 <= k <= N.
- Sort values by (phi(k), k).
- Use dynamic programming combined with a Fenwick tree (Binary Indexed Tree)
  to count valid sequences efficiently under modulo 1e8.

The implementation targets Python 3.12 and uses only the standard library.
"""

from __future__ import annotations

from dataclasses import dataclass
from time import perf_counter
from typing import List, Tuple

START: int = 6
MOD: int = 100_000_000
TARGET_N: int = 20_000_000


@dataclass
class FenwickTree:
    """Fenwick Tree (Binary Indexed Tree) supporting prefix sums.

    Indexing is 0-based externally and mapped to 1-based internally.
    No modulo operations for speed; caller handles modulo.
    """

    size: int

    def __post_init__(self) -> None:
        # +2 for 1-based indexing and safety margin
        self._tree: List[int] = [0] * (self.size + 2)

    def update(self, idx: int, val: int) -> None:
        """Add val to position idx.

        idx is 0-based; internally converted to 1-based.
        """

        i = idx + 1
        size_bound = self.size + 1
        while i <= size_bound:
            self._tree[i] += val
            i += i & -i

    def prefix(self, idx: int) -> int:
        """Return prefix sum for [0, idx].

        idx is 0-based; internally converted to 1-based.
        """

        if idx < 0:
            return 0

        res = 0
        i = idx + 1
        while i > 0:
            res += self._tree[i]
            i -= i & -i
        return res

    def query(self, left: int, right: int) -> int:
        """Return sum over [left, right].

        If left > right, returns 0.
        """

        if left > right:
            return 0
        return self.prefix(right) - self.prefix(left - 1)


def sieve_phi(limit: int) -> List[int]:
    """Compute Euler's totient function φ(k) for 0 ≤ k ≤ limit using a
    linear-time sieve.
    """
    phi = [0] * (limit + 1)
    primes: List[int] = []
    is_composite = [False] * (limit + 1)

    phi[0] = 0
    if limit >= 1:
        phi[1] = 1

    for i in range(2, limit + 1):
        if not is_composite[i]:
            primes.append(i)
            phi[i] = i - 1
        for p in primes:
            x = i * p
            if x > limit:
                break
            is_composite[x] = True
            if i % p == 0:
                phi[x] = phi[i] * p
                break
            phi[x] = phi[i] * (p - 1)

    return phi


def compute_s(n: int) -> int:
    """Compute S(n) modulo MOD.

    Optimized version using sorting approach.
    """

    if n < START:
        return 0

    phi_values = sieve_phi(n)

    # Create pairs (phi(i), i) for i in [START, n] and sort by phi then i.
    pairs: List[Tuple[int, int]] = [
        (phi_values[i], i) for i in range(START, n + 1)
    ]
    pairs.sort()

    dp: List[int] = [0] * (n + 1)
    tree = FenwickTree(n)

    total = 0
    i = 0
    length = len(pairs)

    # Process elements grouped by equal phi to avoid within-group dependencies.
    while i < length:
        current_phi = pairs[i][0]
        group_indices: List[int] = []

        # Collect all indices with the same phi value.
        while i < length and pairs[i][0] == current_phi:
            group_indices.append(pairs[i][1])
            i += 1

        # Compute dp for all indices in the current group.
        for j in group_indices:
            left = max(START, current_phi + 1)
            right = j - 1
            sum_prev = tree.query(left, right) % MOD
            base = 1 if j == START else 0
            value = (base + sum_prev) % MOD
            dp[j] = value
            total = (total + value) % MOD

        # Update the Fenwick tree with finished dp values.
        for j in group_indices:
            tree.update(j, dp[j])

    return total


def run_tests() -> None:
    """Run basic self-checks based on known values from the problem statement."""

    test_cases = [
        (5, 0),
        (6, 1),
        (10, 4),
        (100, 482_073_668),
    ]

    for n, expected in test_cases:
        result = compute_s(n) % MOD
        status = "PASS" if result == (expected % MOD) else "FAIL"
        print(f"Test S({n}), got {result}, expected {expected} -> {status}")

    n = 10_000
    expected_mod = 73_808_307
    result = compute_s(n) % MOD
    status = "PASS" if result == expected_mod else "FAIL"
    print(f"Test S({n}) mod 1e8, got {result}, expected {expected_mod} -> {status}")


def main() -> None:
    """Entry point used when executing this module directly.

    Computes S(TARGET_N) modulo MOD and prints the result and timing.
    """

    start = perf_counter()
    result = compute_s(TARGET_N)
    elapsed = perf_counter() - start

    print(f"Final answer: S({TARGET_N}) mod {MOD} = {result}")
    print(f"Computation completed in {elapsed:.2f} seconds")


if __name__ == "__main__":  # pragma: no cover - CLI behavior
    main()