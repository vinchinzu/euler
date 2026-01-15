"""Project Euler Problem 287: Quadtree encoding.

An image can be described with a quad-tree encoding: 10 represents a
black pixel, 11 represents a white pixel, and 0 represents that 4
recursive descriptions follow. Find the minimum number of bits required
to encode an image where (x, y) is black if (x - 2^(N-1))² + (y -
2^(N-1))² ≤ 2^(2N-2).
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 287."""
    N = 24
    L = 1 << (N - 1)

    def black(x: int, y: int) -> bool:
        """Check if pixel is black."""
        return (x - L) ** 2 + (y - L) ** 2 <= L * L

    def len_encoding(x: int, y: int, side: int) -> int:
        """Compute encoding length."""
        if (
            black(x, y) == black(x + side - 1, y + side - 1)
            and black(x + side - 1, y) == black(x, y + side - 1)
        ):
            return 2
        half = side // 2
        return (
            1
            + len_encoding(x, y, half)
            + len_encoding(x + half, y, half)
            + len_encoding(x, y + half, half)
            + len_encoding(x + half, y + half, half)
        )

    return 1 + len_encoding(0, 0, L) + 2 * len_encoding(L, 0, L) + len_encoding(L, L, L)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
