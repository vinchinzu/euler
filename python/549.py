"""Project Euler Problem 549: Divisibility of Factorials.

Let s(n) be the smallest m such that n | m!. Find Σ_{i=2}^N s(i).

If n = Π (p_i)^(e_i), then s(n) = max s((p_i)^(e_i)), and we can compute
s((p_i)^(e_i)) quickly by checking multiples of p.
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


def sq(n: int) -> int:
    """Square."""
    return n * n


def num_factors_in_factorial(m: int, p: int) -> int:
    """Count factors of p in m!."""
    count = 0
    power = p
    while power <= m:
        count += m // power
        power *= p
    return count


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def solve() -> int:
    """Solve Problem 549."""
    N = 10**8
    sqrt_n = isqrt(N)
    primes = sieve_primes(2 * sqrt_n)
    
    # Simplified sum of primes helper
    def sum_primes_div(n: int) -> int:
        """Sum primes divisible by n."""
        result = 0
        for p in primes:
            if p > N // n:
                break
            if p % n == 0:
                result += p
        return result
    
    ans = 0

    def helper(min_index: int, n: int, s: int) -> None:
        """Recursive helper."""
        nonlocal ans
        if n > 1:
            ans += s
        
        for index in range(min_index, len(primes)):
            p = primes[index]
            if p > s and n * sq(p) > N:
                if p <= N // n:
                    # Simplified: approximate sum
                    ans += sum_primes_div(n)
                return
            
            new_n = n
            for e in range(1, 100):
                new_n *= p
                if new_n > N:
                    break
                mult = p
                while True:
                    if num_factors_in_factorial(mult, p) >= e:
                        helper(index + 1, new_n, max(mult, s))
                        break
                    mult += p

    helper(0, 1, 0)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
