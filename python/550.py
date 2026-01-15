"""Project Euler Problem 550: Divisor game.

Two players play a game with k piles of stones, taking turns replacing a pile
of n stones with two piles, each one with size a proper divisor of n greater
than 1, until the player with no valid moves loses. Find the number of winning
positions of K piles, each with 2 to N stones.

First we compute the nimber N(n) of a pile of n stones, by taking mex N(a) ^ N(b)
for all pairs of proper divisors a,b of n greater than 1. As an optimization,
we note that the nimber only depends on the exponents of the prime factorization
of n, so k*p1, k*p2, ... all have the same nimber. We can use this to quickly
compute the number of n with any particular nimber value N(n).

A winning position is then any set of K piles such that the bitwise XOR of all
nimbers is nonzero. We can use the Hadamard transform to get exactly the number
of K-pile games with each total nimber, and finally add up the counts for K piles
with nonzero nimber.
"""

from __future__ import annotations

from math import isqrt
from typing import Dict, List, Tuple


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def all_divisors(n: int) -> List[int]:
    """Return all divisors of n."""
    divisors = []
    i = 1
    while i * i <= n:
        if n % i == 0:
            divisors.append(i)
            if i * i != n:
                divisors.append(n // i)
        i += 1
    divisors.sort()
    return divisors


def num_primes_up_to(limit: int) -> Dict[int, int]:
    """Return a dictionary mapping n to the number of primes <= n."""
    primes = sieve(limit)
    result: Dict[int, int] = {}
    prime_set = set(primes)
    count = 0
    for i in range(limit + 1):
        if i in prime_set:
            count += 1
        result[i] = count
    return result


def xor_convolution_power(arr: List[int], exp: int, mod: int) -> List[int]:
    """Compute arr raised to exp power using XOR convolution.

    For XOR convolution, we use the Fast Walsh-Hadamard Transform (FWHT).
    The transform for XOR is: T[i] = sum over j (arr[j] * (-1)^popcount(i&j))
    """
    n = len(arr)
    # Ensure n is a power of 2
    while n & (n - 1):
        n += 1
    if len(arr) < n:
        arr = arr + [0] * (n - len(arr))

    def fwht(a: List[int], inv: bool = False) -> List[int]:
        """Fast Walsh-Hadamard Transform for XOR convolution."""
        n = len(a)
        a = a[:]
        j = 0
        for i in range(1, n):
            bit = n >> 1
            while j & bit:
                j ^= bit
                bit >>= 1
            j ^= bit
            if i < j:
                a[i], a[j] = a[j], a[i]

        length = 2
        while length <= n:
            for i in range(0, n, length):
                for j in range(i, i + length // 2):
                    u = a[j]
                    v = a[j + length // 2]
                    a[j] = (u + v) % mod
                    a[j + length // 2] = (u - v) % mod
            length <<= 1
        if inv:
            inv_n = pow(n, mod - 2, mod)
            a = [(x * inv_n) % mod for x in a]
        return a

    # Apply FWHT
    transformed = fwht(arr)
    # Raise to power element-wise
    powered = [pow(x, exp, mod) for x in transformed]
    # Inverse transform
    result = fwht(powered, inv=True)
    return result[:L]


def solve() -> int:
    """Solve Problem 550."""
    N = 10**7
    K = 10**12
    L = 64
    M = 987654321

    primes = sieve(N)
    num_primes_dict = num_primes_up_to(N)

    nimbers = [0] * (N + 1)
    counts = [0] * L

    def helper(
        min_index: int,
        n: int,
        primes_list: List[int],
        nimbers_arr: List[int],
        counts_arr: List[int],
        num_primes: Dict[int, int],
    ) -> None:
        """Recursively compute nimbers for all numbers."""
        if n > 1:
            used = [False] * L
            divisors = all_divisors(n)
            # Check all pairs of proper divisors (excluding 1 and n)
            for i in range(1, len(divisors) - 1):
                for j in range(i, len(divisors) - 1):
                    d1 = divisors[i]
                    d2 = divisors[j]
                    xor_val = nimbers_arr[d1] ^ nimbers_arr[d2]
                    if xor_val < L:
                        used[xor_val] = True
            # Compute mex (minimum excluded value)
            while nimbers_arr[n] < L and used[nimbers_arr[n]]:
                nimbers_arr[n] += 1
            if nimbers_arr[n] < L:
                counts_arr[nimbers_arr[n]] += 1

        # Recursively process multiples
        for index in range(min_index, len(primes_list)):
            p = primes_list[index]
            if n * p > N:
                break
            new_n = n
            while new_n * p <= N:
                new_n *= p
                helper(
                    index + 1,
                    new_n,
                    primes_list,
                    nimbers_arr,
                    counts_arr,
                    num_primes,
                )
            # Optimization: if n * p^2 > N, we can batch count remaining
            if n * p * p > N:
                if n * p <= N:
                    nim_val = nimbers_arr[n * p]
                    if nim_val < L:
                        # Count numbers of form n * q where q > p and q is prime
                        # and n * q <= N
                        if n > 1:
                            prev_p = primes_list[index - 1] if index > 0 else 0
                            count_to_add = (
                                num_primes.get(N // n, 0)
                                - num_primes.get(prev_p, 0)
                                - 1
                            )
                            if count_to_add > 0:
                                counts_arr[nim_val] += count_to_add
                return

    helper(0, 1, primes, nimbers, counts, num_primes_dict)

    # Compute K-th power using XOR convolution
    pow_counts = xor_convolution_power(counts, K, M)

    # Sum counts for nonzero nimbers
    ans = sum(pow_counts[i] for i in range(1, L)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
