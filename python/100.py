#!/usr/bin/env python3
"""
Project Euler Problem 100: Arranged probability

Find the number of blue discs when the total number of discs exceeds 10^12
and the probability of drawing two blue discs without replacement is 1/2.
"""

TARGET_TOTAL = 1_000_000_000_000

def next_solution(blue, total):
    """Generate next solution of the Diophantine system derived from 2*b*(b-1)=t*(t-1)."""
    next_blue = 3 * blue + 2 * total - 2
    next_total = 4 * blue + 3 * total - 3
    return [next_blue, next_total]

def main():
    """Start from the smallest non-trivial solution (15 blue, 21 total) and iterate until total exceeds required threshold."""
    blue, total = 15, 21

    while total <= TARGET_TOTAL:
        blue, total = next_solution(blue, total)

    print(blue)


if __name__ == "__main__":
    main()
