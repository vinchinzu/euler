"""Project Euler Problem 769: Binary Quadratic Form.

Find the number of ways that a perfect square z² can be represented as
x²+5xy+3y², where z ≤ N.

Similar to p143, we find a parameterization of (x,y,z) for the equation
x²+5xy+y² = z². The equation goes through (-1,0) and we are interested in
points in the first quadrant, so let s=m/n be the slope of a line through
(-1,0) and a rational point in the first quadrant. We must have 0≤s<1/√3.
Substituting y=s(1+x) gives x² + 5x(1+x)s + 3(1+x)²s² = 1, which has the
solution x = (1-3s²)/(1+5s+3s²) and y = (5s²+2s)/(1+5s+3s²), which gives:

x = n²-3m²
y = 5m²+2mn
z = 3m²+5mn+n².

We are interested in relatively prime (m,n), with m≤n/√3, and
3m²+5mn+n²≤N. For each n, we can compute the largest possible m. We can
remove the relatively prime constraint in the usual way.

Finally, we note that it is possible for h = (x,y,z) to be divisible by 13.
This happens when m≡-3n (mod 13). So we remove (m,n) that satisfies
m≡-3n (mod 13), and then add them back when iterating over the range
3m²+5mn+n²≤13N.
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


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b:
        a, b = b, a % b
    return a


def solve() -> int:
    """Solve Problem 769."""
    N = 10**14
    sqrt_N = isqrt(N)

    mobius = pre_mobius(sqrt_N)
    ans = 0

    for g in range(1, sqrt_N + 1):
        if sq(g) > N:
            break
        for h in [1, 13]:
            n_max = int((h * N / sq(g)) ** 0.5)
            for n in range(1, n_max + 1):
                if sq(g * n) > h * N:
                    break
                max_m = int(
                    min(
                        n / (3**0.5),
                        ((13 * sq(n) + 12 * h * N / sq(g)) ** 0.5 - 5 * n) / 6,
                    )
                )
                if (g % 13 == 0) == (h == 13):
                    ans += mobius[g] * max_m
                if g % 13 != 0:
                    sign = -1 if h == 1 else 1
                    ans += sign * mobius[g] * (max_m + 3 * n % 13) // 13

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
