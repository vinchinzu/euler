#!/usr/bin/env python3
"""
Square root digital expansion (Problem 80)

Compute the total of the digital sums of the first one hundred digits of the
square roots of the first one hundred natural numbers, excluding perfect
squares. Uses integer arithmetic to avoid precision loss.
"""

DIGITS = 100
SCALE = 10 ** DIGITS

def integer_sqrt(n):
    """Integer square root using Newton's method."""
    if n == 0:
        return 0
    x = n
    y = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x

def digital_sum_for_sqrt(n):
    """Calculate digital sum of first DIGITS of sqrt(n), excluding perfect squares."""
    root = integer_sqrt(n)
    if root * root == n:  # skip perfect squares
        return 0

    scaled_root = integer_sqrt(n * SCALE * SCALE)

    integer_part = scaled_root // SCALE
    fractional_part = scaled_root % SCALE

    digits = str(integer_part) + str(fractional_part).rjust(DIGITS, '0')
    total = 0
    for ch in digits[:DIGITS]:
        total += ord(ch) - 48  # Convert char to digit
    return total

total = 0
for n in range(1, 101):
    total += digital_sum_for_sqrt(n)

print(total)
