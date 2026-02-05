"""Project Euler Problem 348: Sum of a Square and a Cube

Find palindromic numbers that can be expressed as sum of a square and a cube
in exactly 4 different ways. Sum the five smallest such palindromes.
"""

from __future__ import annotations
import math


def is_palindrome(n: int) -> bool:
    """Check if a number is a palindrome."""
    s = str(n)
    return s == s[::-1]


def generate_palindromes(max_val: int) -> list[int]:
    """Generate all palindromic numbers up to max_val in sorted order.

    Constructs palindromes by mirroring digits rather than checking all numbers.
    """
    palindromes = []

    # Single digit palindromes: 1-9
    for d in range(1, 10):
        if d <= max_val:
            palindromes.append(d)

    # Generate palindromes by length
    # For even length 2k: take k-digit number, mirror it
    # For odd length 2k+1: take k+1 digit number, mirror all but last digit

    length = 2
    while True:
        if length % 2 == 0:
            # Even length palindromes
            half_len = length // 2
            start = 10 ** (half_len - 1)
            end = 10 ** half_len
            for half in range(start, end):
                s = str(half)
                palindrome = int(s + s[::-1])
                if palindrome > max_val:
                    break
                palindromes.append(palindrome)
            else:
                length += 1
                continue
            # If we broke out early, check if we should stop entirely
            if int(str(start) + str(start)[::-1]) > max_val:
                break
            length += 1
        else:
            # Odd length palindromes
            half_len = length // 2 + 1
            start = 10 ** (half_len - 1)
            end = 10 ** half_len
            for half in range(start, end):
                s = str(half)
                palindrome = int(s + s[-2::-1])
                if palindrome > max_val:
                    break
                palindromes.append(palindrome)
            else:
                length += 1
                continue
            # If we broke out early, check if we should stop entirely
            if int(str(start) + str(start)[-2::-1]) > max_val:
                break
            length += 1

    palindromes.sort()
    return palindromes


def count_representations(n: int) -> int:
    """Count ways to express n as a^2 + b^3 where a, b > 1.

    Iterate b from 2 to cbrt(n), compute remainder = n - b^3,
    check if remainder is a perfect square > 1.
    """
    count = 0
    b = 2
    while b ** 3 < n:
        b_cubed = b ** 3
        remainder = n - b_cubed
        if remainder > 1:
            a = int(math.isqrt(remainder))
            if a > 1 and a * a == remainder:
                count += 1
        b += 1
    return count


def solve() -> int:
    """Solve PE 348 for palindromes up to 10^9.

    Find numbers that are palindromes AND can be written as a^2 + b^3
    (where a, b > 1) in exactly 4 different ways.
    Return the sum of the five smallest such palindromes.
    """
    max_val = 10 ** 9
    palindromes = generate_palindromes(max_val)

    results = []
    for p in palindromes:
        if p < 28:  # Minimum is 2^2 + 2^3 = 12, but we need 4 ways
            continue
        ways = count_representations(p)
        if ways == 4:
            results.append(p)
            if len(results) == 5:
                break

    return sum(results)


if __name__ == "__main__":
    print(solve())
