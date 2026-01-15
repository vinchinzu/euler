"""Project Euler Problem 582: Integer sided triangles with 120° angle.

Find the number of integer sided triangles with a 120° angle and sides
a ≤ b ≤ c satisfying b - a ≤ K, and c ≤ N.

By the Law of Cosines we have a² + a*b + b² = c². Letting d = b - a,
this becomes a² + a(a+d) + (a+d)² = c² => (2c)² - 3(2a + d)² = d².

This is a Pell equation. We can plug in all values of d ≤ K, and filter
only the solutions where 2c is even and c ≤ N.
"""

from __future__ import annotations

from math import isqrt
from typing import List

from sympy import primerange


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


class PellSolution:
    """Represents a solution to a Pell equation."""

    def __init__(self, x: int, y: int) -> None:
        """Initialize a Pell solution."""
        self.x = x
        self.y = y


def solve_pell_general(d: int, n: int) -> List[PellSolution]:
    """Solve the general Pell equation x² - d*y² = n.

    Returns solutions until x exceeds a threshold.
    """
    if is_square(d):
        return []

    # Find fundamental solution to x² - d*y² = 1
    a0 = isqrt(d)
    m = 0
    den = 1
    a = a0

    h_prev, k_prev = 1, 0
    h, k = a0, 1

    fundamental = None
    max_iterations = 10000

    for _ in range(max_iterations):
        if h * h - d * k * k == 1:
            fundamental = PellSolution(h, k)
            break

        m = den * a - m
        den = (d - m * m) // den
        a = (a0 + m) // den

        h_next = a * h + h_prev
        k_next = a * k + k_prev

        h_prev, k_prev = h, k
        h, k = h_next, k_next

    if fundamental is None:
        return []

    # Find a particular solution to x² - d*y² = n
    # Try small values of y
    solutions: List[PellSolution] = []
    for y0 in range(0, 1000):
        x_sq = n + d * y0 * y0
        if x_sq < 0:
            continue
        x0 = isqrt(x_sq)
        if x0 * x0 == x_sq:
            solutions.append(PellSolution(x0, y0))
            break

    if not solutions:
        return []

    # Generate more solutions using the fundamental solution
    x0, y0 = solutions[0].x, solutions[0].y
    u, v = fundamental.x, fundamental.y

    x, y = x0, y0
    while True:
        x_next = u * x + d * v * y
        y_next = u * y + v * x
        x, y = x_next, y_next
        if x > 10**100:  # Large enough threshold
            break
        solutions.append(PellSolution(x, y))

    return solutions


def solve() -> int:
    """Solve Problem 582."""
    N = 10**100  # This is a BigInteger in Java, we'll use int approximation
    K = 100

    ans = 0

    for d in range(1, K + 1):
        # Solve Pell equation (2c)² - 3(2a + d)² = d²
        # This is x² - 3*y² = d² where x = 2c, y = 2a + d
        solutions = solve_pell_general(3, d * d)

        for sol in solutions:
            # Check if x (which is 2c) is even
            if sol.x % 2 != 0:
                continue

            c = sol.x // 2
            if c > N:
                break

            # Check if y > d (which means 2a + d > d, so a > 0)
            if sol.y > d:
                ans += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
