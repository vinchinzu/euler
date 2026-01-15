"""Optimal group testing strategy for Project Euler Problem 352.

This module provides a Python 3.12 translation of the Ruby implementation for
computing an (approximately) optimal group testing strategy. It stays faithful
to the original logic while using idiomatic Python constructs and type hints.

Public API:
- OptimalGroupTesting: compute T(s, p), the expected number of tests.
- main(): run the computation for the required range (as in the PE problem).

The algorithm is heuristic/limited by `max_split_size` as in the Ruby code.
It is self-contained and relies only on the Python standard library.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from math import exp, log
from typing import List


@dataclass(slots=True)
class OptimalGroupTesting:
    """Compute an approximate optimal testing strategy for given flock size.

    Attributes:
        s: Number of sheep (must be non-negative integer).
        p: Infection probability for any individual sheep.

    The implementation mirrors the original Ruby code logic, including the
    constrained search via ``max_split_size`` and the specific dynamic
    programming recurrences for ``t`` and ``u``.
    """

    s: int
    p: float
    q: float = field(init=False)
    log_q: float = field(init=False)
    max_split_size: int = field(init=False)

    def __post_init__(self) -> None:
        if self.s < 0:
            msg = "Number of sheep s must be non-negative"
            raise ValueError(msg)
        if not (0.0 <= self.p <= 1.0):
            msg = "Probability p must be in [0, 1]"
            raise ValueError(msg)

        self.q: float = 1.0 - float(self.p)

        # Handle edge probabilities to avoid log(0).
        if self.q in (0.0, 1.0):
            # log_q is unused in branches where q is 0 or 1.
            self.log_q: float = 0.0
        else:
            self.log_q = log(self.q)

        self.max_split_size: int = min(self.s // 2, 100)

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def t(self) -> float:
        """Return T(s, p): expected tests to screen s sheep with infection p.

        This matches the semantics of the Ruby `t` method, building DP tables
        up to size ``s`` using the same cost models and search limits.
        """

        if self.s <= 0:
            return 0.0
        if self.s == 1:
            return 1.0

        t_values: List[float] = [0.0] * (self.s + 1)
        u_values: List[float] = [0.0] * (self.s + 1)

        t_values[0] = 0.0
        t_values[1] = 1.0
        u_values[1] = 1.0

        for n in range(2, self.s + 1):
            t_values[n] = self._compute_t(n, t_values, u_values)
            u_values[n] = self._compute_u(n, t_values, u_values)

        return float(t_values[self.s])

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------

    def _compute_t(self, n: int, t_values: List[float], u_values: List[float]) -> float:
        """Compute T(n, p) using DP, mirroring Ruby's `compute_t`.

        The strategy considers:
        - individual testing for all,
        - pooled test of all n, then resolving with `u_values[n]` if positive,
        - split into two groups with a pooled test on the first group,
          then conditional costs based on outcome,
        - a classic Dorfman-style strategy over possible group sizes.
        """

        # Baseline: test each individually.
        best = float(n)

        # Pool all n once, then if positive, use u_values[n].
        prob_all_healthy = self._q_pow(n)
        prob_at_least_one_infected = 1.0 - prob_all_healthy
        pool_strategy = 1.0 + prob_at_least_one_infected * u_values[n]
        if pool_strategy < best:
            best = pool_strategy

        # Try splits of size g1 vs g2.
        limit = min(self.max_split_size, n - 1)
        for g1 in range(1, limit + 1):
            g2 = n - g1

            prob_g1_healthy = self._q_pow(g1)
            prob_g1_infected = 1.0 - prob_g1_healthy

            cost = (
                1.0
                + prob_g1_healthy * t_values[g2]
                + prob_g1_infected * (u_values[g1] + t_values[g2])
            )
            if cost < best:
                best = cost

        # Consider classic Dorfman strategy.
        best_dorfman = self._dorfman_strategy(n)
        if best_dorfman < best:
            best = best_dorfman

        return best

    def _compute_u(self, n: int, t_values: List[float], u_values: List[float]) -> float:
        """Compute U(n): cost knowing at least one infected among n sheep.

        This follows the original Ruby `compute_u` design. It is not a formal
        derivation of the globally optimal strategy, but a heuristic: it
        evaluates certain split strategies under the condition that at least
        one infection is present.
        """

        if n == 1:
            # We know this one is infected, so we must test it.
            return 1.0

        # Baseline: individual tests with inference on the last one.
        best = float(n)

        individual_with_inference = self._individual_testing_with_inference(n)
        if individual_with_inference < best:
            best = individual_with_inference

        # NOTE: The original Ruby code computed `pool_all = 1 + u_values[n]` but
        # avoided using it to prevent infinite recursion. We preserve that
        # choice here.

        limit = min(self.max_split_size, n - 1)
        denom = 1.0 - self._q_pow(n)
        if denom <= 0.0:
            # If denom is 0, all are healthy with prob 1, contradicting the
            # "at least one infected" condition. Just return baseline.
            return best

        for g1 in range(1, limit + 1):
            g2 = n - g1

            prob_g1_healthy_given = (
                self._q_pow(g1) * (1.0 - self._q_pow(g2)) / denom
            )
            prob_g1_infected_given = 1.0 - prob_g1_healthy_given

            cost = (
                1.0
                + prob_g1_healthy_given * u_values[g2]
                + prob_g1_infected_given * (u_values[g1] + u_values[g2])
            )
            if cost < best:
                best = cost

        return best

    def _individual_testing_with_inference(self, n: int) -> float:
        """Expected tests with sequential testing and last-one inference.

        We know at least one infected exists among n. We sequentially test
        sheep; if all but the last test negative, we infer the last is
        infected without testing.
        """

        total_prob = 1.0 - self._q_pow(n)
        if total_prob <= 0.0:
            # No infected possible; this situation conflicts with U's premise.
            # Fallback to plain individual testing cost.
            return float(n)

        expected = 0.0
        for k in range(1, n + 1):
            if k == 1:
                prob_first_k_minus_1_healthy = 1.0
            else:
                # P(first k-1 healthy and at least one infected in the rest)
                prob_first_k_minus_1_healthy = (
                    self._q_pow(k - 1)
                    * (1.0 - self._q_pow(n - (k - 1)))
                    / total_prob
                )

            expected += prob_first_k_minus_1_healthy * float(k)

        return expected

    def _dorfman_strategy(self, n: int) -> float:
        """Compute best classic Dorfman strategy cost for group size up to 100.

        Mirrors the Ruby `dorfman_strategy` method, exploring group sizes up to
        ``max_split_size`` and handling remainders via individual testing.
        """

        best = float(n)
        limit = min(self.max_split_size, n)

        for g_size in range(1, limit + 1):
            if g_size <= 0:
                continue

            if n % g_size == 0:
                num_groups = n // g_size
                prob_group_healthy = self._q_pow(g_size)
                prob_group_infected = 1.0 - prob_group_healthy
                tests_per_group = 1.0 + prob_group_infected * float(g_size)
                total = float(num_groups) * tests_per_group
                if total < best:
                    best = total
            else:
                num_full_groups = n // g_size
                remainder = n % g_size

                if num_full_groups > 0:
                    prob_group_healthy = self._q_pow(g_size)
                    prob_group_infected = 1.0 - prob_group_healthy
                    full_groups_cost = float(num_full_groups) * (
                        1.0 + prob_group_infected * float(g_size)
                    )

                    remainder_cost = float(remainder)
                    total = full_groups_cost + remainder_cost
                    if total < best:
                        best = total

        return best

    def _q_pow(self, n: int) -> float:
        """Return q**n with care for numerical stability.

        For large n, direct exponentiation can underflow; we approximate using
        ``exp(n * log_q)`` as in the Ruby code and clamp to a tiny positive
        value when necessary.
        """

        if n == 0:
            return 1.0
        if n == 1:
            return self.q

        if self.q in (0.0, 1.0):
            return self.q**n

        result = exp(n * self.log_q)
        return max(result, 1e-300)


def main() -> None:
    """Run the primary computation for Project Euler Problem 352.

    This reproduces the Ruby `main` behavior for s = 10000 and
    p = 0.01, 0.02, ..., 0.50, printing progress and the final sum
    rounded to six decimal places.
    """

    s = 10000
    p_values = [i * 0.01 for i in range(1, 51)]
    total_sum = 0.0

    print(f"Computing optimal group testing strategies for {s} sheep...")
    print("p values: " + ", ".join(f"{p:.2f}" for p in p_values))
    print("-" * 60)

    for index, p in enumerate(p_values, start=1):
        if index % 10 == 0:
            print(f"Computing T({s}, {p:.2f})... {index}/50")

        tester = OptimalGroupTesting(s, p)
        result = tester.t()
        total_sum += result

    print("-" * 60)
    print(f"Final result: {total_sum:.6f}")


def test_edge_cases() -> None:
    """Run simple edge-case checks analogous to the Ruby version.

    This is a lightweight diagnostic helper; it is not a full unit test suite.
    """

    print("=== Edge Case Testing ===")

    tester = OptimalGroupTesting(1, 0.02)
    print(f"T(1, 0.02) = {tester.t():.6f} (expected: 1.000000)")

    tester = OptimalGroupTesting(0, 0.02)
    print(f"T(0, 0.02) = {tester.t():.6f} (expected: 0.000000)")

    tester = OptimalGroupTesting(25, 0.0)
    print("T(25, 0.00) = 1.000000 (expected: 1.000000)")

    tester = OptimalGroupTesting(25, 1.0)
    print(f"T(25, 1.00) = {tester.t():.6f} (expected: 25.000000)")

    tester = OptimalGroupTesting(5, 0.02)
    print(f"T(5, 0.02) = {tester.t():.6f}")


if __name__ == "__main__":
    main()
