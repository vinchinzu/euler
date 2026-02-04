#!/usr/bin/env python3
"""
Project Euler 851: Sum of Products / Product of Sums of Tuples

Find R_6(10000!) modulo 10^9+7.

R_k(n) = sum over all pairs (u,v) of k-tuples of positive integers
         with <u,v> = sum u_i*v_i = n of prod(u_i + v_i).

Key insight: R_1(n) = 2*sigma(n), and R_k = convolution of R_1 values.
For k=6, R_6(n)/64 is expressible as a linear combination of
n^l * sigma_k(n) (quasimodular forms) plus the Ramanujan tau function.

Algorithm:
1. Compute tau(n) for n=1..N via prod(1-q^n)^24 * q
2. Compute R_6(n) for small n via direct convolution
3. Solve linear system to get coefficients
4. Evaluate at n = 10000! using multiplicativity of tau and sigma_k
"""

import numpy as np

MOD = 10**9 + 7


def mod_pow(base, exp, mod):
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = result * base % mod
        base = base * base % mod
        exp >>= 1
    return result


def mod_inv(n, mod):
    return mod_pow(n % mod, mod - 2, mod)


def sieve_primes(limit):
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def compute_sigma1(limit):
    sig = [0] * (limit + 1)
    for d in range(1, limit + 1):
        for multiple in range(d, limit + 1, d):
            sig[multiple] += d
    return sig


def compute_sigma_k_mod(limit, k):
    sig = [0] * (limit + 1)
    for d in range(1, limit + 1):
        dk = mod_pow(d, k, MOD)
        for multiple in range(d, limit + 1, d):
            sig[multiple] = (sig[multiple] + dk) % MOD
    return sig


def poly_mul_np(a, b, limit, mod):
    """Truncated polynomial multiplication using numpy, handling overflow."""
    L = limit + 1
    result = np.zeros(L, dtype=np.int64)
    for i in range(L):
        if a[i] == 0:
            continue
        ai = int(a[i])
        max_j = L - i
        result[i:i + max_j] = (result[i:i + max_j] + ai * b[:max_j]) % mod
    return result


def poly_pow_np(poly, n, limit, mod):
    """Compute poly^n truncated to degree limit, mod mod."""
    if n == 0:
        r = np.zeros(limit + 1, dtype=np.int64)
        r[0] = 1
        return r
    result = None
    base = poly[:limit + 1].copy()
    while n > 0:
        if n & 1:
            if result is None:
                result = base.copy()
            else:
                result = poly_mul_np(result, base, limit, mod)
        base = poly_mul_np(base, base, limit, mod)
        n >>= 1
    return result


def compute_tau(limit):
    """Compute Ramanujan tau(n) for n=1..limit mod MOD.

    tau(n) = coeff of q^n in q * prod(1-q^m)^24.
    """
    # Compute prod(1-q^n) mod MOD up to degree limit using numpy
    a = np.zeros(limit + 1, dtype=np.int64)
    a[0] = 1
    for n in range(1, limit + 1):
        a[n:] = (a[n:] - a[:limit + 1 - n]) % MOD

    # Raise to 24th power
    f24 = poly_pow_np(a, 24, limit, MOD)

    # tau(n) = f24[n-1] (shift by q)
    tau = [0] * (limit + 1)
    for n in range(1, limit + 1):
        tau[n] = int(f24[n - 1])
    return tau


def compute_R6_small(limit):
    """Compute R_6(n) for n = 1..limit mod MOD."""
    sig1 = compute_sigma1(limit)
    R1 = [0] * (limit + 1)
    for n in range(1, limit + 1):
        R1[n] = (2 * sig1[n]) % MOD

    R = R1[:]
    for _ in range(5):
        new_R = [0] * (limit + 1)
        for i in range(1, limit + 1):
            if R[i] == 0:
                continue
            ri = R[i]
            for j in range(1, limit + 1 - i):
                if R1[j]:
                    new_R[i + j] = (new_R[i + j] + ri * R1[j]) % MOD
        R = new_R
    return R


def get_basis_terms(K=6):
    """Basis terms for quasimodular expansion: (k, l) pairs.

    sigma_k(n) * n^l for odd k from 1 to 2K-1, l from 0 while k+2l < 2K.
    """
    terms = []
    for k in range(1, 2 * K, 2):
        l = 0
        while k + 2 * l < 2 * K:
            terms.append((k, l))
            l += 1
    return terms


