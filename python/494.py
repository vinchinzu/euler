"""Project Euler Problem 494: Collatz prefix families.

A Collatz prefix is a Collatz sequence that ends right before a power of 2.
Find the number of distinct families of Collatz prefixes of N terms.
"""

from __future__ import annotations


def fibonacci(n: int) -> int:
    """Fibonacci number."""
    a, b = 1, 1
    for _ in range(n - 1):
        a, b = b, a + b
    return a


def solve() -> int:
    """Solve Problem 494."""
    N = 90
    # The number of sequences is F_n (Fibonacci number)
    ans = fibonacci(N)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
