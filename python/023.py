#!/usr/bin/env python3
"""
Project Euler Problem 23: Non-abundant sums

Find the sum of all positive integers which cannot be written as the sum of two abundant numbers.

A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
A number n is called deficient if the sum of its proper divisors is less than n and it is called
abundant if this sum exceeds n. As 12 is the smallest abundant number, 1+2+3+4+6=16, the smallest
number that can be written as the sum of two abundant numbers is 24. By mathematical analysis,
it can be shown that all integers greater than 28123 can be written as the sum of two abundant
numbers. Find the sum of all the positive integers which cannot be written as the sum of two
abundant numbers.
"""

from typing import Set


def precompute_divisor_sums(limit: int) -> list[int]:
    """Sieve-like precomputation of all divisor sums up to limit."""
    divisor_sums = [0] * (limit + 1)
    
    for i in range(1, limit + 1):
        for multiple in range(2 * i, limit + 1, i):
            divisor_sums[multiple] += i
    
    return divisor_sums


def generate_abundant_numbers(limit: int, divisor_sums: list[int]) -> list[int]:
    """Generate all abundant numbers up to the limit."""
    abundants = []
    for n in range(12, limit + 1):
        if divisor_sums[n] > n:
            abundants.append(n)
    return abundants


def generate_possible_sums(abundants: list[int], limit: int) -> Set[int]:
    """Generate all possible sums of two abundant numbers."""
    possible_sums = set()
    n = len(abundants)
    
    for i in range(n):
        for j in range(i, n):
            sum_val = abundants[i] + abundants[j]
            if sum_val > limit:
                break
            possible_sums.add(sum_val)
    
    return possible_sums


def main():
    ABUNDANT_LIMIT = 28123
    
    divisor_sums = precompute_divisor_sums(ABUNDANT_LIMIT)
    abundants = generate_abundant_numbers(ABUNDANT_LIMIT, divisor_sums)
    possible_sums = generate_possible_sums(abundants, ABUNDANT_LIMIT)
    
    total = sum(n for n in range(1, ABUNDANT_LIMIT + 1) if n not in possible_sums)
    print(total)


if __name__ == "__main__":
    main()