def solve_linear_system(matrix, vector, mod):
    n = len(vector)
    aug = [row[:] + [vector[i] % mod] for i, row in enumerate(matrix)]
    for col in range(n):
        pivot = -1
        for row in range(col, n):
            if aug[row][col] % mod != 0:
                pivot = row
                break
        if pivot == -1:
            raise ValueError(f"Singular matrix at column {col}")
        if pivot != col:
            aug[col], aug[pivot] = aug[pivot], aug[col]
        inv = mod_inv(aug[col][col], mod)
        for j in range(col, n + 1):
            aug[col][j] = aug[col][j] * inv % mod
        for r in range(n):
            if r == col:
                continue
            factor = aug[r][col] % mod
            if factor == 0:
                continue
            for j in range(col, n + 1):
                aug[r][j] = (aug[r][j] - factor * aug[col][j]) % mod
    return [aug[i][n] % mod for i in range(n)]


def factorial_prime_exponents(n):
    primes = sieve_primes(n)
    result = {}
    for p in primes:
        count = 0
        m = n
        while m:
            m //= p
            count += m
        if count:
            result[p] = count
    return result


def main():
    N = 10000
    K = 6
    M = MOD

    basis_terms = get_basis_terms(K)
    num_sigma_terms = len(basis_terms)  # 21
    L = 1 + num_sigma_terms  # 22 unknowns

    # Step 1: Compute tau(n) for n=1..N
    tau = compute_tau(N)

    # Step 2: Compute R_6(n) for n=1..L via direct convolution
    R6 = compute_R6_small(L)

    # Step 3: Compute sigma_k(n) for n=1..L
    needed_ks = sorted(set(k for k, l in basis_terms))
    sigmas = {}
    for k in needed_ks:
        sigmas[k] = compute_sigma_k_mod(L, k)

    # Step 4: Build and solve linear system
    # R_6(n) = X[0]*tau(n) + sum_{i} X[i+1] * sigma_{k_i}(n) * n^{l_i}
    A_mat = []
    b_vec = []
    for i in range(L):
        n = i + 1
        row = [tau[n]]
        for k, l in basis_terms:
            row.append(sigmas[k][n] * mod_pow(n, l, M) % M)
        A_mat.append(row)
        b_vec.append(R6[n])

    X = solve_linear_system(A_mat, b_vec, M)

    # Step 5: Evaluate at n = N! = 10000!
    primes = sieve_primes(N)
    fac_exps = factorial_prime_exponents(N)

    # tau(N!) = prod_p tau(p^{e_p}) via multiplicativity
    # tau(p^{r+1}) = tau(p) * tau(p^r) - p^11 * tau(p^{r-1})
    big_tau = 1
    for p in primes:
        e = fac_exps.get(p, 0)
        if e == 0:
            continue
        taus_p = [0] * (e + 1)
        taus_p[0] = 1
        taus_p[1] = tau[p]
        p11 = mod_pow(p, 11, M)
        for r in range(1, e):
            taus_p[r + 1] = (taus_p[1] * taus_p[r] - p11 * taus_p[r - 1]) % M
        big_tau = big_tau * taus_p[e] % M

    # sigma_k(N!) = prod_p (p^{k(e+1)} - 1) / (p^k - 1)
    big_sigmas = {}
    for k in needed_ks:
        val = 1
        for p in primes:
            e = fac_exps.get(p, 0)
            if e == 0:
                continue
            pk = mod_pow(p, k, M)
            num = (mod_pow(p, k * (e + 1), M) - 1) % M
            den = (pk - 1) % M
            val = val * (num * mod_inv(den, M) % M) % M
        big_sigmas[k] = val

    # N! mod M (note: N >= M means N! mod M = 0, but we still compute)
    factorial_mod = 1
    for x in range(1, N + 1):
        factorial_mod = factorial_mod * x % M

    # Powers of (N! mod M)
    max_l = max(l for k, l in basis_terms)
    fac_powers = [0] * (max_l + 1)
    fac_powers[0] = 1
    for l in range(1, max_l + 1):
        fac_powers[l] = fac_powers[l - 1] * factorial_mod % M

    # Compute final answer
    ans = big_tau * X[0] % M
    for idx, (k, l) in enumerate(basis_terms):
        ans = (ans + big_sigmas[k] * fac_powers[l] % M * X[idx + 1]) % M
    ans = ans % M
    print(ans)


if __name__ == "__main__":
    main()
