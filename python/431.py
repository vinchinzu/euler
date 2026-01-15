"""Project Euler Problem 431: Square Space.

Grain is poured into a (sufficiently deep) cylindrical container with
radius R from a point x away from the center of the container. The grain
forms a conical shape with a repose angle of α degrees. Find the sum of
all values of x such that the amount of empty space in the container, in
cubic units, is a perfect square.

Consider a top-down view of the cylindrical container. Suppose the center
of the cylinder is at O and the grain is being poured from point P. We can
integrate the empty space with respect to an angle θ around P. We draw a
line from P at angle θ, until it hits the edge of the cylinder at point Q.
Note that θ = m∠OPQ, and define β = m∠OQP.

By the law of sines on triangle OPQ, we have sin(β) = x sin(θ) / R. Then
by projecting the other two sides onto side PQ, we find that PQ = x
cos(θ) + R cos(β). The infinitesimal volume can be approximated as a
rectangular pyramid with height PQ, and base lengths d tan(α) and d dθ.
The volume is therefore (d³ tan(α) / 3).

We use Simpson's Rule to get a good numerical approximation of the
integral, and use binary search to find the desired values of x.
"""

from __future__ import annotations

import math


def integrate(func, a: float, b: float, n: int) -> float:
    """Numerical integration using Simpson's rule."""
    h = (b - a) / n
    result = func(a) + func(b)
    for i in range(1, n):
        x = a + i * h
        if i % 2 == 0:
            result += 2 * func(x)
        else:
            result += 4 * func(x)
    return result * h / 3


def is_square(n: float) -> bool:
    """Check if n is a perfect square."""
    root = int(math.sqrt(n))
    return abs(root * root - n) < 1e-9


def solve() -> float:
    """Solve Problem 431."""
    R = 6.0
    ALPHA = 40.0
    alpha_rad = math.radians(ALPHA)

    def V(x: float) -> float:
        """Compute empty space volume for given x."""
        def integrand(theta: float) -> float:
            beta = math.asin(x * math.sin(theta) / R)
            pq = x * math.cos(theta) + R * math.cos(beta)
            return pq**3

        return integrate(integrand, 0, 2 * math.pi, 1000) * math.tan(alpha_rad) / 3

    lower = V(0)
    higher = V(R)
    ans = 0.0

    for v_val in range(1, int(higher) + 1):
        if v_val > lower and is_square(float(v_val)):
            low = 0.0
            high = R
            while abs(high - low) > 1e-12:
                mid = (low + high) / 2
                if V(mid) < v_val:
                    low = mid
                else:
                    high = mid
            ans += low

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9f}")
    return result


if __name__ == "__main__":
    main()
