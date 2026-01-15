#!/usr/bin/env python3
"""
Project Euler Problem 40: Champernowne's constant

An irrational decimal fraction is created by concatenating the positive integers:
0.123456789101112131415161718192021...

Find d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
"""

from math import prod


def main():
    positions = [1, 10, 100, 1000, 10000, 100000, 1000000]
    result_digits = []
    
    current_length = 0
    number = 1
    next_pos_index = 0
    
    while next_pos_index < len(positions):
        num_str = str(number)
        if current_length + len(num_str) >= positions[next_pos_index]:
            while next_pos_index < len(positions) and current_length + len(num_str) >= positions[next_pos_index]:
                offset = positions[next_pos_index] - current_length - 1
                result_digits.append(int(num_str[offset]))
                next_pos_index += 1
        current_length += len(num_str)
        number += 1
    
    print(prod(result_digits))


if __name__ == "__main__":
    main()
