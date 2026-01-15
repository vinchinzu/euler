"""Project Euler Problem 618: Numbers with a given prime factor sum.

Find sum_{k=2}^N S(F_k), where F_k is the kth Fibonacci number, and S(k) is
the sum of all numbers whose prime factors sum to k.

Let dp[k] be the sum of all numbers whose prime factors sum to k, for the
first few primes p. We repeatedly update this table for each prime p under F_N,
by incrementing each dp[k] by p * dp[k - p]. We update the table from left to
right, so that we allow any number of primes p to be added.
"""

from __future__ import annotations

from sympy import primerange


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number."""
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a


def solve() -> int:
    """Solve Problem 618."""
    N = 24
    M = 10**9

    limit = fibonacci(N)
    dp = [0] * (limit + 1)
    dp[0] = 1

    primes = list(primerange(2, limit + 1))
    for p in primes:
        for k in range(p, limit + 1):
            dp[k] = (dp[k] + p * dp[k - p]) % M

    ans = 0
    for i in range(2, N + 1):
        ans = (ans + dp[fibonacci(i)]) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
