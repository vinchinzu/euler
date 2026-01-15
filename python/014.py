#!/usr/bin/env python3
"""
Project Euler Problem 14: Longest Collatz sequence

The following iterative sequence is defined for the set of positive integers:
- n → n/2 (n is even)
- n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1.

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
Although it has not been proved yet (Collatz Problem), it is thought that all starting
numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
"""

MAX_START = 999_999


def next_collatz(n: int) -> int:
    """Compute the next number in the Collatz sequence."""
    if n % 2 == 0:
        return n // 2
    return 3 * n + 1


from typing import Optional


def chain_length(n: int, memo: Optional[dict] = None) -> int:
    """Compute the chain length for a starting number using memoization."""
    if memo is None:
        memo = {}
    
    if n == 1:
        return 1
    
    if n in memo:
        return memo[n]
    
    next_n = next_collatz(n)
    length = 1 + chain_length(next_n, memo)
    memo[n] = length
    
    return length


def main():
    memo = {}
    max_length = 0
    starting_number = 1
    
    for i in range(1, MAX_START + 1):
        length = chain_length(i, memo)
        if length > max_length:
            max_length = length
            starting_number = i
    
    print(starting_number)


if __name__ == "__main__":
    main()
