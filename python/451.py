"""Project Euler Problem 451: Modular inverses.

Find Σ_{n=3}^N l(n), where l(n) is the largest number smaller than n-1 whose
multiplicative inverse is itself.

l(n) is the largest square root of 1 (mod n), other than n-1.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def mod_inv(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def mod(a: int, m: int) -> int:
    """Modulo operation."""
    return a % m


def solve() -> int:
    """Solve Problem 451."""
    N = 20_000_000
    primes = [p for p in sieve_primes(N) if p >= 3]
    ans = 0

    def helper(min_index: int, n: int, sqrts: List[int]) -> None:
        """Recursive helper to compute square roots modulo n."""
        nonlocal ans
        l = 0
        for sqrt_val in sqrts:
            if sqrt_val < n - 1 and sqrt_val > l:
                l = sqrt_val
        ans += l

        for index in range(min_index, len(primes)):
            p = primes[index]
            if n * p > N:
                break
            pe = p
            while n * pe <= N:
                new_sqrts = []
                for sqrt_val in sqrts:
                    pe_inv = mod_inv(pe, n)
                    n_inv = (1 - pe * pe_inv) // n
                    new_sqrts.append(
                        mod(sqrt_val * pe * pe_inv + n * n_inv, n * pe)
                    )
                    new_sqrts.append(
                        mod(sqrt_val * pe * pe_inv - n * n_inv, n * pe)
                    )
                helper(index + 1, n * pe, new_sqrts)
                pe *= p

    helper(0, 1, [0])
    helper(0, 2, [1])
    helper(0, 4, [1, 3])
    pow2 = 8
    while pow2 <= N:
        helper(0, pow2, [1, pow2 // 2 - 1, pow2 // 2 + 1, pow2 - 1])
        pow2 *= 2

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
