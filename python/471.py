"""Project Euler Problem 471: Triangle inscribed in ellipse.

Let r(a, b) be the radius of a circle centered at (2b, 0) such that it is the
in-circle of a triangle with one vertex at (a/2, √3/2 b) and all vertices on
the ellipse x²/a² + y²/b² = 1. Find G(N) = Σ_{a=3}^N Σ_{b=1}^⌊(a-1)/2⌋ r(a, b).
"""

from __future__ import annotations


def harmonic(n: float) -> float:
    """Harmonic number."""
    result = 0.0
    for i in range(1, int(n) + 1):
        result += 1.0 / i
    return result


def solve() -> float:
    """Solve Problem 471."""
    N = 10.0**11
    ans = (
        N * (2 * N - 1) * (3 * N + 4) / 24
        - N * (N + 1) * (2 * N + 1) * (harmonic(N) - harmonic(N / 2)) / 6
    )
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9e}")
    return result


if __name__ == "__main__":
    main()
