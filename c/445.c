/*
 * Project Euler 445 - Retractions A
 *
 * Extracted from embedded C in python/445.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define NN 10000000
#define MOD 1000000007LL

static int spf[NN + 1];     /* smallest prime factor */
static int exps[NN + 1];    /* exponents of each prime in current C(N,k) */

static long long power_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static long long mod_inv(long long a, long long mod) {
    return power_mod(a, mod - 2, mod);
}

int main(void) {
    /* Sieve smallest prime factor */
    for (int i = 0; i <= NN; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= NN; i++)
        if (spf[i] == i)
            for (int j = i * i; j <= NN; j += i)
                if (spf[j] == j)
                    spf[j] = i;

    /* Initialize exponent array */
    for (int i = 0; i <= NN; i++) exps[i] = 0;

    long long res_prod_pe = 1;   /* product of p^e mod MOD */
    long long res_prod_1pe = 1;  /* product of (1 + p^e) mod MOD */
    long long ans = 0;

    for (int k = 1; k <= NN / 2; k++) {
        /* Multiply by (N + 1 - k): factor it and update exponents */
        int num = NN + 1 - k;
        while (num > 1) {
            int p = spf[num];
            int e = 0;
            while (num % p == 0) { num /= p; e++; }

            long long old_pow = power_mod(p, exps[p], MOD);
            exps[p] += e;
            long long new_pow = power_mod(p, exps[p], MOD);

            res_prod_pe = res_prod_pe % MOD * mod_inv(old_pow, MOD) % MOD * new_pow % MOD;
            long long old_term = (old_pow == 1) ? 1 : (1 + old_pow) % MOD;
            long long new_term = (1 + new_pow) % MOD;
            res_prod_1pe = res_prod_1pe % MOD * mod_inv(old_term, MOD) % MOD * new_term % MOD;
        }

        /* Divide by k: factor it and update exponents */
        int den = k;
        while (den > 1) {
            int p = spf[den];
            int e = 0;
            while (den % p == 0) { den /= p; e++; }

            long long old_pow = power_mod(p, exps[p], MOD);
            exps[p] -= e;
            long long new_pow = power_mod(p, exps[p], MOD);

            res_prod_pe = res_prod_pe % MOD * mod_inv(old_pow, MOD) % MOD * new_pow % MOD;
            long long old_term = (1 + old_pow) % MOD;
            long long new_term = (new_pow == 1) ? 1 : (1 + new_pow) % MOD;
            res_prod_1pe = res_prod_1pe % MOD * mod_inv(old_term, MOD) % MOD * new_term % MOD;
        }

        /* R(n) = res_prod_1pe - res_prod_pe */
        long long R_val = (res_prod_1pe - res_prod_pe + MOD) % MOD;

        /* Double all terms except the middle one */
        if (k == NN / 2)
            ans = (ans + R_val) % MOD;
        else
            ans = (ans + 2 * R_val) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
