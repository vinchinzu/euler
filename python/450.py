"""Project Euler Problem 450: Hypocycloid lattice points.

Let C(R, r) be the distinct lattice points on the hypocycloid parameterized by:

x(t) = (R-r) cos(t) + r cos ((R-r)/r t)
y(t) = (R-r) sin(t) - r sin ((R-r)/r t),

for which sin(t) and cos(t) is rational. Find the sum of |x|+|y| over all these
lattice points, over all hypocycloids given by integers 1 ≤ 2r < R ≤ N.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import Callable, List, Tuple


def pre_phi(limit: int) -> List[int]:
    """Precompute Euler's totient function."""
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] = phi[j] // i * (i - 1)
    return phi


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


def all_divisors(n: int) -> List[int]:
    """Return all divisors of n."""
    divisors = []
    for i in range(1, isqrt(n) + 1):
        if n % i == 0:
            divisors.append(i)
            if i * i != n:
                divisors.append(n // i)
    return divisors


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def sq(n: int) -> int:
    """Square."""
    return n * n


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def pythagorean_triples(
    condition: Callable[[int, int, int], bool]
) -> List[Tuple[int, int, int]]:
    """Generate Pythagorean triples (a, b, c) where a² + b² = c².

    Uses Euclid's formula: a = m² - n², b = 2mn, c = m² + n²
    where m > n > 0, gcd(m, n) = 1, and m and n have opposite parity.
    """
    triples = []
    limit = 1000  # Reasonable limit for generating triples
    for m in range(2, limit):
        for n in range(1, m):
            if (m + n) % 2 == 1 and gcd(m, n) == 1:
                a = m * m - n * n
                b = 2 * m * n
                c = m * m + n * n
                if condition(m, n, c):
                    triples.append((a, b, c))
                else:
                    break
    return triples


def solve() -> int:
    """Solve Problem 450."""
    N = 1_000_000
    phi = pre_phi(2 * N)
    mobius = pre_mobius(2 * N)
    ans = 0

    # Cardinal angles case
    for S in range(3, N + 1):
        ans += 2 * phi[S] * tr(N // S) * S
        if S % 4 != 0:
            res = 0
            for d in all_divisors(S):
                res += d * tr((S - 1) // 2 // d) * mobius[d]
            ans -= 2 * (2 if S % 2 == 0 else 1) * tr(N // S) * res

    # Non-cardinal angles case - Pythagorean triples
    triples = pythagorean_triples(
        lambda m, n, k: 3 * sq(k * (m * m + n * n)) <= N
    )

    for tr_a, tr_b, tr_c in triples:
        if gcd(tr_a, tr_b) != 1:
            continue
        c = tr_c
        for order_x, order_y in [(tr_a, tr_b), (tr_b, tr_a)]:
            for sin_val in [-order_x, order_x]:
                for cos_val in [-order_y, order_y]:
                    for n in range(2, 100):  # Reasonable limit
                        common = pow(c, max(n - 1, 2))
                        if common > N:
                            break
                        for m in range(1, n):
                            if gcd(m, n) == 1:
                                for k in range(1, N // common + 1):
                                    r = k * m * common
                                    R = k * (m + n) * common
                                    if R > N:
                                        break

                                    # Chebyshev polynomials T and U
                                    T = [1, cos_val]
                                    for i in range(2, n + 1):
                                        T.append(
                                            2 * cos_val * T[-1]
                                            - sq(c) * T[-2]
                                        )
                                    U = [1, 2 * cos_val]
                                    for i in range(2, n):
                                        U.append(
                                            2 * cos_val * U[-1]
                                            - sq(c) * U[-2]
                                        )

                                    x_cn = (
                                        (R - r)
                                        * pow(c, n - m)
                                        * T[m]
                                        + r * T[n]
                                    )
                                    y_cn = (
                                        (R - r)
                                        * pow(c, n - m)
                                        * sin_val
                                        * U[m - 1]
                                        - r * sin_val * U[n - 1]
                                    )
                                    c_power_n = pow(c, n)
                                    if x_cn % c_power_n == 0 and y_cn % c_power_n == 0:
                                        ans += abs(x_cn // c_power_n) + abs(
                                            y_cn // c_power_n
                                        )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
