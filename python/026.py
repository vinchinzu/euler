#!/usr/bin/env python3
"""
Project Euler Problem 26: Reciprocal cycles

A unit fraction contains 1 in the numerator. The decimal representation of the unit
fractions with denominators 2 to 10 are given. Find the value of d < 1000 for which
1/d contains the longest recurring cycle in its decimal fraction part.
"""

from typing import Tuple


def decimal_cycle_length(d: int, limit: int = 1000) -> int:
    """Compute the length of the recurring cycle in 1/d."""
    if d <= 0 or d >= limit:
        raise ValueError("d must be positive and less than limit")
    
    # Decimals terminate if d shares factors with 10 (2 or 5)
    if d % 2 == 0 or d % 5 == 0:
        return 0
    
    # Simulate long division: track remainders to detect cycle
    remainders = {}
    remainder = 1
    digit_pos = 0
    
    while remainder != 0 and remainder not in remainders:
        remainders[remainder] = digit_pos
        remainder *= 10
        remainder %= d
        digit_pos += 1
    
    if remainder == 0:
        return 0
    return digit_pos - remainders[remainder]


def main():
    LIMIT = 1000
    
    # Filter candidates (multiples of 2 or 5 have no recurring cycles)
    candidates = [d for d in range(1, LIMIT) if d % 2 != 0 and d % 5 != 0]
    
    best_d = max(candidates, key=lambda d: decimal_cycle_length(d, LIMIT))
    print(best_d)


if __name__ == "__main__":
    main()
