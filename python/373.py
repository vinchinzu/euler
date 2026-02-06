#!/usr/bin/env python3
"""
Project Euler Problem 373: Circumscribed Circles

Find S(10^7) = sum of circumradii of all integer-sided triangles with integer
circumradius r <= 10^7.

Algorithm (from Java reference):
For each circumradius r, each triangle side must be twice the leg of a right
triangle with hypotenuse r. So for each r we find all x > 0 with x^2 + y^2 = r^2
and form candidate sides = 2x. Then check all triples (a <= b <= c) for the
circumradius formula: (abc)^2 = r^2 * (a+b+c)(-a+b+c)(a-b+c)(a+b-c).

The number of valid triangles depends only on the sorted exponent signature of r's
prime factors congruent to 1 mod 4, allowing memoization across r values.
"""
from math import isqrt


def solve():
    N = 10**7

    # Smallest prime factor sieve
    spf = list(range(N + 1))
    for i in range(2, isqrt(N) + 1):
        if spf[i] == i:
            for j in range(i * i, N + 1, i):
                if spf[j] == j:
                    spf[j] = i

    def get_exponent_signature(n):
        """Get sorted tuple of exponents of primes = 1 mod 4 in factorization of n."""
        exps = []
        while n > 1:
            p = spf[n]
            e = 0
            while n % p == 0:
                n //= p
                e += 1
            if p % 4 == 1:
                exps.append(e)
        exps.sort()
        return tuple(exps)

    cache = {}
    total = 0

    for r in range(1, N + 1):
        key = get_exponent_signature(r)

        if key in cache:
            total += cache[key] * r
            continue

        # Find all x > 0 with x^2 + y^2 = r^2, y >= 0
        sides = []
        r2 = r * r
        for x in range(1, r + 1):
            y2 = r2 - x * x
            if y2 < 0:
                break
            y = isqrt(y2)
            if y * y == y2:
                sides.append(2 * x)

        # Count triangles (a <= b <= c) with circumradius = r
        # (abc)^2 = r^2 * (a+b+c)(-a+b+c)(a-b+c)(a+b-c)
        num_triangles = 0
        n_sides = len(sides)
        for i in range(n_sides):
            a = sides[i]
            for j in range(i, n_sides):
                b = sides[j]
                for k in range(j, n_sides):
                    c = sides[k]
                    if a + b <= c:
                        break
                    s2 = a + b + c
                    p1 = -a + b + c
                    p2 = a - b + c
                    p3 = a + b - c
                    lhs = (a * b * c) ** 2
                    rhs = r2 * s2 * p1 * p2 * p3
                    if lhs == rhs:
                        num_triangles += 1

        cache[key] = num_triangles
        total += num_triangles * r

    return total


if __name__ == "__main__":
    print(solve())
