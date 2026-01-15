"""Project Euler Problem 835: Supernatural Triangles.

Find the sum of the perimeters of all supernatural triangles (triangles
where two sides are consecutive integers) with perimeter ≤ 10^N.

First suppose one of the legs and the hypotenuse are consecutive, so the
side lengths are a, b, b+1. Then a²=2b+1, so the sides are parameterized
as a=2t+1, b=2t²+2t for t≥1. The perimeter is 4t²+6t+2, and we the
quadratic formula tells us the maximum t is (√(1+4*10^N) - 3) / 4, which
for even N is 10^{N/2})/2 - 1.

Now suppose the two legs are consecutive, so the side lengths are a, a+1, c.
Then we have the Pell equation a²+(a+1)²=c² => (2a+1)²-2c²=-1, and the
perimeter is (2a+1)+c. Since this is a Pell equation, the solutions
satisfy a recurrence, which we can find is p_t = 6p_{t-1}-p_{t-2}. We can
also solve for the closed form formula
p(t) = ((3+2√2)^t - (3-2√2)^t) / 2√2. This converges to (3+2√2)^t / 2√2,
so we can compute the maximum t where the perimeter is at most 10^N. From
this total we subtract 2 (for the degenerate triangle 0,1,1) and 12 (for
the triangle 3,4,5, which is already counted in the first case).
"""

from __future__ import annotations

import math
from typing import Callable


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse."""
    return pow(a, -1, m)


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers from 1 to n mod mod."""
    if k == 0:
        return n % mod
    if k == 1:
        return (n * (n + 1) // 2) % mod
    if k == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    # For higher powers, use Faulhaber's formula (simplified)
    result = 0
    for i in range(1, n + 1):
        result = (result + pow_mod(i, k, mod)) % mod
    return result


def triangular(n: int, mod: int) -> int:
    """Triangular number n(n+1)/2 mod mod."""
    return (n * (n + 1) // 2) % mod


def lagrange_extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def solve() -> int:
    """Solve Problem 835."""
    N = 10**10
    M = 1234567891
    B = 10

    ans = 0

    # First case: leg and hypotenuse consecutive
    limit1 = (pow_mod(B, N // 2, M) * mod_inverse(2, M) % M - 1) % M
    ans = (
        ans
        + 4 * sum_powers(limit1, 2, M)
        + 6 * triangular(limit1, M)
        + 2 * limit1
    ) % M

    # Second case: two legs consecutive (Pell equation)
    sqrt2 = math.sqrt(2)
    log_base = math.log(3 + 2 * sqrt2)
    limit2 = int((N * math.log(B) + math.log(2 * sqrt2)) / log_base)

    def p_sequence(n: int) -> int:
        """Compute p(n) using recurrence."""
        if n == 0:
            return 0
        p = [0] * (n + 2)
        p[1] = 2
        for i in range(2, n + 1):
            p[i] = (6 * p[i - 1] - p[i - 2]) % M
        # Sum prefix
        result = 0
        for i in range(1, n + 1):
            result = (result + p[i]) % M
        return result

    extrap = lagrange_extrapolation(p_sequence, 2, M)
    ans = (ans + extrap(limit2) - 14) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
