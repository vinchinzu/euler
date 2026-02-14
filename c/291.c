/*
 * Project Euler Problem 291: Panaitopol Primes
 *
 * Count primes p < 5*10^15 of the form 2y^2 + 2y + 1.
 * Sieve approach: for each prime q ≡ 1 (mod 4), find roots of f(y) ≡ 0 (mod q)
 * and mark composites.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 5000000000000000LL  /* 5 * 10^15 */
#define LIMIT 50000000        /* f(LIMIT) >= N */

static unsigned char *is_prime_arr;

/* Modular exponentiation */
static long long power_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (__int128)result * base % mod;
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    /* Step 1: Sieve primes up to sqrt(N) ~ 70.7M */
    long long sqrt_N = (long long)sqrt((double)N) + 2;
    char *sieve_small = calloc(sqrt_N + 1, 1);
    if (!sieve_small) return 1;
    /* 1 = prime, 0 = not prime */
    for (long long i = 2; i <= sqrt_N; i++) sieve_small[i] = 1;
    for (long long i = 2; i * i <= sqrt_N; i++) {
        if (sieve_small[i]) {
            for (long long j = i * i; j <= sqrt_N; j += i)
                sieve_small[j] = 0;
        }
    }

    /* Step 2: is_prime_arr[y] means f(y) = 2y^2+2y+1 is prime */
    is_prime_arr = malloc(LIMIT);
    if (!is_prime_arr) return 1;
    memset(is_prime_arr, 1, LIMIT);
    is_prime_arr[0] = 0; /* f(0) = 1 */

    /* Candidate bases for finding sqrt(-1) mod p */
    static const int CANDIDATES[] = {
        2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23,
        26, 29, 31, 33, 34, 37, 38, 41, 42, 43, 46, 47, 51, 53
    };
    int num_candidates = sizeof(CANDIDATES) / sizeof(CANDIDATES[0]);

    for (long long p = 5; p <= sqrt_N; p += 4) {
        if (!sieve_small[p]) continue;

        long long exp = (p - 1) >> 2;
        long long r = 0;

        for (int ci = 0; ci < num_candidates; ci++) {
            long long a = CANDIDATES[ci];
            if (a >= p) continue;
            long long t = power_mod(a, exp, p);
            if ((__int128)t * t % p == p - 1) {
                r = t;
                break;
            }
        }
        if (r == 0) {
            for (long long a = 54; a < p; a++) {
                long long t = power_mod(a, exp, p);
                if ((__int128)t * t % p == p - 1) {
                    r = t;
                    break;
                }
            }
        }
        if (r == 0) continue;

        long long inv2 = (p + 1) >> 1;
        long long y1 = ((__int128)(r - 1 + p) * inv2) % p;
        long long y2 = ((__int128)(-r - 1 + 2 * p) * inv2) % p;

        long long roots[2] = {y1, y2};
        int nroots = (y1 != y2) ? 2 : 1;

        for (int ri = 0; ri < nroots; ri++) {
            long long yr = roots[ri];
            long long start = yr;
            if (start == 0) {
                start = p;
            } else if (start < LIMIT) {
                long long fval = 2LL * start * start + 2LL * start + 1;
                if (fval == p) start += p;
            }
            if (start < LIMIT) {
                for (long long i = start; i < LIMIT; i += p) {
                    is_prime_arr[i] = 0;
                }
            }
        }
    }

    long long count = 0;
    for (long long i = 0; i < LIMIT; i++) {
        if (is_prime_arr[i]) count++;
    }

    printf("%lld\n", count);

    free(sieve_small);
    free(is_prime_arr);
    return 0;
}
