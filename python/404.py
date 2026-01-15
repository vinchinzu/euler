"""Project Euler Problem 404: Crisscross Ellipses.

Rotate the ellipse x² + 4y² = 4a² by an angle θ. The rotated ellipse
intersects the original ellipse at four points, with two distinct distances
b ≤ c to the origin. Find the number of canonical ellipsoidal triplets
(a, b, c) where a,b,c are integers and a ≤ N.

The equation of the ellipse in polar coordinates is r = 2a / √(1 + 3 cos²ϕ).
For any canonical ellipsoidal triplet, we have two angles ϕ1 and ϕ2, 90°
apart, such that:
b = 2a / √(1 + 3 cos²(ϕ1))
c = 2a / √(1 + 3 cos²(ϕ2)) = 2a / √(1 + 3 sin²(ϕ1))

We can write these as cos²(ϕ1) = ((2a/b)² - 1) / 3 and sin²(ϕ1) =
((2a/c)² - 1) / 3, and using the identity cos²ϕ + sin²ϕ = 1 gives
(2a/b)² + (2a/c)² = 5.

Let (b, c) = g, b = g*x, and c = g*y. Then (2a/x)² + (2a/y)² = 5g², and
since (x, y) = 1, 2a must be divisible by both: 2a = x*y*k. Plugging that
in gives (y*k)² + (x*k)² = 5g².

Assume without loss of generality that k = 1, because we can generate
solutions where k > 1 by scaling. Then (x + y*i)(x - y*i) = 5g², and as x
and y are relatively prime, we must either have 1 + 2i | x + y*i or
1 - 2i | x + y*i.

In the first case, x + y*i = (1 + 2i)(x' + y'i) and x - y*i = (1 - 2i)(x' -
y'i), and (x' + y'i)(x' - y'i) = (x')² + (y')² = g². We can generate
solutions (x', y', g) using the parameterization for primitive Pythagorean
triples: x' = m² - n², y' = 2mn, g = m² + n². From that we can compute
x = |m² - n² - 4mn|, y = 2m² - 2n² + 2mn. We have a geometric restriction
that b ≥ a => g ≥ y/2 => m ≤ 2n. This yields a base solution of a = x*y/2,
and a total of ⌊N / (x*y/2)⌋ solutions. The base solution is only valid if
(x, y) = 1, which is true as long as x and y do not share a factor of 5.
We can stop when x*y > 2N => 4n⁴ > N.

In the second case, x + y*i = (1 - 2i)(x' + y'i), so x = m² - n² + 4mn and
y = |2m² - 2n² - 2mn|. The geometric restriction c ≥ a implies g ≥ x/2 =>
m ≥ 3n. Again, the number of solutions is ⌊N / (x*y/2)⌋ and is valid if
(x, y) ≠ 5, and we can stop when x*y > 2N => 20n⁴ > N.

As a final optimization, we precompute GCDs up to the constraint that
4n⁴ ≤ N.
"""

from __future__ import annotations

from math import gcd, isqrt


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def gcds(limit: int) -> list[list[int]]:
    """Precompute GCDs up to limit."""
    result = [[0] * (i + 1) for i in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, i + 1):
            result[i][j] = gcd(i, j)
    return result


def solve() -> int:
    """Solve Problem 404."""
    N = 10**17

    limit = int((N / 4) ** 0.25)
    gcds_table = gcds(limit)

    ans = 0

    # First case: m ≤ 2n
    for n in range(1, limit + 1):
        if 4 * n**4 > N:
            break
        for m in range(n + 1, 2 * n + 1, 2):
            if gcds_table[n][m % n] == 1:
                x = abs(sq(m) - sq(n) - 4 * m * n)
                y = 2 * (sq(m) - sq(n) + m * n)
                if x * y // 2 > N:
                    break
                if x % 5 > 0 or y % 5 > 0:
                    ans += N // (x * y // 2)

    # Second case: m ≥ 3n
    limit2 = int((N / 20) ** 0.25)
    for n in range(1, limit2 + 1):
        if 20 * n**4 > N:
            break
        m = 3 * n + 1
        while True:
            if gcds_table[n][m % n] == 1:
                x = sq(m) - sq(n) + 4 * m * n
                y = 2 * abs(sq(m) - sq(n) - m * n)
                if x * y // 2 > N:
                    break
                if x % 5 > 0 or y % 5 > 0:
                    ans += N // (x * y // 2)
            m += 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
