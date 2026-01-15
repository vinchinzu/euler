#!/usr/bin/env python3
"""
Project Euler Problem 44: Pentagon numbers

Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference
are pentagonal and D = |Pk - Pj| is minimised; what is the value of D?
"""

from math import isqrt


def pentagonal(n: int) -> int:
    """Generate the nth pentagonal number."""
    return n * (3 * n - 1) // 2


def is_pentagonal(n: int) -> bool:
    """Check if n is a pentagonal number."""
    # n = k(3k-1)/2 => 24n + 1 = (6k-1)^2
    d = 1 + 24 * n
    root = isqrt(d)
    return root * root == d and (1 + root) % 6 == 0


def main():
    LIMIT = 3000
    pent_set = set(pentagonal(n) for n in range(1, LIMIT + 1))
    
    min_d = float('inf')
    
    for i in range(1, LIMIT + 1):
        pi = pentagonal(i)
        for j in range(i + 1, LIMIT + 1):
            pj = pentagonal(j)
            diff = pj - pi
            if diff > min_d:
                break
            if diff in pent_set and (pi + pj) in pent_set:
                min_d = diff
    
    print(min_d)


if __name__ == "__main__":
    main()
