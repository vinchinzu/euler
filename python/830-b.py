#!/usr/bin/env python3
# Computes S(10^18) mod 83^3 * 89^3 * 97^3

import math

def binom_mod_prime_power_n_small_k(n: int, j: int, p: int, a: int = 3) -> int:
    """
    binom(n, j) modulo p^a, with n huge and j small (<= ~3p).
    Uses p-adic factor tracking: split numerator and j! into p^e * unit.
    """
    m = p**a
    # Numerator product U * p^E, where U is unit mod p^a (no p factors), E = total v_p
    E = 0
    U = 1 % m
    for t in range(j):
        x = n - t
        v = 0
        # extract p-adic valuation up to a
        while v < a and x % p == 0:
            x //= p
            v += 1
        E += v
        U = (U * (x % m)) % m

    # Denominator j! = u * p^d (u is unit)
    d = 0
    u = 1 % m
    for r in range(1, j + 1):
        y = r
        while y % p == 0:
            y //= p
            d += 1
        u = (u * (y % m)) % m

    inv_u = pow(u, -1, m)  # u is coprime to p, so invertible mod p^a
    res = U * inv_u % m
    res = (res * pow(p, E - d, m)) % m
    return res

def S_mod_p3(n: int, p: int, a: int = 3) -> int:
    """
    S(n) modulo p^a (with a=3 here).
    Truncates j at 3p-1 since v_p(j!) >= 3 for j >= 3p, so those terms vanish mod p^3.
    Uses: term_j = (sum_{i=0}^j (-1)^{j-i} C(j,i) i^n) * C(n,j) * 2^{n-j}
    """
    m = p**a
    j_max = min(n, 3 * p - 1)
    pow2n = pow(2, n, m)
    total = 0
    for j in range(0, j_max + 1):
        # N_j = j! * S(n,j) = sum_{i=0}^j (-1)^{j-i} C(j,i) i^n
        N = 0
        for i in range(0, j + 1):
            term = math.comb(j, i) * pow(i, n, m)
            if (j - i) & 1:
                N = (N - term) % m
            else:
                N = (N + term) % m

        Cnj = binom_mod_prime_power_n_small_k(n, j, p, a)
        term_j = (N * Cnj) % m
        # multiply by 2^{n-j} but reuse 2^n / 2^j
        term_j = (term_j * (pow2n * pow(pow(2, j, m), -1, m) % m)) % m
        total = (total + term_j) % m
    return total

def crt(remainders, moduli):
    """Chinese Remainder Theorem for pairwise-coprime moduli."""
    M = 1
    for mi in moduli: M *= mi
    x = 0
    for ai, mi in zip(remainders, moduli):
        Mi = M // mi
        inv = pow(Mi, -1, mi)
        x = (x + ai * Mi * inv) % M
    return x, M

def main():
    n = 10**18
    primes = [83, 89, 97]
    mods = [p**3 for p in primes]
    residues = [S_mod_p3(n, p, 3) for p in primes]
    x, M = crt(residues, mods)

    # Sanity check from prompt:
    assert sum(math.comb(10, k) * (k**10) for k in range(11)) == 142469423360

    print("Residues mod p^3:")
    for p, r in zip(primes, residues):
        print(f"  mod {p**3}: {r}")
    print(f"\nModulus (M) = {M}")
    print(f"S(10^18) mod M = {x}")

if __name__ == "__main__":
    main()

