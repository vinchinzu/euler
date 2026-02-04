"""Project Euler Problem 489: Common factors between two sequences.

Ported from Java solution. Uses prime factorization + CRT approach.

For a given (a, b), the largest GCD of (n^3+b, (n+a)^3+b) only involves
prime factors of 6a and a^6 + 27*b^2. For each such prime p, we find the
highest power p^e that divides the GCD for some n, and then use CRT to
combine solutions across all primes.
"""

from math import gcd, isqrt
from itertools import product as cartprod
from functools import reduce


def prime_factors(n):
    """Return dict of prime factors {p: exponent}."""
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def mod_inv(a, m):
    """Extended GCD-based modular inverse."""
    if m == 1:
        return 0
    g, x, _ = extended_gcd(a % m, m)
    if g != 1:
        return None
    return x % m


def extended_gcd(a, b):
    if a == 0:
        return b, 0, 1
    g, x, y = extended_gcd(b % a, a)
    return g, y - (b // a) * x, x


def crt(remainders, moduli):
    """Chinese Remainder Theorem. Returns smallest nonneg solution."""
    if not remainders:
        return 0
    r, m = remainders[0], moduli[0]
    for i in range(1, len(remainders)):
        r2, m2 = remainders[i], moduli[i]
        g = gcd(m, m2)
        if (r2 - r) % g != 0:
            return None  # No solution
        lcm = m // g * m2
        inv = mod_inv(m // g, m2 // g)
        r = (r + m * ((r2 - r) // g * inv % (m2 // g))) % lcm
        m = lcm
    return r % m


def cb(n, m=None):
    """Cube of n, optionally mod m."""
    if m is not None:
        return pow(n, 3, m)
    return n * n * n


def solve():
    M = 18
    N = 1900

    ans = 0
    for a in range(1, M + 1):
        expr_val = a**6 + 27 * b * b if False else 0  # computed per b below
        for b in range(1, N + 1):
            ans += G(a, b)
    return ans


def G(a, b):
    # Get all relevant primes: prime factors of 6a and a^6 + 27*b^2
    pf1 = prime_factors(6 * a)
    pf2 = prime_factors(a**6 + 27 * b * b)
    all_primes = set(pf1.keys()) | set(pf2.keys())

    all_ns = []  # list of lists of possible n values for each prime power
    ms = []      # corresponding moduli

    for p in all_primes:
        prev_ns = []
        prev_m = 1
        m = 1
        while True:
            m = prev_m * p
            ns = []
            if gcd(p, 6 * a) == 1:
                # n = -(2a^2)^{-1} * (3b + a^3) mod m
                inv_val = mod_inv((2 * a * a) % m, m)
                if inv_val is not None:
                    n = (-inv_val * (3 * b + a * a * a)) % m
                    if (cb(n, m) + b) % m == 0 and (cb(n + a, m) + b) % m == 0:
                        ns.append(n)
            else:
                for n in range(m):
                    if (cb(n, m) + b) % m == 0 and (cb(n + a, m) + b) % m == 0:
                        ns.append(n)

            if not ns:
                break
            prev_ns = ns
            prev_m = m

        if prev_ns:
            all_ns.append(prev_ns)
            ms.append(prev_m)

    if not all_ns:
        return 0

    # Compute all combinations via cartesian product and CRT
    best = None
    for combo in cartprod(*all_ns):
        val = crt(list(combo), ms)
        if val is not None:
            if best is None or val < best:
                best = val

    return best if best is not None else 0


def main():
    M = 18
    N = 1900
    ans = 0
    for a in range(1, M + 1):
        for b in range(1, N + 1):
            ans += G(a, b)
    print(ans)


if __name__ == "__main__":
    main()
