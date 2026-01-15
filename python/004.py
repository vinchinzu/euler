#!/usr/bin/env python3
"""
Project Euler Problem 4: Largest palindrome product

A palindromic number reads the same both ways.
Find the largest palindrome made from the product of two 3-digit numbers.
"""


def is_palindrome(num: int) -> bool:
    """Check if a number is a palindrome using integer reversal."""
    if num < 0:
        return False
    original = num
    reversed_num = 0
    while num > 0:
        reversed_num = reversed_num * 10 + num % 10
        num //= 10
    return original == reversed_num


def find_largest_palindrome_product(min_num: int, max_num: int) -> int:
    """Find the largest palindrome product of two n-digit numbers."""
    largest_palindrome = 0
    
    # Descend from largest to smallest to find maximum early
    for i in range(max_num, min_num - 1, -1):
        for j in range(i, min_num - 1, -1):
            product = i * j
            
            # Early exit if product can't beat current maximum
            if product <= largest_palindrome:
                break
            
            if is_palindrome(product) and product > largest_palindrome:
                largest_palindrome = product
    
    return largest_palindrome


def main():
    result = find_largest_palindrome_product(100, 999)
    print(result)


if __name__ == "__main__":
    main()
