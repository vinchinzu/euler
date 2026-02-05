#!/usr/bin/env python3
"""Project Euler Problem 764: Sum of Solutions to 16x^2+y^4=z^2.

Find S(N), the sum of x+y+z for all solutions to 16x^2+y^4=z^2 with
1<=x,y,z<=N and GCD(x,y,z)=1.

Direct translation from Java reference using exact integer arithmetic.
"""

from fractions import Fraction
import math


def solve():
    N = 10**16
    M = 10**9
    L = int((2 * N) ** 0.25) + 10

    # Mobius sieve
    mobius = [1] * (L + 1)
    mobius[0] = 0
    is_prime = [True] * (L + 1)

    for p in range(2, L + 1):
        if is_prime[p]:
            for j in range(p, L + 1, p):
                if j > p:
                    is_prime[j] = False
                if (j // p) % p == 0:
                    mobius[j] = 0
                else:
                    mobius[j] *= -1

    # Sum of fourth powers (as Python ints - exact)
    sumFourthPowers = [0] * (L + 1)
    sumOddFourthPowers = [0] * (L + 1)
    for i in range(1, L + 1):
        sumFourthPowers[i] = sumFourthPowers[i - 1] + i ** 4
        sumOddFourthPowers[i] = sumOddFourthPowers[i - 1] + (2 * i - 1) ** 4

    sqrt2 = math.sqrt(2)

    # Use Fraction for exact arithmetic
    S = Fraction(0)

    for g in range(1, L):
        if mobius[g] == 0:
            continue

        g4 = g ** 4
        n = N // g4
        if n == 0:
            break

        deg4 = Fraction(0)
        deg2 = Fraction(0)

        # Case (1): z-4x = a^4, z+4x = b^4, a and b both odd, a < b
        # x = (b^4 - a^4) / 8, y = ab, z = (a^4 + b^4) / 2
        # x + y + z = (5*b^4 + 3*a^4)/8 + ab
        # Only when g is odd
        if g % 2 == 1:
            b = 1
            while b ** 4 <= 2 * n:
                b4 = b ** 4
                # Java: num_a = Math.min(b / 2, (int) (Math.pow(2 * n - pow(b, 4), 1. / 4) + 1) / 2)
                limit1 = b // 2
                val = (2 * n - b4) ** 0.25 if 2 * n > b4 else 0
                limit2 = (int(val) + 1) // 2
                num_a = min(limit1, limit2)

                if num_a > 0:
                    # Java: deg4 += ((5 * modInvs[8]) * num_a % mod * pow(b, 4, mod)
                    #              + (3 * modInvs[8]) * sumOddFourthPowers[num_a]) % mod
                    deg4 += Fraction(5, 8) * num_a * b4 + Fraction(3, 8) * sumOddFourthPowers[num_a]

                    # Java: deg2 += sq(num_a) * b % mod
                    deg2 += num_a * num_a * b

                b += 2

        # Case (2): z-4x = 2a^4, z+4x = 8b^4
        # x = (8b^4 - 2a^4) / 8 = b^4 - a^4/4, y = 2ab, z = a^4 + 4b^4
        # x + y + z = 5b^4 + 3a^4/4 + 2ab
        b = 1
        while 4 * (b ** 4) <= n:
            b4 = b ** 4
            limit1 = int(sqrt2 * b)
            remaining = n - 4 * b4
            limit2 = int(remaining ** 0.25) if remaining > 0 else 0
            num_a = min(limit1, limit2)

            if g % 2 == 1:
                num_a = num_a // 2

            mult = 1 if g % 2 == 0 else 2

            if num_a > 0:
                # Java: deg4 += (5 * num_a * pow(b, 4, mod)
                #              + (3 * modInvs[4]) * pow(mult, 4) % mod * sumFourthPowers[num_a]) % mod
                deg4 += 5 * num_a * b4 + Fraction(3, 4) * (mult ** 4) * sumFourthPowers[num_a]

                # Java: deg2 += 2 * mult * tr(num_a) * b % mod
                tr_num_a = num_a * (num_a + 1) // 2
                deg2 += 2 * mult * tr_num_a * b

            b += 1

        # Case (3): z-4x = 8a^4, z+4x = 2b^4
        # x = (2b^4 - 8a^4) / 8 = b^4/4 - a^4, y = 2ab, z = 4a^4 + b^4
        # x + y + z = 5b^4/4 + 3a^4 + 2ab
        mult = 1 if g % 2 == 0 else 2
        b = mult
        while b ** 4 <= n:
            b4 = b ** 4
            limit1 = int(b / sqrt2)
            remaining = (n - b4) / 4.0
            limit2 = int(remaining ** 0.25) if remaining > 0 else 0
            num_a = min(limit1, limit2)

            if num_a > 0:
                # Java: deg4 += ((5 * modInvs[4]) * num_a % mod * pow(b, 4, mod)
                #              + 3 * sumFourthPowers[num_a]) % mod
                deg4 += Fraction(5, 4) * num_a * b4 + 3 * sumFourthPowers[num_a]

                # Java: deg2 += 2 * tr(num_a) * b % mod
                tr_num_a = num_a * (num_a + 1) // 2
                deg2 += 2 * tr_num_a * b

            b += mult

        # Java: S += mobius[g] * (deg4 % mod * pow(g, 4, mod) + deg2 % mod * sq(g, mod)) % mod
        S += mobius[g] * (deg4 * g4 + deg2 * (g ** 2))

    # Convert to integer and take mod
    result = int(S)
    return result % M


def main():
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
