"""Project Euler Problem 557: Triangle regions.

Find the sum of a+b+c+d for all quadruples (a, b, c, d) that can be the
integer areas of the four regions of a triangle.

Divide d into two halves, the left half d1 and the right half d2. Then we know
b/d1 = (a+b)/(c+d) because they share the same bases. Similarly, c/d2 =
(b+c)/(b+d). Solving for d1 and d2 and plugging into d1 + d2 = d gives

b(c+d)/(a+b) + c(b+d)/(a+c) = d
=> bc(2a+b+c+d) = a²d.

Fix a and S=a+b+c+d. For this to have integer solutions, bc=a²d/(a+S) must be
an integer, so d must be a multiple of (a+S)/gcd(a²,a+S). Then b and c must
satisfy b+c=S-a-d and b*c=a²d/(a+S). This is a quadratic equation with
discriminant (S-a-d)² - 4a²d/(a+S), so it has a solution iff the discriminant
is a perfect square.
"""

from __future__ import annotations

from math import gcd, isqrt


def precompute_gcds(limit: int) -> list[list[int]]:
    """Precompute GCD table."""
    gcds = [[0] * (limit + 1) for _ in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, limit + 1):
            gcds[i][j] = gcd(i, j)
    return gcds


def is_perfect_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def solve() -> int:
    """Solve Problem 557."""
    N = 10000

    gcds = precompute_gcds(2 * N)
    ans = 0

    for a in range(1, N):
        for S in range(a, N + 1):
            a_plus_S = a + S
            a_sq = a * a
            g = gcds[a_plus_S][a_sq % a_plus_S]
            mult = a_plus_S // g

            d = mult
            while d < S:
                disc = (S - a - d) ** 2 - 4 * a_sq * d // a_plus_S
                if disc >= 0 and is_perfect_square(disc):
                    ans += S
                d += mult

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
