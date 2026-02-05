#!/usr/bin/env python3
"""Project Euler Problem 397 - Triangle on Parabola

Count quadruplets (k, a, b, c) with 1 <= k <= K, -N <= a < b < c <= N where
triangle on parabola y = x^2/k at x=a, x=b, x=c contains at least one 45 degree angle.

Solution ported from Java reference using efficient divisor enumeration.
"""

from math import isqrt, floor


def truncdiv(a, b):
    """Java-style integer division: truncates towards zero."""
    if (a >= 0) == (b >= 0):
        return abs(a) // abs(b)
    else:
        return -(abs(a) // abs(b))


def all_divisors(n, prime_set):
    """Generate all divisors of n given its set of prime factors."""
    divs = [1]
    for p in prime_set:
        if n % p != 0:
            continue
        e = 0
        temp = n
        while temp % p == 0:
            e += 1
            temp //= p
        new_divs = []
        pk = 1
        for _ in range(e + 1):
            for dd in divs:
                new_divs.append(dd * pk)
            pk *= p
        divs = new_divs
    return divs


def solve():
    K = 10**6
    N = 10**9

    max_val = 2 * K
    smallest_prime = [0] * (max_val + 1)
    for i in range(2, max_val + 1):
        if smallest_prime[i] == 0:
            for j in range(i, max_val + 1, i):
                if smallest_prime[j] == 0:
                    smallest_prime[j] = i

    def get_prime_set(n):
        factors = set()
        while n > 1:
            p = smallest_prime[n]
            factors.add(p)
            while n % p == 0:
                n //= p
        return factors

    ans = 0

    for k in range(1, K + 1):
        prod = 2 * k * k
        prime_set = get_prime_set(2 * k)
        divs = all_divisors(prod, prime_set)

        for d in divs:
            # First case
            a_plus_b = -(k + d)
            b_plus_c = prod // d + k
            # Java integer division truncates towards zero
            min_b = max(truncdiv(a_plus_b + 1, 2), b_plus_c - N)
            max_b = min(truncdiv(b_plus_c - 1, 2), a_plus_b + N)
            if min_b <= max_b:
                ans += max_b - min_b + 1

            # Check for double-counted triangles
            if prod % (d + 2 * k) == 0:
                a_plus_c = prod // (d + 2 * k) - k
                total = a_plus_b + b_plus_c + a_plus_c
                if total % 2 == 0:
                    a = total // 2 - b_plus_c
                    c = total // 2 - a_plus_b
                    if -N <= a and c <= N:
                        ans -= 2

            # Second case
            a_plus_b = k - d
            b_plus_c = prod // d - k
            # Java: Math.floor(b_plus_c / 2.) gives floor division
            min_b = int(floor(b_plus_c / 2.0)) + 1
            min_b = max(min_b, b_plus_c - N)
            max_b = min(a_plus_b + N, N)
            if min_b <= max_b:
                ans += 2 * (max_b - min_b + 1)

            # Check for double-counted triangles
            if d != 2 * k and prod % (2 * k - d) == 0:
                a_plus_c = k - prod // (2 * k - d)
                total = a_plus_b + b_plus_c + a_plus_c
                if total % 2 == 0:
                    a = total // 2 - b_plus_c
                    b = total // 2 - a_plus_c
                    c = total // 2 - a_plus_b
                    if -N <= a and c < b and b <= N:
                        ans -= 1

    return ans


if __name__ == "__main__":
    print(solve())
