#!/usr/bin/env python3
"""
Project Euler 445 - Retractions A

R(n) = product of (1 + p^e) for each prime power p^e dividing n, minus n.
Find sum of R(C(N, k)) for k = 1 to N-1 where N = 10^7, mod 10^9+7.

Key insight:
R(n) = prod(1 + p^e) - prod(p^e) for factorization n = prod(p^e)

C(N, k) = C(N, k-1) * (N+1-k) / k, so we can incrementally update the factorization.
"""

def solve():
    N = 10**7
    MOD = 10**9 + 7

    # Sieve smallest prime factor
    spf = list(range(N + 1))
    for i in range(2, int(N**0.5) + 1):
        if spf[i] == i:
            for j in range(i * i, N + 1, i):
                if spf[j] == j:
                    spf[j] = i

    def prime_factor(n):
        """Return dict of prime: exponent for n."""
        factors = {}
        while n > 1:
            p = spf[n]
            e = 0
            while n % p == 0:
                n //= p
                e += 1
            factors[p] = e
        return factors

    # Precompute modular inverses
    inv = [0] * (N + 1)
    inv[1] = 1
    for i in range(2, N + 1):
        inv[i] = (MOD - MOD // i) * inv[MOD % i] % MOD

    # Track exponents of each prime in current C(N, k)
    exps = [0] * (N + 1)

    # res[0] = product of p^e (mod M)
    # res[1] = product of (1 + p^e) (mod M)
    res_prod_pe = 1
    res_prod_1pe = 1

    ans = 0

    for k in range(1, N // 2 + 1):
        # Multiply by (N + 1 - k): update exponents
        for p, e in prime_factor(N + 1 - k).items():
            old_pow = pow(p, exps[p], MOD)
            exps[p] += e
            new_pow = pow(p, exps[p], MOD)

            res_prod_pe = res_prod_pe * inv[old_pow] % MOD * new_pow % MOD
            old_term = 1 if old_pow == 1 else (1 + old_pow) % MOD
            new_term = (1 + new_pow) % MOD
            res_prod_1pe = res_prod_1pe * inv[old_term] % MOD * new_term % MOD

        # Divide by k: update exponents
        for p, e in prime_factor(k).items():
            old_pow = pow(p, exps[p], MOD)
            exps[p] -= e
            new_pow = pow(p, exps[p], MOD)

            res_prod_pe = res_prod_pe * inv[old_pow] % MOD * new_pow % MOD
            old_term = (1 + old_pow) % MOD
            new_term = 1 if new_pow == 1 else (1 + new_pow) % MOD
            res_prod_1pe = res_prod_1pe * inv[old_term] % MOD * new_term % MOD

        # R(n) = res_prod_1pe - res_prod_pe
        R_val = (res_prod_1pe - res_prod_pe) % MOD

        # Double all terms except the middle one (for k = N/2)
        if k == N // 2:
            ans = (ans + R_val) % MOD
        else:
            ans = (ans + 2 * R_val) % MOD

    return ans

if __name__ == "__main__":
    print(solve())
