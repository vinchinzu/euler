"""Project Euler Problem 513: Triangles with Integer Median.

Find the number of triangles with integer sides a ≤ b ≤ c ≤ N such that the
median to c also has integer length.

By Apollonius's Theorem, a²+b² = 2(d²+m²), where d = c/2 and m is the length
of the median. This means a and b are the same parity, so we can write
a = x-y and b = x+y.

This has a parameterization x-d = k*p, m-y = k*q, m+y = l*p, x+d = l*q,
where (p,q) = 1, and all four values are the same parity.

Finally, we remove the (p,q) = 1 condition in the usual way, by iterating
over all g = (p,q) and multiplying each sum by µ(g).
"""

from __future__ import annotations

from math import isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def num_lattice_points_simple(
    constraints: List[tuple[int, int, int, bool]], n: int
) -> int:
    """Simplified lattice point counting for linear constraints.
    
    Constraints are (a, b, c, strict) meaning a*x + b*y <= c (or < if strict).
    This is a simplified version - full implementation would be more complex.
    """
    # For this problem, we use a brute-force approach for small cases
    # In practice, FloorSums would use more sophisticated algorithms
    count = 0
    for x in range(1, n + 1):
        for y in range(1, n + 1):
            valid = True
            for a, b, c, strict in constraints:
                val = a * x + b * y
                if strict:
                    if val >= c:
                        valid = False
                        break
                else:
                    if val > c:
                        valid = False
                        break
            if valid:
                count += 1
    return count


def f(n: int, check_parity: bool, mobius: List[int]) -> int:
    """Count triangles for given n."""
    L = isqrt(3 * n // 2)
    result = 0
    
    # Simplified implementation - the full version would use FloorSums
    # For now, we use a basic approach
    for l in range(1, L + 1):
        for k in range(1, l):
            if not check_parity or (k % 2 == 0 and l % 2 == 0):
                # Count lattice points with constraints
                constraints = [
                    (0, -1, -l, False),
                    (3 * k + l, -(k + l), 0, False),
                    (-k, l, n, False),
                    (-l, k, 0, False),
                ]
                result += num_lattice_points_simple(constraints, n)
            elif k % 2 == 0 or l % 2 == 0:
                constraints = [
                    (0, -1, -(l + 1) // 2, False),
                    (3 * k + l, -(k + l), 0, False),
                    (-k, l, n // 2, False),
                    (-l, k, 0, False),
                ]
                result += num_lattice_points_simple(constraints, n // 2)
            else:
                constraints = [
                    (-1, -1, -l, False),
                    (2 * k, -(4 * k + 2 * l), 0, False),
                    (l - k, k + l, n, False),
                    (k - l, k + l, 0, False),
                ]
                result += num_lattice_points_simple(constraints, n)
    
    for q in range(1, L + 1):
        for p in range(1, q):
            if not check_parity or (p % 2 == 0 and q % 2 == 0):
                constraints = [
                    (0, -1, -q, True),
                    (q, -p, 0, False),
                    (3 * p - q, -(q - p), 0, False),
                    (-p, q, n, False),
                ]
                result += num_lattice_points_simple(constraints, n)
            elif p % 2 == 0 or q % 2 == 0:
                constraints = [
                    (0, -1, -q // 2, True),
                    (q, -p, 0, False),
                    (3 * p - q, -(q - p), 0, False),
                    (-p, q, n // 2, False),
                ]
                result += num_lattice_points_simple(constraints, n // 2)
            else:
                constraints = [
                    (-1, 1, 0, True),
                    (-1, -1, -q, True),
                    (q - p, -(p + q), 0, False),
                    (4 * p - 2 * q, -2 * p, 0, False),
                    (q - p, p + q, n, False),
                ]
                result += num_lattice_points_simple(constraints, n)
    
    return result


def solve() -> int:
    """Solve Problem 513."""
    N = 100000
    mobius = pre_mobius(N)
    
    ans = 0
    for g in range(1, N + 1):
        ans += mobius[g] * f(N // g, g % 2 == 1, mobius)
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
