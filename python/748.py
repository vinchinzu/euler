"""Project Euler Problem 748: Upside Down Diophantine Equation.

Find the sum of all solutions to 1/x² + 1/y² = 13/z² such that x≤y, x,y,z ≤
N, and GCD(x,y,z) = 1.

We can write this as (z/x)² + (z/y)² = 13, and we can parameterize rational
solutions by drawing a line with slope m/n from the solution (3,2) to each
other solution. The other point is (x',y') = (3+d, 2-d*m/n) for some d, and
solving gives (x',y') = (3m²+4mn-3n²)/(m²+n²),
(-2m²+6mn+2n²)/(m²+n²), which gives us our parameterization. Further, we
can divide these polynomials to see that GCD(x,y,z) contains a factor of 4
if m≡n (mod 2), and contains a factor of 13² if m≡8n (mod 13).
"""

from __future__ import annotations

from math import gcd, isqrt, pow as math_pow


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def gcd_table(limit: int) -> list[list[int]]:
    """Precompute GCD table."""
    result = [[0] * (limit + 1) for _ in range(limit + 1)]
    for i in range(1, limit + 1):
        for j in range(1, limit + 1):
            result[i][j] = gcd(i, j)
    return result


def solve() -> int:
    """Solve Problem 748."""
    n = 10**16
    m = 10**9
    a = (isqrt(26) - 4) / (6 - isqrt(26))
    b = 2 / (isqrt(13) - 3)

    limit = int(math_pow(n * 4 / 8, 0.25))
    gcds = gcd_table(limit)

    ans = 0

    def process(m_val: int, n_val: int, g: int) -> None:
        """Process a solution."""
        nonlocal ans
        a_val = sq(m_val) + sq(n_val)
        b_val = -2 * sq(m_val) + 6 * m_val * n_val + 2 * sq(n_val)
        c_val = 3 * sq(m_val) + 4 * m_val * n_val - 3 * sq(n_val)
        x = a_val * b_val // g
        y = a_val * c_val // g
        z = b_val * c_val // g
        if y <= n and z <= n and y > 0 and z > 0:
            ans = (ans + x + y + z) % m

    # First section
    for n_val in range(1, int(isqrt(4 * n / 8)) + 1):
        for m_val in range(n_val + 1, int(b * n_val) + 1):
            if (
                sq(m_val) + sq(n_val)
            ) * (3 * sq(m_val) + 4 * m_val * n_val - 3 * sq(n_val)) <= 4 * n:
                if (
                    m_val > a * n_val
                    and gcds[m_val % n_val][n_val] == 1
                    and (2 * m_val - 3 * n_val) % 13 != 0
                ):
                    g = 4 if (m_val + n_val) % 2 == 0 else 1
                    process(m_val, n_val, g)

    # Second section
    for n_val in range(1, int(isqrt(676 * n / 8)) + 1):
        m_start = n_val + (7 * n_val) % 13
        for m_val in range(m_start, int(b * n_val) + 1, 13):
            if (
                sq(m_val) + sq(n_val)
            ) * (3 * sq(m_val) + 4 * m_val * n_val - 3 * sq(n_val)) <= 676 * n:
                if m_val > a * n_val and gcd(m_val, n_val) == 1:
                    g = 676 if (m_val + n_val) % 2 == 0 else 169
                    process(m_val, n_val, g)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
