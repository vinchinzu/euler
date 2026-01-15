"""Project Euler Problem 339 solver.

This module computes E(n) for the "sheep flocks" process described in the
problem statement. It is a direct, but Pythonic and type-annotated,
translation of the provided Ruby implementation.

Key choices:
- Uses a dynamic programming table dp[i][j] storing the optimal expected
  final number of black sheep given i white and j black.
- Uses a suffix-max table suffix_max[i][j] so that the effect of optionally
  removing white sheep can be applied in O(1).
- Provides a CLI similar to the original Ruby script.

This module is self-contained and uses only the Python standard library.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List
import argparse
import sys

DEFAULT_N: int = 10_000
VERIFY_N: int = 5
EXPECTED_E5: float = 6.871346


@dataclass
class SheepFlocks:
    """Compute E(n) for the sheep flocks process.

    The state (i, j) represents i white sheep and j black sheep currently
    present. The value stored is the optimal expected final number of black
    sheep, assuming optimal removals of white sheep at each step.
    """

    n: int

    def __post_init__(self) -> None:
        if self.n < 0:
            raise ValueError("n must be non-negative")

        self._max_size: int = 2 * self.n
        size: int = self._max_size + 1

        # dp[i][j]: expected optimal final number of black sheep from (i, j)
        self._dp: List[List[float]] = [
            [0.0 for _ in range(size)] for _ in range(size)
        ]

        # suffix_max[i][j]: best expectation if we may remove some white sheep
        # starting from state (i, j) by reducing i but not j.
        self._suffix_max: List[List[float]] = [
            [0.0 for _ in range(size)] for _ in range(size)
        ]

    def compute(self) -> float:
        """Return E(n) for the configured initial flock size.

        For n < 0 a ValueError is raised in __post_init__.
        """

        if self.n == 0:
            return 0.0

        self._setup_base_cases()
        self._fill_dp_table_with_incremental_suffix()

        return self._dp[self.n][self.n]

    # Internal helpers -------------------------------------------------

    def _setup_base_cases(self) -> None:
        size = self._max_size

        # If there are 0 white sheep and j black sheep, the game ends with j.
        for j in range(size + 1):
            self._dp[0][j] = float(j)

        # If there are i white sheep and 0 black sheep, final black count is 0.
        for i in range(1, size + 1):
            self._dp[i][0] = 0.0

    def _fill_dp_table_with_incremental_suffix(self) -> None:
        """Fill DP table while incrementally maintaining prefix maximums.

        This computes prefix_max[i][j] = max(dp[k][j] for k <= i) as we go,
        which represents the best outcome if we remove white sheep to get any
        count from 0 to i.
        """
        size = self._max_size

        # Initialize prefix_max with base cases
        prefix_max = [[0.0 for _ in range(size + 1)] for _ in range(size + 1)]
        for j in range(size + 1):
            prefix_max[0][j] = self._dp[0][j]

        # Iterate forward so that prefix_max[k][j] for k < i are available
        for i in range(1, size + 1):
            for j in range(1, size + 1):
                total = i + j
                if total <= 0:
                    continue

                p_white = i / total
                p_black = j / total

                # For transitions, we need already-computed DP values
                exp_white = 0.0
                if j > 0 and i + 1 <= size:
                    # Use prefix_max for (i+1, j-1) if available, else compute
                    if i + 1 <= size and j - 1 >= 0:
                        exp_white = prefix_max[min(i + 1, size)][j - 1]

                exp_black = 0.0
                if i > 0 and j + 1 <= size:
                    # i-1 < i, so it's already in prefix_max
                    exp_black = prefix_max[max(i - 1, 0)][min(j + 1, size)]

                expected_before_removal = p_white * exp_white + p_black * exp_black

                # Best if we remove some white sheep: max of prefix_max[i-1][j] and current
                best_immediate_removal = prefix_max[i - 1][j] if i > 0 else 0.0

                self._dp[i][j] = max(expected_before_removal, best_immediate_removal)

                # Update prefix_max for this row
                prefix_max[i][j] = max(self._dp[i][j], prefix_max[i - 1][j])

    def _optimal_after_removal(self, white: int, black: int) -> float:
        """Compute the best outcome if we can remove white sheep from (white, black).

        Returns max(dp[k][black] for k < white), since removing white sheep
        reduces the white count. Note: we exclude k=white itself to avoid circular
        dependency when computing dp[white][black].
        """
        if (
            white < 0
            or black < 0
            or white > self._max_size
            or black > self._max_size
        ):
            return 0.0

        # Compute maximum by checking all possible white sheep counts < current
        # (excluding current to avoid circular dependency)
        max_val = 0.0
        for k in range(0, white):
            if self._dp[k][black] > max_val:
                max_val = self._dp[k][black]
        return max_val


def verify_solution() -> None:
    """Run basic checks for small n against known/expected values."""

    print("Verifying solution for small values...")

    test_cases: list[tuple[int, float]] = [
        (0, 0.0),
        (1, 1.0),
        (2, 2.0),
        (3, 3.0),
        (4, 4.0),
        (VERIFY_N, EXPECTED_E5),
    ]

    for n, expected in test_cases:
        solver = SheepFlocks(n)
        result = solver.compute()
        rounded = round(result, 6)
        print(f"E({n}) = {rounded:.6f}")
        if abs(result - expected) > 1e-6:
            print(
                f"  WARNING: Expected {expected:.6f}, got {rounded:.6f}",
            )
        else:
            print("   Matches expected value")


def _build_arg_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Compute E(n) for Project Euler Problem 339 using a dynamic "
            "programming approach."
        )
    )
    parser.add_argument(
        "-n",
        "--n",
        type=int,
        default=DEFAULT_N,
        help="Value of n (default: %(default)s)",
    )
    parser.add_argument(
        "-v",
        "--verify",
        action="store_true",
        help="Run verification tests for small n values",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    """Command-line entry point.

    Returns process exit code.
    """

    if argv is None:
        argv = sys.argv[1:]

    parser = _build_arg_parser()
    args = parser.parse_args(argv)

    if args.n < 0:
        print("Error: n must be non-negative", file=sys.stderr)
        return 1

    if args.verify:
        verify_solution()
        return 0

    n: int = args.n
    print(f"Computing E({n}) for Project Euler Problem 339...")

    try:
        solver = SheepFlocks(n)
        result = solver.compute()
        print(f"E({n}) = {result:.6f}")

        # Print only final answer for test harness
        print()
        print(int(result))
    except Exception as exc:  # pragma: no cover - defensive
        print(f"Error during computation: {exc}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
