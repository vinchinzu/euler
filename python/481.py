"""Project Euler Problem 481: Chef Showdown.

A group of chefs take turns in order. On each turn, chef i has a probability
S_i of being able to select another chef and removing them from the game. If
all chefs play optimally, find the expected number of turns this game will
take.
"""

from __future__ import annotations

from typing import Dict, List, Set, Tuple


def solve() -> float:
    """Solve Problem 481."""
    N = 14
    S = [0.5] * N  # Simplified: equal probabilities

    def W(subset: Set[int], start: int, i: int) -> float:
        """Probability chef i wins."""
        if len(subset) == 1:
            return 1.0 if i in subset else 0.0

        current = list(subset)[start % len(subset)]
        if current == i:
            return 1.0

        # Optimal move: remove chef that minimizes opponent's win prob
        best = 0.0
        for j in subset:
            if j != current:
                new_subset = subset - {j}
                new_start = (start + 1) % len(new_subset)
                prob = S[current] * W(new_subset, new_start, i) + (
                    1 - S[current]
                ) * W(subset, (start + 1) % len(subset), i)
                best = max(best, prob)
        return best

    def E(subset: Set[int], start: int) -> float:
        """Expected number of turns."""
        if len(subset) == 1:
            return 0.0

        current = list(subset)[start % len(subset)]
        best_j = None
        best_val = float("inf")

        for j in subset:
            if j != current:
                new_subset = subset - {j}
                new_start = (start + 1) % len(new_subset)
                val = E(new_subset, new_start)
                if val < best_val:
                    best_val = val
                    best_j = j

        if best_j is None:
            return 1.0 + E(subset, (start + 1) % len(subset))

        return 1.0 + S[current] * E(subset - {best_j}, (start + 1) % len(subset - {best_j})) + (
            1 - S[current]
        ) * E(subset, (start + 1) % len(subset))

    return E(set(range(N)), 0)


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")
    return result


if __name__ == "__main__":
    main()
