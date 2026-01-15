"""Project Euler Problem 421: Prime factors of n^15+1.

Let s(n, m) be the sum of all distinct prime factors of n^15 + 1 that are
no greater than m. Find Σ_{n=1}^N s(n, K).

We compute this sum by iterating over all possible primes p no greater than
m, and for each one computing how many values of n^15 + 1 are divisible by
p.

Given a prime p, there is a generator g with order p-1. If some n satisfies
n^15 + 1 ≡ 0 (mod p), then (-n)^15 ≡ 1 ≡ g^(k*(p-1)), which means -n ≡
g^(k*(p-1)/15). This means that the exponent is a multiple of (p-1) /
GCD(p-1,15), where the multiple k ranges from 1 to GCD(p-1,15) (which we
abbreviate as just GCD from now on).

For a prime p, by trying different values of g, we first find a 15th root
g^((p-1) / GCD) that has GCD distinct exponent multiples, i.e. GCD is the
smallest integer to multiply the exponent by such that the result is 1.
Then, for each of those values r = g^(k*(p-1) / GCD) (where 0 ≤ r ≤ p-1),
n can be p - r, 2p - r, etc. If the largest possible value is N, then this
gives (N + r) / p different values of n.
"""

from __future__ import annotations

from math import gcd

from sympy import primerange


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 421."""
    N = 10**11
    K = 10**8
    R = 15

    ans = 0
    for p in primerange(2, K + 1):
        g_val = gcd(p - 1, R)
        nth_root = 1
        for g in range(1, p):
            if g == 1:
                nth_root = 1
            else:
                nth_root = pow_mod(g, (p - 1) // g_val, p)
            e = 1
            r = nth_root
            while r != 1:
                r = (r * nth_root) % p
                e += 1
            if e == g_val:
                break

        r = 1
        for e in range(1, g_val + 1):
            ans += (N + r) // p
            r = (r * nth_root) % p

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
