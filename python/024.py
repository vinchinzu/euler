#!/usr/bin/env python3
"""
Project Euler Problem 24: Lexicographic permutations

Find the millionth lexicographic permutation of the digits 0-9.
"""

from math import factorial


def nth_lexicographic_permutation(digits: list[int], n: int) -> list[int]:
    """Find the nth lexicographic permutation using factorial number system."""
    n -= 1  # zero-based index
    available = digits[:]
    result = []
    
    for i in range(len(digits) - 1, -1, -1):
        fact = factorial(i)
        idx, n = divmod(n, fact)
        result.append(available.pop(idx))
    
    return result


def main():
    result = nth_lexicographic_permutation(list(range(10)), 1_000_000)
    print(''.join(map(str, result)))


if __name__ == "__main__":
    main()
