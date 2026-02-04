"""Ant grid solver for a Project Euler style problem.

This module is a translation of a Ruby implementation to idiomatic Python 3.12.
It attempts to preserve the algorithmic intent:
- Count ant movement configurations on an n x n grid.
- Each ant starts on a distinct square.
- All ants move simultaneously to an adjacent square.
- No two ants may end on the same square.
- No two ants may traverse the same grid edge.

The implementation is kept self-contained, using only the Python standard library.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Iterable, List, Tuple

MOD: int = 1_000_000_007


@dataclass(slots=True)
class AntGridSolver:
    """Solve the constrained ant-movement count for an n x n grid.

    This is a direct port of the provided Ruby structure. The original algorithm
    appears to mix a DP over row states with horizontal move enumerations and
    encodes vertical/horizontal movements in base-4 state values.

    WARNING
    -------
    The original Ruby code ("393.rb") is incomplete/incorrect
    w.r.t. the known values for the classic Problem 393 (e.g. f(4) = 88), and
    its state encoding/transition logic is non-trivial. This translation keeps
    the same structure and behavior rather than re-deriving the mathematically
    correct solution. If you rely on mathematically verified values, treat this
    as a starting point and validate/adjust the transitions.
    """

    n: int
    _grid_size: int = 0  # Will be computed in __post_init__

    def __post_init__(self) -> None:
        if not isinstance(self.n, int) or self.n < 1:
            msg = "Grid size must be a positive integer"
            raise ValueError(msg)
        # Use object.__setattr__ to set slot field in frozen-like dataclass
        object.__setattr__(self, '_grid_size', self.n * self.n)

    # ---------------------------------------------------------------------
    # Public API
    # ---------------------------------------------------------------------

    def compute_f(self) -> int:
        """Compute f(n) using the translated DP approach.

        Returns
        -------
        int
            Number of valid movement configurations modulo MOD.
        """

        if self.n == 1:
            # Single ant has no valid moves under the problem constraints.
            return 0

        horizontal_states = self._precompute_horizontal_states()
        state_cnt = self._state_count()

        dp: List[List[int]] = [
            [0 for _ in range(state_cnt)] for _ in range(self.n + 1)
        ]

        initial_state = 0
        dp[0][initial_state] = 1

        for row in range(self.n):
            for prev_state, ways in enumerate(dp[row]):
                if ways == 0:
                    continue

                for h_state, h_ways in horizontal_states.items():
                    if not self._valid_horizontal_transition(
                        prev_state, h_state, row
                    ):
                        continue

                    next_state = self._compute_next_state(prev_state, h_state)

                    dp[row + 1][next_state] = (
                        dp[row + 1][next_state] + ways * h_ways
                    ) % MOD

        # Ruby code's final_states logic was incorrect: it inspected values, not
        # state indices. Fixed to sum all non-zero final state values.
        final_values = [v for v in dp[self.n] if v != 0]
        return sum(final_values) % MOD

    # ------------------------------------------------------------------
    # Internal helpers: state encoding and precomputation
    # ------------------------------------------------------------------

    def _state_count(self) -> int:
        return 4**self.n

    def _precompute_horizontal_states(self) -> Dict[int, int]:
        """Precompute all horizontal move state encodings and their multiplicity.

        Returns
        -------
        dict
            Mapping from encoded state to number of distinct ways.
        """

        states: Dict[int, int] = {}
        self._generate_horizontal_combinations(0, [], states)
        return states

    def _generate_horizontal_combinations(
        self,
        col: int,
        current_matching: List[str],
        states: Dict[int, int],
    ) -> None:
        if col == self.n:
            return

        for move_type in self._get_horizontal_moves(col):
            new_matching = [*current_matching, move_type]

            if not self._valid_partial_horizontal_matching(new_matching, col):
                continue

            if col == self.n - 1:
                ways = self._count_horizontal_ways(new_matching)
                state_key = self._matching_to_state(new_matching)
                states[state_key] = states.get(state_key, 0) + ways
            else:
                self._generate_horizontal_combinations(col + 1, new_matching, states)

    def _get_horizontal_moves(self, col: int) -> List[str]:
        moves: List[str] = ["stay"]  # Ants can move vertically (stay in same column)
        if col > 0:
            moves.append("left")
        if col < self.n - 1:
            moves.append("right")
        return moves

    def _valid_partial_horizontal_matching(
        self,
        matching: Iterable[str],
        col: int,  # Unused; kept to mirror original signature
    ) -> bool:
        positions: Dict[int, int] = {}

        for c, move in enumerate(matching):
            if move == "left":
                target_col = c - 1
            elif move == "right":
                target_col = c + 1
            else:
                target_col = c

            prev = positions.get(target_col)
            if prev is not None and prev != c:
                return False
            positions[target_col] = c

        return True

    def _count_horizontal_ways(self, matching: List[str]) -> int:
        ways = 1
        i = 0
        n = self.n
        while i < n:
            if (
                i < n - 1
                and matching[i] == "right"
                and matching[i + 1] == "left"
            ):
                # Two directions to traverse this shared edge.
                ways = (ways * 2) % MOD
                i += 2
            else:
                i += 1
        return ways

    def _matching_to_state(self, matching: List[str]) -> int:
        state = 0
        base = 1
        for move in matching:
            if move == "left":
                state += 1 * base
            elif move == "right":
                state += 2 * base
            base *= 4
        return state

    # ------------------------------------------------------------------
    # Internal helpers: transitions and decoding
    # ------------------------------------------------------------------

    def _valid_horizontal_transition(
        self,
        prev_state: int,
        h_state: int,
        row: int,
    ) -> bool:
        prev_vertical = self._decode_state(prev_state)
        horizontal = self._decode_horizontal_state(h_state)

        if not self._no_vertical_horizontal_conflicts(prev_vertical, horizontal, row):
            return False

        if row == 0 and any(v != "none" for v in prev_vertical):
            # No incoming vertical moves from above first row.
            return False

        return True

    def _decode_state(self, state: int) -> List[str]:
        vertical: List[str] = []
        for col in range(self.n):
            digit = (state // (4**col)) % 4
            if digit == 1:
                vertical.append("up")
            elif digit == 2:
                vertical.append("down")
            else:
                vertical.append("none")
        return vertical

    def _decode_horizontal_state(self, h_state: int) -> List[str]:
        horizontal: List[str] = []
        for col in range(self.n):
            digit = (h_state // (4**col)) % 4
            if digit == 1:
                horizontal.append("left")
            elif digit == 2:
                horizontal.append("right")
            else:
                horizontal.append("none")
        return horizontal

    def _no_vertical_horizontal_conflicts(
        self,
        vertical: List[str],
        horizontal: List[str],
        row: int,
    ) -> bool:
        """Check that no edge is used by more than one ant.

        This mirrors the Ruby logic using string keys to identify edges.
        """

        edge_uses: set[str] = set()

        for col, v_move in enumerate(vertical):
            if v_move == "none":
                continue
            edge_key = self._vertical_edge_key(row, col, v_move)
            if edge_key in edge_uses:
                return False
            edge_uses.add(edge_key)

        for col, h_move in enumerate(horizontal):
            if h_move == "none":
                continue
            edge_key = self._horizontal_edge_key(row, col, h_move)
            if edge_key in edge_uses:
                return False
            edge_uses.add(edge_key)

        return True

    def _vertical_edge_key(self, row: int, col: int, direction: str) -> str:
        if direction == "up":
            return f"{row-1},{col}-{row},{col}"
        if direction == "down":
            return f"{row},{col}-{row+1},{col}"
        msg = "Unsupported vertical direction (expected 'up' or 'down')"
        raise ValueError(msg)

    def _horizontal_edge_key(self, row: int, col: int, direction: str) -> str:
        if direction == "left":
            return f"{row},{col-1}-{row},{col}"
        if direction == "right":
            return f"{row},{col}-{row},{col+1}"
        msg = "Unsupported horizontal direction (expected 'left' or 'right')"
        raise ValueError(msg)

    def _compute_next_state(self, prev_state: int, h_state: int) -> int:
        prev_vertical = self._decode_state(prev_state)
        horizontal = self._decode_horizontal_state(h_state)

        next_vertical: List[str] = []
        for col in range(self.n):
            v_move = prev_vertical[col]
            h_move = horizontal[col]

            if v_move == "none" and h_move == "none":
                next_vertical.append("none")
            elif v_move == "up" and h_move == "none":
                # Moved up, no outgoing
                next_vertical.append("none")
            elif v_move == "down" and h_move == "none":
                # Continues down
                next_vertical.append("down")
            else:
                next_vertical.append("none")

        return self._state_to_encode(next_vertical)

    def _state_to_encode(self, vertical: List[str]) -> int:
        state = 0
        base = 1
        for v_move in vertical:
            if v_move == "up":
                state += 1 * base
            elif v_move == "down":
                state += 2 * base
            base *= 4
        return state


class AntGridTester:
    """Simple embedded tester mirroring the Ruby constants.

    Note: Known values were copied from the Ruby code. They do not match the
    canonical Problem 393 values (e.g. f(4) = 88). Treat this as convenience
    regression tests for this translation rather than proof of correctness.
    """

    KNOWN_VALUES: Dict[int, int] = {
        1: 0,
        2: 8,
        3: 184,
        4: 10_944,
    }

    @classmethod
    def run_tests(cls) -> bool:
        """Run sanity tests for the AntGridSolver implementation.

        Returns
        -------
        bool
            True if all tests pass, False otherwise.
        """

        print("Running unit tests for AntGridSolver...")
        passed = 0
        total = 0

        for n, expected in cls.KNOWN_VALUES.items():
            solver = AntGridSolver(n)
            result = solver.compute_f()
            total += 1
            if result == expected:
                print(f"[32m[0m Test n={n} == {expected}")
                passed += 1
            else:
                print(f"x Test n={n} != {expected} (got {result})")

        print(f"Test Summary: {passed}/{total} passed")
        return passed == total


def solve() -> int:
    """Solve PE 393."""
    return AntGridSolver(10).compute_f()


if __name__ == "__main__":
    print(solve())
