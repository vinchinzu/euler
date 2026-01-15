"""Project Euler Problem 692: Siegbert and Jo.

Let H(N) be the minimum number of pebbles to remove on the first turn of
Fibonacci Nim in order to guarantee a win. Find G(N) = sum_{k=1}^N H(k).

It turns out that H(N) is the smallest number in the Zeckendorf
representation of N. This means that if F is the largest Fibonacci number
less than N, then G(N) = G(F) + G(N - F - 1) + H(N).
"""

from __future__ import annotations


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number."""
    if n <= 1:
        return n
    a, b = 1, 1
    for _ in range(2, n):
        a, b = b, a + b
    return b


def solve() -> int:
    """Solve Problem 692."""
    N = 23416728348467685

    a = 1
    b = 1
    i = 2
    while fibonacci(i) < N:
        c = a + b + fibonacci(i)
        a = b
        b = c
        i += 1

    return b


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
