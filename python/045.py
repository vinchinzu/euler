#!/usr/bin/env python3
"""
Project Euler Problem 45: Triangular, pentagonal, and hexagonal

Find the next triangle number after 40755 that is also pentagonal and hexagonal.
"""

from math import isqrt


def triangular(n: int) -> int:
    return n * (n + 1) // 2


def is_pentagonal(n: int) -> bool:
    d = 1 + 24 * n
    root = isqrt(d)
    return root * root == d and (1 + root) % 6 == 0


def is_hexagonal(n: int) -> bool:
    d = 1 + 8 * n
    root = isqrt(d)
    return root * root == d and (1 + root) % 4 == 0


def main():
    n = 286  # Starting after T_285
    while True:
        t = triangular(n)
        if is_pentagonal(t) and is_hexagonal(t):
            print(t)
            break
        n += 1


if __name__ == "__main__":
    main()
