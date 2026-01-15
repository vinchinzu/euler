#!/usr/bin/env python3
"""
Project Euler Problem 43: Sub-string divisibility

Find the sum of all 0 to 9 pandigital numbers with the property that
d2d3d4 is divisible by 2, d3d4d5 is divisible by 3, etc.
"""

from itertools import permutations


def main():
    DIVISORS = [2, 3, 5, 7, 11, 13, 17]
    total = 0
    
    for perm in permutations('0123456789', 10):
        if perm[0] == '0':
            continue
        
        valid = True
        for i, div in enumerate(DIVISORS):
            num = int(''.join(perm[i+1:i+4]))
            if num % div != 0:
                valid = False
                break
        
        if valid:
            total += int(''.join(perm))
    
    print(total)


if __name__ == "__main__":
    main()
