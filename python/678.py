#!/usr/bin/env python3
"""Project Euler Problem 678: Fermat-like Equations.

Find the number of positive integer tuples (a,b,c,e,f) such that a^e + b^e = c^f
for some a < b, e >= 2, f >= 3, and c^f <= N.

Direct translation from Java reference implementation.
"""

import math
from collections import Counter


def solve():
    N = 10**18

    # Compute limit for smallest prime factor sieve
    limit = int(N ** (1/3)) + 100

    # Smallest prime factor sieve
    ff = list(range(limit + 1))
    for i in range(2, int(limit**0.5) + 1):
        if ff[i] == i:  # i is prime
            for j in range(i * i, limit + 1, i):
                if ff[j] == j:
                    ff[j] = i

    def is_sq(n):
        """Check if n is a perfect square."""
        if n < 0:
            return False
        r = int(math.isqrt(n))
        return r * r == n

    def sums_of_two_squares(prime_factors):
        """
        Find all ways to express a number (given by its prime factorization)
        as a sum of two squares x^2 + y^2 where x <= y.

        Uses Gaussian integers: for each prime p = 1 (mod 4), p = (a+bi)(a-bi).
        For prime 2: 2 = (1+i)(1-i) = -i(1+i)^2.
        Primes p = 3 (mod 4) must appear with even exponent.
        """
        # Count prime factors
        factor_count = Counter(prime_factors)

        # Check if expressible as sum of two squares
        for p, exp in factor_count.items():
            if p % 4 == 3 and exp % 2 == 1:
                return set()  # Not expressible

        # Gaussian factorization approach
        # For each prime p = 1 (mod 4), find a, b such that p = a^2 + b^2
        def find_gaussian_factor(p):
            """Find (a, b) such that a^2 + b^2 = p for prime p = 1 (mod 4)."""
            # Find quadratic non-residue
            x = 2
            while pow(x, (p - 1) // 2, p) != p - 1:
                x += 1

            # r = x^((p-1)/4) mod p is sqrt(-1) mod p
            r = pow(x, (p - 1) // 4, p)

            # Euclidean algorithm
            sqrt_p = int(math.isqrt(p))
            m, n = p, r
            while n > sqrt_p:
                m, n = n, m % n

            a = n
            b = int(math.isqrt(p - n * n))
            return (min(a, b), max(a, b))

        # Enumerate all sums of two squares using Gaussian integer products
        # Start with (1, 0) representing 1
        results = [(1, 0)]  # List of (real, imag) Gaussian integers

        # Process prime 2: 2 = (1+i)(1-i), contributes (1+i) each time
        # 2^k contributes rotation and scaling
        exp_2 = factor_count.get(2, 0)
        for _ in range(exp_2):
            new_results = []
            for (re, im) in results:
                # Multiply by (1+i): (re + im*i) * (1+i) = (re - im) + (re + im)*i
                new_results.append((re - im, re + im))
            results = new_results

        # Process primes p = 1 (mod 4)
        for p, exp in factor_count.items():
            if p == 2:
                continue
            if p % 4 == 1:
                a, b = find_gaussian_factor(p)
                # p = (a + bi)(a - bi)
                # p^exp contributes products of (a+bi)^k * (a-bi)^(exp-k) for k = 0..exp

                # Precompute powers of (a+bi) and (a-bi)
                pow_pos = [(1, 0)]  # powers of (a + bi)
                pow_neg = [(1, 0)]  # powers of (a - bi)
                for _ in range(exp):
                    re, im = pow_pos[-1]
                    # (re + im*i) * (a + bi) = (re*a - im*b) + (re*b + im*a)*i
                    pow_pos.append((re * a - im * b, re * b + im * a))
                    re, im = pow_neg[-1]
                    # (re + im*i) * (a - bi) = (re*a + im*b) + (-re*b + im*a)*i
                    pow_neg.append((re * a + im * b, -re * b + im * a))

                new_results = []
                for (re, im) in results:
                    for k in range(exp + 1):
                        # Multiply by (a+bi)^k * (a-bi)^(exp-k)
                        fre, fim = pow_pos[k]
                        gre, gim = pow_neg[exp - k]
                        # (fre + fim*i) * (gre + gim*i)
                        pre = fre * gre - fim * gim
                        pim = fre * gim + fim * gre
                        # Now multiply with (re, im)
                        nre = re * pre - im * pim
                        nim = re * pim + im * pre
                        new_results.append((nre, nim))
                results = new_results

            elif p % 4 == 3:
                # p^exp where exp is even, contributes p^(exp/2) to both real and imaginary scaling
                scale = p ** (exp // 2)
                results = [(re * scale, im * scale) for (re, im) in results]

        # Convert Gaussian integers to (x, y) pairs with x^2 + y^2 = n and x <= y
        pairs = set()
        for (re, im) in results:
            x, y = abs(re), abs(im)
            if x > y:
                x, y = y, x
            pairs.add((x, y))

        return pairs

    def get_all_divisors(n, prime_factors):
        """Get all divisors of n given its prime factorization."""
        factor_count = Counter(prime_factors)
        divisors = [1]
        for p, exp in factor_count.items():
            new_divisors = []
            p_power = 1
            for _ in range(exp + 1):
                for d in divisors:
                    new_divisors.append(d * p_power)
                p_power *= p
            divisors = new_divisors
        return divisors

    # Precompute sums a^e + b^e for e >= 5
    counts = {}
    for e in range(5, 64):
        if (1 << e) >= N:
            break
        pows = []
        a = 1
        while True:
            ae = a ** e
            if ae >= N:
                break
            pows.append(ae)
            a += 1

        cfs = Counter()
        for i in range(len(pows)):
            for j in range(i + 1, len(pows)):
                cf = pows[i] + pows[j]
                if cf <= N:
                    cfs[cf] += 1
        counts[e] = cfs

    ans = 0

    # Iterate over all c^f where f >= 3 and c^f <= N
    f = 3
    while (2 ** f) <= N:
        c = 2
        while True:
            cf = c ** f
            if cf > N:
                break

            # Get prime factorization of c^f (which is just factors of c, each repeated f times)
            prime_factors = []
            cc = c
            while cc > 1:
                p = ff[cc]
                for _ in range(f):
                    prime_factors.append(p)
                cc //= p

            # e = 2: Count sums of two squares
            pairs = sums_of_two_squares(prime_factors)
            for (x, y) in pairs:
                if x > 0 and x < y:
                    ans += 1

            # e = 3: Count sums of two cubes
            # c^f = a^3 + b^3 = (a+b)(a^2 - ab + b^2)
            # For divisor d = a + b, we need a^2 - ab + b^2 = cf / d
            # Let s = a + b = d, then a^2 + b^2 = s^2 - 2ab
            # a^2 - ab + b^2 = s^2 - 3ab = cf/d
            # So ab = (s^2 - cf/d) / 3 = (d^2 - cf/d) / 3
            # For real a, b: discriminant = s^2 - 4ab = d^2 - 4(d^2 - cf/d)/3 = (4cf/d - d^2)/3 >= 0
            # So 4cf/d >= d^2, i.e., d^3 <= 4cf
            # Also need (4cf/d - d^2) to be divisible by 3 and be a perfect square
            # And sqrt(...) < d for a < b
            divisors = get_all_divisors(cf, prime_factors)
            for d in divisors:
                if d * d * d >= 4 * cf:
                    continue
                disc = 4 * cf // d - d * d
                # Check: disc < 3 * d^2 and 3 * disc is a perfect square
                if disc < 3 * d * d and is_sq(3 * disc):
                    ans += 1

            # e = 4: Count sums of two fourth powers
            # Filter sums of two squares where both components are squares
            for (x, y) in pairs:
                if x > 0 and x < y and is_sq(x) and is_sq(y):
                    ans += 1

            # e >= 5: Lookup precomputed
            for e in range(5, 64):
                if (2 ** e) >= cf:
                    break
                if e in counts and cf in counts[e]:
                    ans += counts[e][cf]

            c += 1
        f += 1

    return ans


def main():
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
