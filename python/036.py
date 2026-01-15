#!/usr/bin/env python3
"""
Project Euler Problem 36: Double-base palindromes

The decimal number, 585 = 1001001001_2 (binary), is palindromic in both bases.
Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
"""


def is_palindrome(s: str) -> bool:
    """Check if a string is palindromic."""
    return s == s[::-1]


def to_binary(n: int) -> str:
    """Convert integer to binary string."""
    return bin(n)[2:]


def main():
    LIMIT = 1_000_000
    total = 0
    
    for num in range(LIMIT):
        decimal_str = str(num)
        if not is_palindrome(decimal_str):
            continue
        
        binary_str = to_binary(num)
        if is_palindrome(binary_str):
            total += num
    
    print(total)


if __name__ == "__main__":
    main()
