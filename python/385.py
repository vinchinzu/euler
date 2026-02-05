#!/usr/bin/env python3
"""Project Euler Problem 385 - Ellipse Inside Triangle

Find sum of areas of triangles with integer coordinates ≤ N where the maximal
inscribed ellipse has foci at (±√13, 0).

Solution uses Pell equations and Marden's Theorem.
"""

from math import isqrt, gcd

def is_perfect_square(n):
    """Check if n is a perfect square"""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n

def solve_pell(D, N_val):
    """Find solutions to X² - D*Y² = N_val with Y ≤ limit"""
    solutions = []

    if N_val == 0:
        return solutions

    # Find fundamental solution to x² - D*y² = 1
    if D == 3:
        x0, y0 = 2, 1  # 2² - 3*1² = 1
    else:
        # General case (simplified for this problem)
        return solutions

    # For negative Pell equation x² - D*y² = N_val
    # Try small values
    limit = 10**9
    for y in range(1, min(100000, limit + 1)):
        val = N_val + D * y * y
        if is_perfect_square(val):
            x = isqrt(val)
            solutions.append((x, y))
            # Generate more solutions using fundamental solution
            for _ in range(10):  # Generate a few more
                x_new = x * x0 + D * y * y0
                y_new = x * y0 + y * x0
                if y_new > limit:
                    break
                solutions.append((x_new, y_new))
                x, y = x_new, y_new

    return solutions

def shoelace_area(x1, y1, x2, y2, x3, y3):
    """Calculate triangle area using shoelace formula"""
    return abs(x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) // 2

def solve():
    N = 10**9
    K = 13

    triangles = set()

    # Iterate over n from 1 to 12K
    for n in range(1, 12 * K + 1):
        # Check if sqrt(3n(12K-n)) is an integer
        if not is_perfect_square(3 * n * (12 * K - n)):
            continue

        # Solve Pell equation X² - 3Y² = n
        solutions = solve_pell(3, n)

        for X, Y in solutions:
            if Y > N:
                break

            # Check if x1 is an integer
            num = (12 * K - n) * X * X
            denom = 3 * n

            if num % denom != 0:
                continue

            x1_sq = num // denom
            if not is_perfect_square(x1_sq):
                continue

            abs_x1 = isqrt(x1_sq)

            for x1 in [abs_x1, -abs_x1]:
                y1 = Y
                x2 = (X - x1) // 2
                y2 = -(x2 + 2 * x1) * y1 // X

                x3 = -(x2 + x1)
                y3 = -(y2 + y1)

                # Check all coordinates are within bounds
                coords = [x1, y1, x2, y2, x3, y3]
                if all(abs(c) <= N for c in coords):
                    # Store as frozenset of points to avoid duplicates
                    triangle = frozenset([(x1, y1), (x2, y2), (x3, y3)])
                    triangles.add(triangle)

    # Sum areas
    total = 0
    for triangle in triangles:
        points = list(triangle)
        if len(points) == 3:
            x1, y1 = points[0]
            x2, y2 = points[1]
            x3, y3 = points[2]
            total += shoelace_area(x1, y1, x2, y2, x3, y3)

    return total

if __name__ == "__main__":
    print(solve())
