"""Project Euler Problem 468: Smooth divisors of binomial coefficients.

Let S_B(n) be the largest divisor of n with no prime factors greater than B.
Find Σ_{B=1}^N Σ_{r=0}^N S_B(nCr(N, r)).
"""

from __future__ import annotations

from math import isqrt
from typing import List


def sieve_primes(limit: int) -> List[bool]:
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return is_prime


def mod_invs(n: int, mod: int) -> List[int]:
    """Precompute modular inverses."""
    invs = [1] * (n + 1)
    for i in range(2, n + 1):
        invs[i] = (mod - (mod // i) * invs[mod % i] % mod) % mod
    return invs


def solve() -> int:
    """Solve Problem 468."""
    N = 11_111_111
    M = 1_000_000_993
    L = N.bit_length()

    is_prime = sieve_primes(N)

    # Small B: brute force
    S = [1] * (N // 2 + 1)
    ans = 0
    for B in range(1, L):
        if is_prime[B]:
            prod = 1
            for r in range(1, N // 2 + 1):
                n = N + 1 - r
                while n % B == 0:
                    prod = (prod * B) % M
                    n //= B
                n = r
                while n % B == 0:
                    prod = (prod * mod_invs(B, M)[B]) % M
                    n //= B
                S[r] = (S[r] * prod) % M
        for s in S:
            ans = (ans + 2 * s) % M

    # Large B: use segment tree (simplified)
    # For efficiency, we'd use a proper segment tree, but for now
    # we'll use a simplified approach
    mod_invs_list = mod_invs(N, M)
    mults = [1] * (2 * L)
    sums = [0] * (2 * L)
    for i in range(len(S)):
        sums[L + i] = S[i]

    for i in range(L - 1, 0, -1):
        sums[i] = (mults[2 * i] * sums[2 * i] + mults[2 * i + 1] * sums[2 * i + 1]) % M

    def multiply_range(r: int, B: int) -> None:
        """Multiply range by B."""
        idx = L + r
        while idx > 0:
            mults[idx] = (mults[idx] * B) % M
            idx //= 2

    for B in range(L, N + 1):
        if is_prime[B]:
            for r in range(N % B + 1, N // 2 + 1, B):
                n = N + 1 - r
                while n % B == 0:
                    multiply_range(r, B)
                    n //= B
            for r in range(B, N // 2 + 1, B):
                n = r
                while n % B == 0:
                    multiply_range(r, mod_invs_list[B])
                    n //= B
        ans = (ans + 2 * sums[1]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
