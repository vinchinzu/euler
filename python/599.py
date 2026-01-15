"""Project Euler Problem 599: Distinct colorings of a 2x2x2 Rubik's Cube.

Find the number of distinct colorings of a 2x2x2 Rubik's Cube using N colors.

The number of corners with 3 colors is N(N-1)(N-2) divided by the 3 ways to
rotate it, the number of corners with 2 colors is N (the doubled color)
times N-1 (the single color), and the number of corners with a single color
is simply N. The number of ways to choose the 8 corners of a 2x2x2 can be
computed with balls and urns. However, the orientation of the last corner
of a 2x2x2 is fixed. This means that if all corners are multicolored (have
at least two colors), then there are 2 additional distinct ways of orienting
the last corner.
"""

from __future__ import annotations

from math import comb


def solve() -> int:
    """Solve Problem 599."""
    N = 10

    num_multicolored_corners = N * (N - 1) * (N - 2) // 3 + N * (N - 1)
    num_corners = num_multicolored_corners + N

    ans = comb(num_corners + 7, 8) + 2 * comb(num_multicolored_corners + 7, 8)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
