"""Project Euler Problem 378 solver (Python 3.12).

This module computes Tr(n), the number of triples (i, j, k) such that
1 <= i < j < k <= n and dT(i) > dT(j) > dT(k), where T(n) is the nth
triangular number and dT(n) is the number of divisors of T(n).

It includes:
- A smallest-prime-factor sieve.
- A function to count divisors of triangular numbers using the SPF table.
- A Fenwick tree (Binary Indexed Tree) implementation for prefix sums.
- A main entry point that prints the last 18 digits of Tr(60_000_000).

The implementation is designed to be:
- Idiomatic Python 3.12.
- Fully typed.
- Self-contained (standard library only).

Note: Executing compute_Tr(60_000_000) is computationally intensive in
pure Python and is primarily suitable for reference, experimentation, or
optimized environments (e.g., PyPy, C-extensions, or further tuning).
"""

from __future__ import annotations

from dataclasses import dataclass
from math import isqrt
from time import perf_counter
from typing import Dict, List

MOD: int = 10**18
N: int = 60_000_000
BATCH_SIZE: int = 1_000_000


def sieve_spf(limit: int) -> List[int]:
    """Return list `spf` where spf[x] is the smallest prime factor of x.

    For x = 0 or 1, spf[x] == x by convention.
    Complexity: O(limit log log limit).
    """

    if limit < 0:
        msg = "limit must be non-negative"
        raise ValueError(msg)

    spf: List[int] = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] != i:  # not prime
            continue
        step = i
        start = i * i
        for j in range(start, limit + 1, step):
            if spf[j] > i:
                spf[j] = i
    return spf


def factor_with_spf(num: int, spf: List[int]) -> Dict[int, int]:
    """Factor `num` into prime powers using a smallest-prime-factor table.

    Returns a dict {prime: exponent}. For num <= 1, returns an empty dict.
    """

    if num <= 1:
        return {}

    factors: Dict[int, int] = {}
    while num > 1:
        p = spf[num]
        exp = 0
        while num % p == 0:
            exp += 1
            num //= p
        factors[p] = factors.get(p, 0) + exp
    return factors


def dT(n: int, spf: List[int]) -> int:
    """Return dT(n): number of divisors of the nth triangle number T(n).

    Uses the relation T(n) = n(n + 1) / 2 and a precomputed SPF table.
    """

    if n <= 1:
        return 1

    factors_n = factor_with_spf(n, spf)
    factors_np1 = factor_with_spf(n + 1, spf)

    # Combine exponents for n and n+1.
    factors: Dict[int, int] = dict(factors_n)
    for p, e in factors_np1.items():
        factors[p] = factors.get(p, 0) + e

    # Divide by 2 (remove one factor of 2 from the product).
    if 2 in factors:
        factors[2] -= 1
        if factors[2] == 0:
            del factors[2]

    result = 1
    for exp in factors.values():
        result *= exp + 1
    return result


def precompute_dT(n: int) -> List[int]:
    """Precompute dT(i) for 1 <= i <= n.

    Returns a list `dt_values` such that dt_values[i] == dT(i).
    Index 0 is set to 0 and unused for convenience of 1-based indexing.
    """

    if n < 1:
        msg = "n must be >= 1"
        raise ValueError(msg)

    print(f"Precomputing SPF sieve for {n}...")
    t0 = perf_counter()
    spf = sieve_spf(n + 1)
    t1 = perf_counter()
    print(f"SPF sieve completed in {t1 - t0:.3f} seconds")

    print("Computing dT values...")
    dt_values: List[int] = [0] * (n + 1)

    for start in range(1, n + 1, BATCH_SIZE):
        batch_end = min(start + BATCH_SIZE - 1, n)
        progress = (start / n) * 100.0
        print(
            f"Processing batch {start}-{batch_end} "
            f"({progress:6.2f}% complete)",
            end="\r",
            flush=True,
        )
        for i in range(start, batch_end + 1):
            dt_values[i] = dT(i, spf)

    t2 = perf_counter()
    print("\nPrecomputation completed in " f"{t2 - t1:.3f} seconds")

    return dt_values


@dataclass
class FenwickTree:
    """Fenwick tree (Binary Indexed Tree) for prefix sums modulo `mod`."""

    size: int
    mod: int = MOD

    def __post_init__(self) -> None:
        if self.size < 0:
            msg = "size must be non-negative"
            raise ValueError(msg)
        # Use size + 2 as safety margin; indexes are 1-based internally.
        self._tree: List[int] = [0] * (self.size + 2)

    def update(self, pos: int, val: int) -> None:
        """Add `val` at index `pos` (0-based external index)."""

        if pos < 0:
            return
        i = pos + 1
        size = self.size + 1
        mod = self.mod
        tree = self._tree
        while i <= size:
            tree[i] = (tree[i] + val) % mod
            i += i & -i

    def query(self, pos: int) -> int:
        """Return prefix sum for indices [0, pos] (0-based).

        If pos < 0, returns 0.
        """

        if pos < 0:
            return 0
        i = pos + 1
        res = 0
        mod = self.mod
        tree = self._tree
        while i > 0:
            res = (res + tree[i]) % mod
            i -= i & -i
        return res

    def query_range(self, left: int, right: int) -> int:
        """Return sum for indices [left, right] (0-based, inclusive)."""

        if right < left:
            return 0
        if right < 0:
            return 0
        left = max(left, 0)
        return (self.query(right) - self.query(left - 1)) % self.mod


def compute_Tr(n: int) -> int:
    """Compute Tr(n) modulo MOD.

    Tr(n) counts triples (i, j, k) such that 1 <= i < j < k <= n and
    dT(i) > dT(j) > dT(k).
    """

    if n < 1:
        msg = "n must be positive integer"
        raise ValueError(msg)
    if n < 3:
        return 0

    dt_values = precompute_dT(n)

    max_dt = max(dt_values[1:])
    print(f"Maximum dT value: {max_dt}")

    ft = FenwickTree(max_dt + 1, MOD)

    result = 0
    # We process j from n-1 down to 1. For each position, we count how
    # many later positions have smaller dT.
    for i in range(n - 1, 0, -1):
        value = dt_values[i]
        count_smaller = ft.query_range(0, value - 1)
        result = (result + count_smaller) % MOD
        ft.update(value, 1)

    return result


def main() -> str:
    """Run the Problem 378 computation for N and print the result.

    Returns the formatted last 18 digits as a string.
    """

    print(f"Project Euler Problem 378 - Computing Tr({N})")
    print("=" * 50)

    t0 = perf_counter()
    result = compute_Tr(N)
    t1 = perf_counter()
    print(f"\nComputation completed in {t1 - t0:.3f} seconds")

    last_18_digits = f"{result % MOD:018d}"
    print(f"\nLast 18 digits of Tr({N}): {last_18_digits}")

    return last_18_digits


def _run_tests() -> None:
    """Basic self-checks mirroring the original Ruby tests.

    These are light sanity tests; they do not exhaustively validate the
    algorithm for large inputs.
    """

    small_n = 20
    spf = sieve_spf(small_n + 1)

    assert dT(1, spf) == 1
    assert dT(7, spf) == 6
    assert dT(2, spf) == 2

    assert compute_Tr(1) == 0
    assert compute_Tr(3) == 0

    # Known reference from the problem statement.
    assert compute_Tr(20) == 14


if __name__ == "__main__":  # pragma: no cover
    import sys

    if "--test" in sys.argv:
        print("Running self-tests...")
        _run_tests()
        print("All tests passed.")
    else:
        main()
