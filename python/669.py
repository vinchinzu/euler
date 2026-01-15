"""Project Euler Problem 669: The King's Banquet.

The integers 1 to N are arranged such that any two consecutive numbers sum to
a Fibonacci number, and the first number is smaller than the last. Find the
Kth number.

We can arrange the numbers such that t_{N-2d} ≡ -d*a (mod N), where a is the
Fibonacci number before N, and t_{N-2d-1} ≡ d*a.
"""

from __future__ import annotations


def fibonacci_list(max_val: int) -> list[int]:
    """Generate Fibonacci numbers up to max_val."""
    fib = [1, 1]
    while fib[-1] < max_val:
        fib.append(fib[-1] + fib[-2])
    return fib


def solve() -> int:
    """Solve Problem 669."""
    N = 99194853094755497
    K = 10**16

    fibonaccis = fibonacci_list(N)
    # Find largest Fibonacci number < N
    a = fibonaccis[-2] if fibonaccis[-1] >= N else fibonaccis[-1]

    if (N - K) % 2 == 0:
        d = (N - K) // 2
        ans = (-d * a) % N
    else:
        d = (N + 1 - K) // 2
        ans = (d * a) % N

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
