#!/usr/bin/env python3
"""
Project Euler Problem 32: Pandigital products

Find the sum of all products whose multiplicand/multiplier/product identity
can be written as a 1 through 9 pandigital.
"""

from itertools import permutations


def is_pandigital(n: int) -> bool:
    """Check if a number is 1-9 pandigital."""
    s = str(n)
    return len(s) == 9 and set(s) == set('123456789')


def solve_problem_032():
    """Find all pandigital products and sum them."""
    products = set()
    digits = list(range(1, 10))
    
    for p in permutations(digits):
        # Split 1: a (1 digit), b (4 digits), product (4 digits)
        if (p[0] * p[4]) % 10 == p[8]:
            a = p[0]
            b = int(''.join(str(d) for d in p[1:5]))
            c = int(''.join(str(d) for d in p[5:9]))
            if a * b == c:
                products.add(c)
        
        # Split 2: a (2 digits), b (3 digits), product (4 digits)
        if (p[1] * p[4]) % 10 == p[8]:
            a = int(''.join(str(d) for d in p[0:2]))
            b = int(''.join(str(d) for d in p[2:5]))
            c = int(''.join(str(d) for d in p[5:9]))
            if a * b == c:
                products.add(c)
    
    return sum(products)


def main():
    result = solve_problem_032()
    print(result)


if __name__ == "__main__":
    main()
