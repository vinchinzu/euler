/*
 * Project Euler 146 - Investigating a Prime Pattern
 *
 * Sum of all n <= 150,000,000 where n^2+{1,3,7,9,13,27} are all prime
 * and n^2+{5,11,15,17,19,21,23,25} are all NOT prime.
 *
 * Uses modular pre-sieve and Miller-Rabin primality testing.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef unsigned long long ull;
typedef __int128 u128;

static ull mod_mul(ull a, ull b, ull m) {
    return (u128)a * b % m;
}

static ull mod_pow(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = mod_mul(result, base, mod);
        base = mod_mul(base, base, mod);
        exp >>= 1;
    }
    return result;
}

static bool miller_rabin(ull n, ull a) {
    if (n % a == 0) return n == a;
    ull d = n - 1;
    int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }
    ull x = mod_pow(a, d, n);
    if (x == 1 || x == n - 1) return true;
    for (int i = 0; i < r - 1; i++) {
        x = mod_mul(x, x, n);
        if (x == n - 1) return true;
    }
    return false;
}

static bool is_prime(ull n) {
    if (n < 2) return false;
    if (n < 4) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    /* Deterministic for n < 3.3 * 10^24 with these witnesses */
    static const ull witnesses[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37};
    for (int i = 0; i < 12; i++) {
        if (!miller_rabin(n, witnesses[i])) return false;
    }
    return true;
}

int main(void) {
    static const int must_prime[] = {1, 3, 7, 9, 13, 27};
    static const int must_composite[] = {5, 11, 15, 17, 19, 21, 23, 25};
    const int LIMIT = 150000000;

    /* Precompute allowed residues mod 510510 */
    static const int sieve_primes[] = {2, 3, 5, 7, 11, 13, 17};
    const int MOD = 510510;

    /* For each small prime, compute allowed residues */
    /* allowed_per_p[prime_idx] is a boolean array of size prime */
    bool allowed_2[2], allowed_3[3], allowed_5[5], allowed_7[7];
    bool allowed_11[11], allowed_13[13], allowed_17[17];
    bool *allowed_per_p[] = {allowed_2, allowed_3, allowed_5, allowed_7, allowed_11, allowed_13, allowed_17};

    for (int pi = 0; pi < 7; pi++) {
        int p = sieve_primes[pi];
        bool *arr = allowed_per_p[pi];
        for (int r = 0; r < p; r++) {
            int sq = (r * r) % p;
            bool ok = true;
            for (int ki = 0; ki < 6; ki++) {
                if ((sq + must_prime[ki]) % p == 0) { ok = false; break; }
            }
            arr[r] = ok;
        }
    }

    /* Build allowed residues mod MOD */
    int *allowed = malloc(MOD * sizeof(int));
    int nallowed = 0;
    for (int r = 0; r < MOD; r++) {
        bool ok = true;
        for (int pi = 0; pi < 7; pi++) {
            int p = sieve_primes[pi];
            if (!allowed_per_p[pi][r % p]) { ok = false; break; }
        }
        if (ok) allowed[nallowed++] = r;
    }

    /* Extra prime checks */
    static const int extra_primes[] = {19, 23, 29, 31, 37, 41, 43};
    bool extra_allowed[7][50]; /* max prime is 43 */
    for (int ei = 0; ei < 7; ei++) {
        int p = extra_primes[ei];
        for (int r = 0; r < p; r++) {
            int sq = (r * r) % p;
            bool ok = true;
            for (int ki = 0; ki < 6; ki++) {
                if ((sq + must_prime[ki]) % p == 0) { ok = false; break; }
            }
            extra_allowed[ei][r] = ok;
        }
    }

    long long total = 0;
    for (int ai = 0; ai < nallowed; ai++) {
        long long n = allowed[ai];
        if (n == 0) n = MOD;
        while (n < LIMIT) {
            /* Quick modular checks */
            bool skip = false;
            for (int ei = 0; ei < 7; ei++) {
                int p = extra_primes[ei];
                if (!extra_allowed[ei][(int)(n % p)]) { skip = true; break; }
            }
            if (skip) { n += MOD; continue; }

            ull sq = (ull)n * (ull)n;
            /* Check must-be-prime */
            bool all_prime = true;
            for (int ki = 0; ki < 6; ki++) {
                if (!is_prime(sq + must_prime[ki])) { all_prime = false; break; }
            }
            if (all_prime) {
                /* Check must-be-composite */
                bool all_comp = true;
                for (int ki = 0; ki < 8; ki++) {
                    if (is_prime(sq + must_composite[ki])) { all_comp = false; break; }
                }
                if (all_comp) total += n;
            }
            n += MOD;
        }
    }

    free(allowed);
    printf("%lld\n", total);
    return 0;
}
