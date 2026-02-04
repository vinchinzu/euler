"""Project Euler Problem 653: Frictionless Tube.

Suppose that N marbles with diameter D are in a long tube of length L and
diameter D with a closed left end and open right end, and each marble at some
location and moving left or right with constant speed v. If collisions between
balls and with the closed left end are perfectly elastic, determine the
distance the Jth ball from the left travels before its center reaches the
rightmost end of the tube.

We can "collapse" each marble into a point by shifting the Jth ball to the left
by D*J - D/2, and reducing the entire tube by (D/2) * N. Then, an elastic
collision is equivalent to two balls traveling directly through each other if
the balls are indistinguishable. This means a ball will move L' - p to the
right end if it is initially moving rightward, or L' + p if initially moving
leftward.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass
class Marble:
    """Marble with gap before and direction."""

    gap_before: int
    is_west: bool


def blum_blum_shub(n: int) -> list[int]:
    """Generate the pseudo-random sequence as specified in problem 653."""
    m = 32745673
    x = 6563116
    result = [x]
    for _ in range(n - 1):
        x = (x * x) % m
        result.append(x)
    return result


def solve() -> int:
    """Solve Problem 653."""
    N = 10**6 + 1
    J = 500001
    D = 20
    L = 10**9

    # Generate marbles using the problem's pseudo-random sequence
    rng = blum_blum_shub(N)
    marbles = []
    for r in rng:
        gap_before = (r % 1000) + 1
        is_west = r > 10_000_000
        marbles.append(Marble(gap_before, is_west))

    pos = 0
    distances = []
    for marble in marbles:
        pos += marble.gap_before
        distance = L - N * D // 2 + (pos if marble.is_west else -pos)
        distances.append(distance)

    distances.sort()
    return distances[N - J]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
