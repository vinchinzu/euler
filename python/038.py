#!/usr/bin/env python3
"""
Project Euler Problem 38: Pandigital multiples

Take the number 192 and multiply it by each of 1, 2, and 3:
192 × 1 = 192
192 × 2 = 384
192 × 3 = 576
By concatenating each product we get the 1 to 9 pandigital, 192384576.

What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
concatenated product of an integer with (1,2,…,n) where n > 1?
"""

from typing import Optional


def is_pandigital(num: int) -> bool:
    """Check if number is 1-9 pandigital."""
    s = str(num)
    return len(s) == 9 and set(s) == set('123456789')


def concatenated_product(k: int, n: int) -> Optional[int]:
    """Generate concatenated product of k with (1..n)."""
    parts = []
    total_length = 0
    
    for i in range(1, n + 1):
        product_str = str(k * i)
        new_length = total_length + len(product_str)
        
        if new_length > 9:
            return None
        
        parts.append(product_str)
        total_length = new_length
        
        if total_length == 9:
            break
    
    return int(''.join(parts)) if total_length == 9 else None


def main():
    MAX_K = 9999
    MAX_N = 20
    
    max_pandigital = 0
    
    for k in range(MAX_K, 1, -1):
        for n in range(2, MAX_N + 1):
            product = concatenated_product(k, n)
            if product and is_pandigital(product):
                if product > max_pandigital:
                    max_pandigital = product
    
    print(max_pandigital)


if __name__ == "__main__":
    main()
