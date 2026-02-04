"""Project Euler Problem 261: Pivotal Square Sums.

Call k a square-pivot if there exist m > 0 and n >= k such that the sum of
the m+1 consecutive squares up to k equals the sum of the m consecutive
squares starting from (n+1). Find the sum of all distinct square pivots up to N.

We rewrite the equation as a general Pell equation x^2 - D*y^2 = N_pell,
with D = m*(m+1), N_pell = m^2*(m+1).

The fundamental solution to x^2 - D*y^2 = 1 is always (2m+1, 2).
Base solutions have y <= m and y^2 divisible by m.

From each base solution (x0, y0), we generate further solutions using
Brahmagupta-Pell composition with the fundamental solution.
We also need the conjugate chain from (x0, -y0).
"""

from __future__ import annotations

from math import isqrt


def is_square(n):
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def factorize(n):
    """Return prime factorization as dict {prime: exponent}."""
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def transform_exponents(prime_factors, f):
    """Transform exponents using function f."""
    result = 1
    for p, e in prime_factors.items():
        result *= p ** f(e)
    return result


def solve():
    """Solve Problem 261."""
    N = 10 ** 10
    L = isqrt(N // 2)

    pivots = set()

    for m in range(1, L + 1):
        D = m * (m + 1)

        # Compute sD: squarefree part of D
        factors_m = factorize(m)
        factors_m1 = factorize(m + 1)
        combined = dict(factors_m)
        for p, e in factors_m1.items():
            combined[p] = combined.get(p, 0) + e
        sD = transform_exponents(combined, lambda e: e % 2)

        # sm: product of primes in m raised to ceil(exponent/2)
        sm = transform_exponents(factors_m, lambda e: (e + 1) // 2)

        # Find base solutions: y from 0 to m, step sm
        base_sols = []
        for y in range(0, m + 1, sm):
            res = m + y * y
            if res % sD == 0 and is_square(res // sD):
                x = sD * isqrt(D // sD) * isqrt(res // sD)
                base_sols.append((x, y))

        # Fundamental solution to x^2 - D*y^2 = 1 is (2m+1, 2)
        xf = 2 * m + 1
        yf = 2

        def check_and_add(x, y):
            if x % m == 0 and (x // m - m - 1) % 2 == 0 and (y + m) % 2 == 0:
                n_val = (x // m - m - 1) // 2
                k = (y + m) // 2
                if n_val >= k:
                    pivots.add(k)

        # Generate solutions from each base solution
        # Both the direct chain and the conjugate chain
        for x0, y0 in base_sols:
            # Direct chain: (x0, y0) composed with (xf, yf)^n
            x, y = x0, y0
            while True:
                if y + m > 2 * N:
                    break
                check_and_add(x, y)
                x, y = xf * x + D * yf * y, xf * y + yf * x

            # Conjugate chain: start from (x0, -y0), compose with (xf, yf)
            # (x0, -y0) * (xf, yf) = (xf*x0 - D*yf*y0, yf*x0 - xf*y0)
            if y0 > 0:
                x, y = xf * x0 - D * yf * y0, yf * x0 - xf * y0
                # y might be negative initially; keep composing until y >= 0
                while y < 0:
                    x, y = xf * x + D * yf * y, xf * y + yf * x
                while True:
                    if y + m > 2 * N:
                        break
                    check_and_add(x, y)
                    x, y = xf * x + D * yf * y, xf * y + yf * x

    return sum(pivots)


def main():
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
