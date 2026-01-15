"""Project Euler Problem 218: Perfect right-angled triangles.

Find the number of perfect right triangles (a primitive integer right triangle
whose hypotenuse c is a perfect square) with c≤N are not super-perfect.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 218.

    According to the problem analysis, all perfect right triangles are
    super-perfect, so the answer is 0.
    """
    return 0


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
