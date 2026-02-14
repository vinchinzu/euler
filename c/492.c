/* Project Euler 492 - Exploding sequence
 * Translated from python/492.py
 *
 * a_1 = 1, a_{n+1} = 6a_n^2 + 10a_n + 3
 * Find sum of a_N (mod p) for all primes X <= p <= X+Y.
 * Uses 2x2 matrix exponentiation + segmented sieve.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>

typedef unsigned long long ull;
typedef __int128 u128;

/* 2x2 matrix multiply mod p */
typedef struct { ull m[4]; } Mat2;

Mat2 mat_mul_2x2(Mat2 a, Mat2 b, ull p) {
    Mat2 c;
    c.m[0] = ((u128)a.m[0] * b.m[0] + (u128)a.m[1] * b.m[2]) % p;
    c.m[1] = ((u128)a.m[0] * b.m[1] + (u128)a.m[1] * b.m[3]) % p;
    c.m[2] = ((u128)a.m[2] * b.m[0] + (u128)a.m[3] * b.m[2]) % p;
    c.m[3] = ((u128)a.m[2] * b.m[1] + (u128)a.m[3] * b.m[3]) % p;
    return c;
}

Mat2 mat_pow_2x2(Mat2 m, ull exp, ull p) {
    Mat2 result = {{1, 0, 0, 1}};
    Mat2 base;
    base.m[0] = m.m[0] % p;
    base.m[1] = ((m.m[1] % p) + p) % p;
    base.m[2] = ((m.m[2] % p) + p) % p;
    base.m[3] = m.m[3] % p;
    while (exp > 0) {
        if (exp & 1) result = mat_mul_2x2(result, base, p);
        base = mat_mul_2x2(base, base, p);
        exp >>= 1;
    }
    return result;
}

ull powmod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    ull N_val = 1000000000000000ULL;  /* 10^15 */
    ull X = 1000000000ULL;            /* 10^9 */
    ull Y = 10000000ULL;              /* 10^7 */

    /* Sieve small primes up to sqrt(X+Y) */
    int sqrt_limit = (int)sqrt((double)(X + Y)) + 1;
    char *small_sieve = calloc(sqrt_limit + 1, 1);
    for (int i = 2; i <= sqrt_limit; i++) small_sieve[i] = 1;
    for (int i = 2; (long long)i * i <= sqrt_limit; i++) {
        if (small_sieve[i]) {
            for (int j = i * i; j <= sqrt_limit; j += i)
                small_sieve[j] = 0;
        }
    }
    int *small_primes = malloc(sqrt_limit * sizeof(int));
    int num_small = 0;
    for (int i = 2; i <= sqrt_limit; i++) {
        if (small_sieve[i]) small_primes[num_small++] = i;
    }
    free(small_sieve);

    /* Segmented sieve for [X, X+Y] */
    char *is_prime = malloc(Y + 1);
    memset(is_prime, 1, Y + 1);
    for (int i = 0; i < num_small; i++) {
        ull p = small_primes[i];
        ull start = ((X + p - 1) / p) * p - X;  /* first multiple of p >= X, as offset */
        if ((ull)p * p > X) {
            start = (ull)p * p - X;
        } else {
            start = (p - X % p) % p;
        }
        for (ull j = start; j <= Y; j += p) {
            is_prime[j] = 0;
        }
    }

    /* A = [[0, 1], [-1, 11]] */
    ull ans = 0;
    Mat2 identity = {{1, 0, 0, 1}};

    for (ull i = 0; i <= Y; i++) {
        if (!is_prime[i]) continue;
        ull p = X + i;

        /* A mod p: entries are [0, 1, p-1, 11] */
        Mat2 A;
        A.m[0] = 0; A.m[1] = 1; A.m[2] = p - 1; A.m[3] = 11 % p;

        /* Check if period is p-1 or p+1 */
        Mat2 test = mat_pow_2x2(A, p - 1, p);
        ull period;
        if (test.m[0] == 1 && test.m[1] == 0 && test.m[2] == 0 && test.m[3] == 1)
            period = p - 1;
        else
            period = p + 1;

        ull exp_val = powmod(2, N_val - 1, period);
        Mat2 mat = mat_pow_2x2(A, exp_val, p);

        /* x_n = 2 * mat[0] + 11 * mat[1] */
        ull x_n = ((u128)2 * mat.m[0] + (u128)11 * mat.m[1]) % p;
        /* a_n = (x_n - 5) / 6 mod p */
        ull a_n = ((x_n + p - 5 % p) % p * powmod(6, p - 2, p)) % p;
        ans += a_n;
    }

    printf("%llu\n", ans);

    free(small_primes);
    free(is_prime);
    return 0;
}
