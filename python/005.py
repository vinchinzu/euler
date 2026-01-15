#!/usr/bin/env python3
"""
Project Euler Problem 5: Smallest multiple

What is the smallest positive number that is evenly divisible by all of
the numbers from 1 to 20?
"""


def gcd(a: int, b: int) -> int:
    """Compute greatest common divisor."""
    while b:
        a, b = b, a % b
    return a


def lcm(a: int, b: int) -> int:
    """Compute least common multiple."""
    return abs(a * b) // gcd(a, b)


def smallest_multiple(n: int = 20) -> int:
    """Find the smallest positive integer divisible by all integers from 1 to n."""
    if n < 1:
        return 1
    
    result = 1
    for i in range(1, n + 1):
        result = lcm(result, i)
    
    return result


def main():
    result = smallest_multiple(20)
    print(result)


if __name__ == "__main__":
    main()
