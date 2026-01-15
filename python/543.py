"""Project Euler Problem 543: Prime-Sum Numbers.

Let P(n, k) = 1 if n can be written as the sum of k primes, and 0
otherwise. Let S(n) = Σ P(i,k) for 1 ≤ i,k ≤ n. Find Σ_{n=3}^N S(F_n)
where F_n is the nth Fibonacci number.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def num_primes_up_to(n: int, primes: List[int]) -> int:
    """Count primes ≤ n."""
    count = 0
    for p in primes:
        if p > n:
            break
        count += 1
    return count


def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number."""
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def S(n: int, primes: List[int]) -> int:
    """Compute S(n)."""
    result = num_primes_up_to(n, primes)
    if n >= 4:
        result += n // 2 - 1
        result += num_primes_up_to(n - 2, primes) - 1
        half = n // 2
        if half >= 3:
            result += (n + 1) * (half - 2) - 2 * (tr(half) - 3)
    return result


def solve() -> int:
    """Solve Problem 543."""
    N = 44

    # Estimate max Fibonacci needed
    max_fib = fibonacci(N)
    primes = sieve_primes(max_fib + 100)

    ans = 0
    for k in range(3, N + 1):
        fib_k = fibonacci(k)
        ans += S(fib_k, primes)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
