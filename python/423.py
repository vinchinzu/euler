"""Project Euler Problem 423: Consecutive die throws.

Let C(n) be the number of ways to throw a K-sided die n times such that at
most π(n) consecutive throws are identical, where π(n) is the number of
primes ≤ n. Find Σ_{n=1}^N C(n).

Clearly C(0) = 1. Now suppose that we've already computed C(n-1). Then the
number of ways to throw a die n times with at most π(n) pairs is almost
K * C(n-1), i.e. the number of ways to throw a die n-1 times with at most
π(n-1) pairs, multiplied by K possibilities for the last throw.

If n is prime, then π(n) = π(n-1) + 1. So we're missing some throws - the
first n-1 throws can have π(n) pairs, which is not counted in C(n-1). Let
R(n) be the number of possibilities for the first n-1 throws. To compute R, we
note that there are K ways to throw the first die, nCr(n-2, π(n)) ways to
choose which of the n-2 pairs are identical, and (K-1) ^ (n-2 - π(n)) ways
to select the distinct values for those throws. Finally, the last throw cannot
match the second last throw, so there are K-1 possibilities for the last throw.

R(n) = K * nCr(n-2, π(n)) * (K-1) ^ (n-2 - π(n))
C(n) = K * C(n-1) + (K-1) * R(n),  if p is prime.

If n is not prime, then we over-counted the throws where the last throw
matches the second last throw, and the first n-1 throws already have π(n)
pairs. This is the same as R(n) computed above.

C(n) = K * C(n-1) - R(n),  if p is not prime.

To compute R(n) efficiently, we note that for large enough n, R(n) = R *
(n-2) / π(n) if n is prime, and R(n) = R * (n-2) / (n - 2 - π(n)) * (K-1)
otherwise. For small n, this recursive relation results in divide by zero
errors, so we compute it directly.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[int]:
    """Sieve of Eratosthenes to find all primes up to limit."""
    if limit < 2:
        return []
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, isqrt(limit) + 1):
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False
    return [i for i in range(2, limit + 1) if sieve[i]]


def prime_counts(limit: int) -> List[int]:
    """Count primes up to each number."""
    is_prime_arr = [True] * (limit + 1)
    is_prime_arr[0] = is_prime_arr[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime_arr[i]:
            for j in range(i * i, limit + 1, i):
                is_prime_arr[j] = False

    counts = [0] * (limit + 1)
    count = 0
    for i in range(limit + 1):
        if is_prime_arr[i]:
            count += 1
        counts[i] = count
    return counts


def mod_invs(limit: int, mod: int) -> List[int]:
    """Compute modular inverses for 1..limit modulo mod."""
    invs = [0] * (limit + 1)
    invs[1] = 1
    for i in range(2, limit + 1):
        invs[i] = (mod - (mod // i) * invs[mod % i] % mod) % mod
    return invs


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def nCr(n: int, r: int, mod: int, invs: List[int]) -> int:
    """Compute C(n, r) modulo mod using precomputed inverses."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    
    result = 1
    for i in range(r):
        result = (result * (n - i)) % mod
        result = (result * invs[i + 1]) % mod
    return result


def solve() -> int:
    """Solve Problem 423."""
    N = 50_000_000
    K = 6
    M = 10**9 + 7
    
    prime_count_arr = prime_counts(N)
    mod_invs_arr = mod_invs(N, M)
    
    R = 1
    C = 1
    ans = 0
    
    for n in range(1, N + 1):
        if n == 1:
            ans = (ans + C) % M
            continue
        
        is_prime_n = prime_count_arr[n] == prime_count_arr[n - 1] + 1
        
        if is_prime_n:
            R = R * (n - 2) % M * mod_invs_arr[prime_count_arr[n]] % M
            C = (K * C + (K - 1) * R) % M
        else:
            if n - 2 > prime_count_arr[n]:
                R = (
                    R * (n - 2) % M
                    * mod_invs_arr[n - 2 - prime_count_arr[n]]
                    * (K - 1)
                    % M
                )
            else:
                R = (
                    nCr(n - 2, prime_count_arr[n], M, mod_invs_arr)
                    * pow_mod(K - 1, n - 2 - prime_count_arr[n], M)
                    % M
                )
            C = (K * C - R) % M
        
        ans = (ans + C) % M
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
