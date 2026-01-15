"""Project Euler Problem 684: Inverse Digit Sum.

Let s(n) be the smallest number with a digit sum of n, and S(k) =
sum_{n=1}^k s(n). Find sum_{i=2}^N S(f_i), where f_i is the i'th Fibonacci
number.

By inspection we can see that s(n) consists of the digit (n%9) followed by
⌊n/9⌋ 9s. This gives a closed formula for S(n).
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number."""
    if n <= 1:
        return n
    a, b = 1, 1
    for _ in range(2, n):
        a, b = b, a + b
    return b


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 684."""
    N = 90
    M = 10**9 + 7
    B = 10

    ans = 0
    for i in range(2, N + 1):
        n = fibonacci(i)
        term = (
            (B // 2 + 1 + n % (B - 1) + tr(n % (B - 1)))
            * pow_mod(B, n // (B - 1), M)
            - (B // 2 + 1 + n)
        ) % M
        ans = (ans + term) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
