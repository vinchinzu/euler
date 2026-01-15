"""Project Euler Problem 229: Four Representations using Squares.

Find the number of numbers not exceeding N that can be expressed as
a²+k*b² for k=1,2,3,7 and positive integers a,b.
"""

from __future__ import annotations

from math import isqrt
from typing import List, Set


def build_spf(limit: int) -> List[int]:
    """Build smallest prime factor array up to limit."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def primes_mod(limit: int, k: int, mod: int) -> List[int]:
    """Return primes p <= limit with p ≡ k (mod mod)."""
    primes_list = sieve(limit)
    return [p for p in primes_list if p % mod == k]


def imod_inv(n: int, mod: int) -> int:
    """Compute modular inverse of n mod mod."""
    # Extended Euclidean algorithm
    def ext_gcd(a: int, b: int) -> tuple[int, int, int]:
        if b == 0:
            return (a, 1, 0)
        g, x1, y1 = ext_gcd(b, a % b)
        return (g, y1, x1 - (a // b) * y1)

    g, x, _ = ext_gcd(n, mod)
    if g != 1:
        return 0
    return x % mod


def solve() -> int:
    """Solve Problem 229."""
    N = 2 * (10**9)
    L = isqrt(N)
    spf = build_spf(L)

    # Sieve for primes ≡ 1 (mod 24)
    sieve_size = N // 24 + 1
    sieve_arr = [True] * sieve_size
    sieve_arr[0] = False

    primes_list = sieve(L)
    for p in primes_list:
        if p >= 5:
            inv = imod_inv(24, p)
            for i in range(p - inv, sieve_size, p):
                sieve_arr[i] = False

    # Collect all primes
    all_primes: List[int] = primes_list.copy()
    for i in range((L - 1) // 24 + 1, sieve_size):
        if sieve_arr[i]:
            all_primes.append(24 * i + 1)

    ans = [0]

    def helper(
        index: int,
        prod: int,
        num_twos: int,
        has_1mod4: bool,
        has_1or3mod8: bool,
        has_1mod6: bool,
        has_1or9or11mod14: bool,
    ) -> None:
        """Recursive helper function."""
        if (
            has_1mod4
            and has_1or3mod8
            and (num_twos >= 2 or has_1mod6)
            and (num_twos >= 3 or has_1or9or11mod14)
        ):
            ans[0] += 1

        i = index
        while i < len(all_primes) and all_primes[i] != 0:
            p = all_primes[i]
            if prod * p > N:
                break

            e = 1
            new_prod = prod * p
            while new_prod <= N:
                if (
                    p % 168 == 1
                    or p % 168 == 25
                    or p % 168 == 121
                    or e % 2 == 0
                ):
                    helper(
                        i + 1,
                        new_prod,
                        num_twos + (e if p == 2 else 0),
                        has_1mod4 or p % 4 == 1,
                        has_1or3mod8 or p % 8 == 1 or p % 8 == 3,
                        has_1mod6 or p % 6 == 1,
                        has_1or9or11mod14
                        or p % 14 == 1
                        or p % 14 == 9
                        or p % 14 == 11,
                    )
                e += 1
                new_prod *= p
            i += 1

    helper(0, 1, 0, False, False, False, False)
    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
