"""Project Euler Problem 536: Modulo Power Identity.

Find the sum of all integers m ≤ N such that a^{m+4} ≡ a (mod m) for all a.

First, note that if p²|m, then p^{m+4} ≡ 0 (mod p²) ≠ p, so m must be
square-free.

To satisfy the property, we must have λ(m) | m+3, where λ(m) is the
Carmichael function. For m square-free, λ(m) = LCM_i (p_i - 1). So we
recursively build up all square-free m while maintaining λ(m), and check the
condition.
"""

from __future__ import annotations

from math import gcd, isqrt
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


def pre_smallest_prime_factor(limit: int) -> List[int]:
    """Precompute smallest prime factor."""
    spf = list(range(limit + 1))
    for i in range(2, isqrt(limit) + 1):
        if spf[i] == i:
            for j in range(i * i, limit + 1, i):
                if spf[j] == j:
                    spf[j] = i
    return spf


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    # Extended Euclidean algorithm
    if m == 1:
        return 0
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


def imod(a: int, m: int) -> int:
    """Integer modulo."""
    return ((a % m) + m) % m


def good(m: int, r: int, max_p: int, spf: List[int]) -> bool:
    """Check if m*r satisfies conditions."""
    while r > 1:
        p = spf[r] if r < len(spf) else r
        if (m + 3) % (p - 1) != 0:
            return False
        if p >= max_p:
            return False
        r //= p
        if r % p == 0:
            return False
    return True


def solve() -> int:
    """Solve Problem 536."""
    N = 10**12
    L = 10**8

    spf = pre_smallest_prime_factor(L)
    sqrt_n = isqrt(N)
    primes = sieve_primes(sqrt_n)

    ans = 0

    def helper(max_index: int, m: int, carmichael: int) -> None:
        """Recursive helper function."""
        nonlocal ans
        g = gcd(m, carmichael)
        if 3 % g != 0:
            return
        if (m + 3) % carmichael == 0:
            ans += m

        # Optimization: use congruence approach for large m
        if N // m < len(spf) and N // m // carmichael < pow(2, max_index):
            mod = carmichael // g
            if mod > 0:
                r_start = imod((-3 // g) * mod_inv(m // g, mod), mod)
                r = r_start
                while m * r <= N:
                    if r > 1 and good(m * r, r, primes[max_index] if max_index < len(primes) else N, spf):
                        ans += m * r
                    r += mod
            return

        # Recursive case: try adding primes
        for index in range(max_index):
            p = primes[index]
            if m * p > N:
                break
            helper(index, m * p, lcm(carmichael, p - 1))

    helper(len(primes), 1, 1)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
