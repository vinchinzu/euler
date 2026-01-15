"""Project Euler Problem 374 - Python translation.

This module computes, for each n, the maximum product f(n) of parts in a partition
of n into distinct positive integers and m(n), the number of parts in a partition
achieving that product. It then supports summing f(n) * m(n) modulo MOD.

The implementation is a direct, cleaned-up translation of the given Ruby draft.
It is NOT a fully optimized solution for n up to 10**14. The large-n behavior
remains a TODO and would require additional mathematical insights.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List

MOD: int = 982_451_653
TARGET_N: int = 10**14
SMALL_LIMIT: int = 1_000


def mod_pow(base: int, exponent: int, mod: int) -> int:
    """Return (base ** exponent) % mod using fast exponentiation."""
    result = 1
    base %= mod
    e = exponent
    while e > 0:
        if e & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        e >>= 1
    return result


def mod_inverse(a: int, mod: int) -> int:
    """Return modular inverse of a modulo mod (mod must be prime)."""
    return mod_pow(a, mod - 2, mod)


@dataclass
class MaxProductPartition:
    """Compute f(n), m(n) and related sums for partitions into distinct parts.

    Public methods:
    - f(n): maximum product of parts of a distinct-parts partition of n.
    - m(n): number of parts in a partition achieving f(n).
    - sum_f_times_m(start_n, end_n): sum f(n) * m(n) over [start_n, end_n].

    Note:
        The approach is suitable for small and moderate n. For n up to 10**14 as
        in the original problem statement, further mathematical optimization is
        required. See TODO in sum_f_times_m_large.
    """

    small_f: List[int]
    small_m: List[int]

    def __init__(self) -> None:
        self.small_f = [0] * (SMALL_LIMIT + 1)
        self.small_m = [0] * (SMALL_LIMIT + 1)
        self._precompute_small()

    def _precompute_small(self) -> None:
        """Precompute f(n), m(n) exactly for n up to SMALL_LIMIT via DP.

        dp[i][j]: max product modulo MOD using distinct parts from 1..i summing j.
        track[i][j]: maximum number of parts achieving dp[i][j].
        """

        dp: List[List[int]] = [
            [0] * (SMALL_LIMIT + 1) for _ in range(SMALL_LIMIT + 1)
        ]
        track: List[List[int]] = [
            [0] * (SMALL_LIMIT + 1) for _ in range(SMALL_LIMIT + 1)
        ]

        for i in range(SMALL_LIMIT + 1):
            dp[i][0] = 1
            track[i][0] = 0

        for i in range(1, SMALL_LIMIT + 1):
            num = i
            for j in range(SMALL_LIMIT + 1):
                # Exclude num
                best_prod = dp[i - 1][j]
                best_count = track[i - 1][j]

                # Include num if possible
                if j >= num and dp[i - 1][j - num] > 0:
                    new_product = (dp[i - 1][j - num] * num) % MOD
                    if new_product > best_prod:
                        best_prod = new_product
                        best_count = track[i - 1][j - num] + 1
                    elif new_product == best_prod:
                        # Prefer partition with more parts when product ties
                        cand_count = track[i - 1][j - num] + 1
                        if cand_count > best_count:
                            best_count = cand_count

                dp[i][j] = best_prod
                track[i][j] = best_count

        for n in range(1, SMALL_LIMIT + 1):
            self.small_f[n] = dp[SMALL_LIMIT][n]
            self.small_m[n] = track[SMALL_LIMIT][n]

    def optimal_k(self, n: int) -> int:
        """Heuristically choose k, the number of distinct parts for large n.

        This mirrors the original Ruby heuristic based on n / e. It's not proven
        optimal for all n and is suitable only for exploratory computations.
        """

        if n <= 0:
            raise ValueError("n must be positive")

        k_start = max(1, int(n / 2.71828))
        k_end = min(n, k_start + 2)

        best_k = k_start
        best_product = 0

        for k in range(k_start, k_end + 1):
            start_num = int((n - k * (k - 1) / 2.0) // k)
            start_num = max(2, start_num)

            numbers = [start_num + i for i in range(k)]
            current_sum = sum(numbers)

            if current_sum != n:
                diff = n - current_sum
                adjust = round(diff / float(k))
                numbers = [x + adjust for x in numbers]
                numbers = sorted(set(numbers))
                if len(numbers) != k or any(x <= 0 for x in numbers):
                    continue

            product = 1
            for x in numbers:
                product = (product * x) % MOD

            if product > best_product:
                best_product = product
                best_k = k

        return best_k

    def f(self, n: int) -> int:
        """Return f(n): max product of parts of a distinct-part partition of n."""

        if n <= 0:
            raise ValueError("n must be positive")
        if n <= 3:
            return n
        if n <= SMALL_LIMIT:
            return self.small_f[n]

        k = self.optimal_k(n)

        start_num = round((n - k * (k - 1) / 2.0) / k)
        start_num = max(2, start_num)

        numbers = [start_num + i for i in range(k)]
        current_sum = sum(numbers)

        if current_sum != n:
            diff = n - current_sum
            adjust = round(diff / float(k))
            numbers = [x + adjust for x in numbers]
            numbers.sort()

            # Ensure distinctness by repairing collisions.
            for i in range(len(numbers) - 1):
                if numbers[i] >= numbers[i + 1]:
                    numbers[i + 1] = numbers[i] + 1

        product = 1
        for num in numbers:
            product = (product * num) % MOD

        return product

    def m(self, n: int) -> int:
        """Return m(n): number of parts in a partition achieving f(n).

        For large n, this uses the same heuristic k as optimal_k(n).
        """

        if n <= 0:
            raise ValueError("n must be positive")
        if n <= 3:
            return 1
        if n <= SMALL_LIMIT:
            return self.small_m[n]

        return self.optimal_k(n)

    def verify(self) -> None:
        """Run simple verification against known small examples.

        This is primarily for sanity-checking the translation and small-n DP.
        """

        print("Verifying against known examples...")

        f5, m5 = self.f(5), self.m(5)
        print(f"n=5: f(5)={f5}, m(5)={m5} (expected: 6, 2)")

        f10, m10 = self.f(10), self.m(10)
        print(f"n=10: f(10)={f10}, m(10)={m10} (expected: 30, 3)")

        expected_sum_100 = 1_683_550_844_462
        computed_sum_100 = 0
        for n in range(1, 101):
            computed_sum_100 = (computed_sum_100 + self.f(n) * self.m(n)) % MOD

        print(
            "Sum for n=1 to 100 (mod MOD): "
            f"{computed_sum_100} (expected value modulo MOD of {expected_sum_100})"
        )

        print("Verification complete.")

    def sum_f_times_m(self, start_n: int, end_n: int) -> int:
        """Return sum_{n=start_n}^{end_n} f(n) * m(n) modulo MOD."""

        if start_n <= 0 or end_n < start_n:
            raise ValueError("Invalid range for summation")

        total = 0
        for n in range(start_n, end_n + 1):
            total = (total + self.f(n) * self.m(n)) % MOD
        return total

    def sum_f_times_m_large(self, start_n: int, end_n: int) -> int:
        """Placeholder for an optimized large-n summation.

        TODO:
            Implement a mathematically optimized approach for n up to 10**14.
            The current implementation simply delegates to sum_f_times_m and is
            not feasible for extremely large ranges.
        """

        return self.sum_f_times_m(start_n, end_n)


def main() -> None:
    """Execute basic demonstration and verification for this module.

    Note:
        This is not the full 10**14 solution; it showcases the current logic.
    """

    print("Solving Project Euler Problem 374 (demonstration)...")
    print(f"Target: sum f(n)*m(n) for 1  n  {TARGET_N}, modulo {MOD}")

    partition = MaxProductPartition()

    partition.verify()

    small_compute_limit = 1_000
    reasonable_limit = 1_000  # Adjust upward cautiously for experimentation.

    print(f"\nComputing sum for n=1 to {small_compute_limit}...")
    sum_small = partition.sum_f_times_m(1, small_compute_limit)
    print(f"Sum for n=1 to {small_compute_limit}: {sum_small}")

    if reasonable_limit > small_compute_limit:
        print(
            f"\nComputing sum for n={small_compute_limit + 1} "
            f"to {reasonable_limit}..."
        )
        sum_medium = partition.sum_f_times_m(
            small_compute_limit + 1,
            reasonable_limit,
        )
        total_sum = (sum_small + sum_medium) % MOD
        print(f"Total sum up to {reasonable_limit}: {total_sum}")
    else:
        total_sum = sum_small

    print(
        "\nNote: For n up to 10**14, additional analytical optimization is required."
    )
    print("Current implementation is suitable only for relatively small n.")
    print(f"Final computed sum (up to {reasonable_limit}): {total_sum}")

    # Print only final answer for test harness
    print()
    print(total_sum)


if __name__ == "__main__":
    main()
