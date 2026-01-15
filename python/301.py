"""Project Euler Problem 301: Counting losing Nim positions.

For how many positive integers n <= 2**30 does n ^ 2n ^ 3n == 0?

The losing positions are exactly those n whose binary representation has no
consecutive 1s. The count of such numbers up to 2^k is F(k+1) where F is
the Fibonacci sequence with F(1)=1, F(2)=2.

For n <= 2^30, we need F(31) = 2178309.
"""

from __future__ import annotations


def solve() -> int:
    """Return the count of n <= 2^30 where n ^ 2n ^ 3n == 0.

    This equals the number of integers up to 2^30 with no consecutive 1s
    in binary representation, which is F(31) where F(1)=1, F(2)=2.
    """
    # Generate Fibonacci numbers up to F(31)
    fib_prev, fib_curr = 1, 2
    for _ in range(29):
        fib_prev, fib_curr = fib_curr, fib_prev + fib_curr
    return fib_curr


if __name__ == "__main__":
    print(solve())
