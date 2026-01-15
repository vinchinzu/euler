#!/usr/bin/env python3
"""
Project Euler Problem 75: Singular integer right triangles

Given that L is the length of wire, for how many values of L <= 1,500,000
can exactly one integer sided right angle triangle be formed?
"""

import math

LIMIT = 1_500_000

def count_singular_triangles():
    """
    We use Euclid's formula for Pythagorean triples:
    a = k * (m^2 - n^2)
    b = k * (2mn)
    c = k * (m^2 + n^2)
    where m > n > 0, m and n are coprime, and one of m, n is even.
    The perimeter L = a + b + c = k * (m^2 - n^2 + 2mn + m^2 + n^2)
    L = k * (2m^2 + 2mn) = 2km(m+n).
    """
    perimeter_counts = [0] * (LIMIT + 1)

    # Determine upper bound for m
    # Since L = 2km(m+n), and k >= 1, n >= 1:
    # L >= 2m(m+1) > 2m^2.
    # So, 2m^2 < LIMIT => m^2 < LIMIT/2 => m < sqrt(LIMIT/2).
    m_limit = int(math.sqrt(LIMIT / 2.0))

    # Loop for m
    for m in range(2, m_limit + 1):
        # Loop for n
        for n in range(1, m):
            # Conditions for (m,n) to form a primitive triple:
            # 1. m and n are coprime
            # 2. m and n have different parities (m-n is odd)
            if not ((m - n) % 2 == 1 and math.gcd(m, n) == 1):
                continue

            # Calculate primitive perimeter (k=1)
            primitive_l = 2 * m * (m + n)

            # If primitive_l itself is too large, no need to check its multiples
            if primitive_l > LIMIT:
                continue

            # Loop for k (multiples of primitive triple)
            k = 1
            while True:
                current_l = k * primitive_l
                if current_l > LIMIT:
                    break

                perimeter_counts[current_l] += 1
                k += 1

    # Count how many perimeters can be formed in exactly one way
    count_singular = perimeter_counts.count(1)
    return count_singular


def main():
    result = count_singular_triangles()
    print(result)


if __name__ == "__main__":
    main()
