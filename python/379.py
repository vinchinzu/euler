#!/usr/bin/env python3
"""
Project Euler Problem 379: Least common multiple count

Let f(n) = number of pairs (x, y) with 1 <= x <= y and lcm(x, y) <= n.
Find g(N) = sum_{n=1}^{N} f(n) for N = 10^12.

By Mobius inversion:
g(N) = sum_d mu(d) * T(N/d^2)
where T(m) = number of ordered triples (a,b,c) with a*b*c <= m.
Then g(N) = (sum + N) / 2 to account for x <= y ordering.

T(m) is computed in O(m^{2/3}) time using the divisor-sum hyperbola method.
"""
from math import isqrt


def solve():
    N = 10**12
    L = isqrt(N)

    # Sieve Mobius function up to sqrt(N) using linear sieve
    mobius = [0] * (L + 1)
    mobius[1] = 1
    is_prime = [True] * (L + 1)
    primes = []
    for i in range(2, L + 1):
        if is_prime[i]:
            primes.append(i)
            mobius[i] = -1
        for p in primes:
            if i * p > L:
                break
            is_prime[i * p] = False
            if i % p == 0:
                mobius[i * p] = 0
                break
            mobius[i * p] = -mobius[i]

    def D(n):
        """Compute sum_{k=1}^{n} floor(n/k) in O(sqrt(n)) time."""
        if n <= 0:
            return 0
        sq = isqrt(n)
        s = 0
        for k in range(1, sq + 1):
            s += n // k
        return 2 * s - sq * sq

    def num_triplets_with_product_at_most(m):
        """Count ordered triples (a, b, c) with a*b*c <= m, a,b,c >= 1.

        T(m) = sum_{a=1}^{m} D(floor(m/a))

        Split at a = cbrt(m):
        - Part 1: a = 1..cbrt_m, compute D(m//a) directly
        - Part 2: a > cbrt_m, group by distinct values of m//a
        """
        if m <= 0:
            return 0

        cbrt_m = int(round(m ** (1.0 / 3)))
        while (cbrt_m + 1) ** 3 <= m:
            cbrt_m += 1
        while cbrt_m ** 3 > m:
            cbrt_m -= 1

        total = 0
        # Part 1: a = 1..cbrt_m
        for a in range(1, cbrt_m + 1):
            total += D(m // a)

        # Part 2: a > cbrt_m, group by v = m//a
        a = cbrt_m + 1
        while a <= m:
            v = m // a
            a_max = m // v
            total += D(v) * (a_max - a + 1)
            a = a_max + 1

        return total

    ans = 0
    for d in range(1, L + 1):
        if mobius[d] != 0:
            ans += mobius[d] * num_triplets_with_product_at_most(N // (d * d))
    ans += N
    ans //= 2
    return ans


if __name__ == "__main__":
    print(solve())
