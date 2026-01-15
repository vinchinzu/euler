"""Project Euler Problem 575: Wandering Robot.

A robot roams in a NxN room. With probability 50%, at each turn it moves to an
existing room or stays in its room with equal probability. Otherwise, with
probability 50%, at each turn it stays in its room with probability 50%,
otherwise it moves to an existing room with equal probability. Find the
probability that after a long period of time, the robot is in a square numbered
room.

The steady state distribution of a random walk is proportional at each vertex
to the vertex's degree. In the first case, each vertex contains an extra edge
that points to itself, so a corner vertex has degree 3, a side vertex degree 4,
and a center vertex degree 5. In the second case, a corner vertex has 2 edges
pointing to itself, a side vertex 3 edges, and a center vertex 4 edges. We then
divide each case by the total to normalize the sum to 1, which is N(5N-4) in
the first case, and 4N(N-1) in the second, and average the two results
together.
"""

from __future__ import annotations

from math import isqrt


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def solve() -> float:
    """Solve Problem 575."""
    N = 1000

    case1 = 0
    case2 = 0

    for i in range(N):
        for j in range(N):
            room_num = i * N + j + 1
            if is_square(room_num):
                is_corner = (i == 0 or i == N - 1) and (j == 0 or j == N - 1)
                is_side = i == 0 or i == N - 1 or j == 0 or j == N - 1

                if is_corner:
                    case1 += 3
                    case2 += 2
                elif is_side:
                    case1 += 4
                    case2 += 3
                else:
                    case1 += 5
                    case2 += 4

    total1 = N * (5.0 * N - 4)
    total2 = 4.0 * N * (N - 1)
    ans = (case1 / total1 + case2 / total2) / 2.0
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.12f}")
    return result


if __name__ == "__main__":
    main()
