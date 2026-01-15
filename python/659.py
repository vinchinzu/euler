"""Project Euler Problem 659: Largest prime.

Let P(k) be the largest prime that divides two successive terms in the sequence
n² + k². Find sum_{k=1}^N P(k).

If both n² + k² and (n+1)² + k² are divisible by p, then n² ≡ (n+1)² =>
n ≡ (p-1)/2 => ((p-1)/2)² ≡ -k² => (2k)² + 1 ≡ 0 (mod p). This means that
P(k) is the largest prime factor of (2k)² + 1. We can compute the P(k)s by
maintaining a table of (2k)² + 1 over all k, then repeatedly dividing out
increasing primes p.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


def imod(n: int, mod: int) -> int:
    """Integer modulo (non-negative result)."""
    return ((n % mod) + mod) % mod


def sqrt_mod(a: int, p: int) -> int:
    """Compute square root of a modulo prime p using Tonelli-Shanks."""
    if pow(a, (p - 1) // 2, p) != 1:
        return -1  # No square root exists

    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)

    # Find Q and S such that p-1 = Q * 2^S
    Q = p - 1
    S = 0
    while Q % 2 == 0:
        Q //= 2
        S += 1

    # Find a quadratic non-residue
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1

    M = S
    c = pow(z, Q, p)
    t = pow(a, Q, p)
    R = pow(a, (Q + 1) // 2, p)

    while t != 1:
        i = 1
        while pow(t, 2**i, p) != 1:
            i += 1
        b = pow(c, 2 ** (M - i - 1), p)
        M = i
        c = (b * b) % p
        t = (t * c) % p
        R = (R * b) % p

    return R


def primes_mod(limit: int, remainder: int, mod: int) -> list[int]:
    """Get primes up to limit that are ≡ remainder (mod mod)."""
    primes = list(primerange(2, limit + 1))
    return [p for p in primes if p % mod == remainder]


def solve() -> int:
    """Solve Problem 659."""
    N = 10**7
    M = 10**18

    P = [0] * (N + 1)
    for k in range(1, N + 1):
        P[k] = (2 * k) ** 2 + 1

    primes = primes_mod(2 * N, 1, 4)
    for p in primes:
        sqrt_val = sqrt_mod(-1, p)
        if sqrt_val == -1:
            continue
        for signed_sqrt in [sqrt_val, p - sqrt_val]:
            k_start = imod(signed_sqrt * (p + 1) // 2, p)
            for k in range(k_start, N + 1, p):
                while P[k] % p == 0 and P[k] > p:
                    P[k] //= p

    ans = 0
    for k in range(1, N + 1):
        ans = (ans + P[k]) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
