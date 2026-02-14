/*
 * Project Euler Problem 396: Weak Goodstein Sequence
 *
 * Compute last 9 digits of sum(G(n)) for 1 <= n <= 15.
 * Uses modular arithmetic with CRT for the iterated tower function.
 */
#include <stdio.h>
#include <string.h>

#define MOD 1000000000LL

static long long power_mod(long long base, long long exp, long long mod) {
    if (mod == 1) return 0;
    long long result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (__int128)result * base % mod;
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static long long mod_inverse(long long a, long long m) {
    return power_mod(a, m - 2, m); /* Works if m is prime */
}

/* For general modular inverse using extended GCD */
static long long ext_gcd_inv(long long a, long long m) {
    long long g = 1, x = 0, y = 1;
    long long a0 = a, m0 = m;
    /* Extended Euclidean */
    long long old_r = a0, r = m0;
    long long old_s = 1, s = 0;
    while (r != 0) {
        long long q = old_r / r;
        long long tmp = r; r = old_r - q * r; old_r = tmp;
        tmp = s; s = old_s - q * s; old_s = tmp;
    }
    long long result = old_s % m;
    if (result < 0) result += m;
    return result;
}

/* 5^k values */
static long long pow5[10];

/* solve_mod: compute iter_B mod M where
 * iter_0 = (B+1)*2^{B+1}, iter_{k+1} = iter_k * 2^{iter_k} */
static long long solve_mod(long long B) {
    long long M9 = pow5[9];

    /* Track iter mod 5^k for k=1..9 */
    long long five_mods[10];
    for (int k = 1; k <= 9; k++) {
        long long Mk = pow5[k];
        five_mods[k] = ((B + 1) % Mk * power_mod(2, B + 1, Mk)) % Mk;
    }

    /* Iterate B times */
    for (long long iter = 0; iter < B; iter++) {
        long long new_five_mods[10];
        for (int k = 1; k <= 9; k++) {
            long long Mk = pow5[k];
            long long iter_mod_Mk = five_mods[k];

            long long exp_mod;
            if (k == 1) {
                exp_mod = 0;
            } else {
                long long M_km1 = pow5[k - 1];
                long long r = five_mods[k - 1];
                long long inv4 = ext_gcd_inv(4, M_km1);
                long long t = ((__int128)r * inv4) % M_km1;
                exp_mod = 4 * t;
            }

            long long pow2 = power_mod(2, exp_mod, Mk);
            new_five_mods[k] = ((__int128)iter_mod_Mk * pow2) % Mk;
        }
        memcpy(five_mods, new_five_mods, sizeof(five_mods));
    }

    /* CRT: combine mod 512 (=0) and mod 5^9 */
    long long mod_5_9 = five_mods[9];
    long long inv512 = ext_gcd_inv(512, M9);
    long long k_val = ((__int128)mod_5_9 * inv512) % M9;
    return (512 * k_val) % MOD;
}

int main(void) {
    /* Precompute powers of 5 */
    pow5[0] = 1;
    for (int i = 1; i <= 9; i++) pow5[i] = pow5[i - 1] * 5;

    long long total = 0;

    /* n=1..3: direct values */
    long long direct[] = {1, 3, 5};
    for (int i = 0; i < 3; i++) {
        total = (total + direct[i]) % MOD;
    }

    /* n=4..7: 3-digit binary. G(n) = c0 * 2^c0 - 3 */
    for (int n = 4; n <= 7; n++) {
        int d1 = (n >> 1) & 1;
        int d0 = n & 1;
        long long F_val = ((1LL << d1) - 1) * (3 + d0) + d0;
        long long c0 = 3 + F_val;
        long long c1_mod = (c0 % MOD * power_mod(2, c0, MOD)) % MOD;
        long long gn = (c1_mod - 3 + MOD) % MOD;
        total = (total + gn) % MOD;
    }

    /* n=8..15: 4-digit binary */
    for (int n = 8; n <= 15; n++) {
        int d2 = (n >> 2) & 1;
        int d1 = (n >> 1) & 1;
        int d0 = n & 1;

        long long sub3;
        if (d2 == 0) {
            if (d1 == 0) {
                sub3 = d0;
            } else {
                sub3 = ((1LL << d1) - 1) * (3 + d0) + d0;
            }
        } else {
            long long F_val = ((1LL << d1) - 1) * (3 + d0) + d0;
            long long c0 = 3 + F_val;
            /* c1 = c0 * 2^c0 - this can be huge, but sub3 = c1 - 3 */
            /* For the iteration, B = 2 + sub3 = 2 + c0 * 2^c0 - 3 = c0 * 2^c0 - 1 */
            /* But sub3 = c1 - 3 = c0 * 2^c0 - 3, B = 2 + sub3 = c0 * 2^c0 - 1 */
            /* These are small enough: c0 is at most 3 + ((2-1)*(3+1)+1) = 3+5 = 8 */
            /* So c0*2^c0 is at most 8*256 = 2048 */
            sub3 = c0 * (1LL << c0) - 3;
        }

        long long B = 2 + sub3;
        long long iter_B_mod = solve_mod(B);
        long long gn = (iter_B_mod - 3 + MOD) % MOD;
        total = (total + gn) % MOD;
    }

    printf("%lld\n", total);
    return 0;
}
