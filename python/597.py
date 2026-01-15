"""Project Euler Problem 597: Torpedo.

N boats starting at points 0, D, 2D, ... on the number line row in the
positive direction, each boat j at a constant speed of -log X_j where
X_j is chosen randomly in [0, 1]. The finish line is located at L, but
if boat i hits boat j before they cross the finish line, then boat i is
removed from the race. Find the probability that the final order is an
even permutation of the original order.
"""

from __future__ import annotations

from itertools import permutations
from typing import List


class Bump:
    """Represents a bump event."""

    def __init__(self, prev: int, bumper: int, bumped: int, after: int) -> None:
        """Initialize bump."""
        self.prev = prev
        self.bumper = bumper
        self.bumped = bumped
        self.after = after


def solve() -> float:
    """Solve Problem 597."""
    N = 13
    L = 1800.0
    D = 40.0

    spots = [0.0] * (N + 2)
    for i in range(N):
        spots[i + 1] = D * i
    spots[N + 1] = L

    ans = 0.0

    def helper(boats: List[int], spots: List[float], bumps: List[Bump], last_bumped: int) -> None:
        """Recursive helper."""
        nonlocal ans
        if len(bumps) == N:
            parity = 0
            constant = 1.0
            exponents = [0.0] * (N + 2)

            for i in range(len(bumps)):
                bump = bumps[i]
                bumper = bump.bumper
                bumped = bump.bumped

                if i + 1 < len(bumps) and bumps[i + 1].bumper == bumped:
                    next_bumper = bump.bumped
                    next_bumped = bump.after
                else:
                    next_bumper = bump.prev
                    next_bumped = bump.bumped

                parity += bumper - bump.prev
                exponent = exponents[bumper] + 1
                constant /= exponent
                pow_val = (spots[bumped] - spots[bumper]) / (
                    spots[next_bumped] - spots[next_bumper]
                )
                exponents[next_bumper] += exponent * pow_val
                exponents[next_bumped] -= exponent * pow_val
                exponents[bumped] += exponent

            if parity % 2 == N % 2:
                ans += constant
            return

        for i in range(1, len(boats)):
            if boats[i] > N or boats[i] > last_bumped:
                break
            new_last_bumper = boats.pop(i)
            new_last_bumped = boats[i]
            bumps.append(
                Bump(boats[i - 1], new_last_bumper, new_last_bumped, boats[i + 1])
            )
            helper(boats, spots, bumps, new_last_bumped)
            boats.insert(i, new_last_bumper)
            bumps.pop()

    boats = list(range(N + 2))
    helper(boats, spots, [], N)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.10f}")


if __name__ == "__main__":
    main()
