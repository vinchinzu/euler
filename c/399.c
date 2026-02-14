/*
 * Project Euler Problem 399: Square-free Fibonacci
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

static unsigned char *prime_sieve;

static inline int is_prime(long long n) {
    if (n < 2) return 0;
    return (prime_sieve[n >> 3] >> (n & 7)) & 1;
}

static void sieve_primes(long long limit) {
    long long bytes = (limit >> 3) + 1;
    prime_sieve = (unsigned char *)malloc(bytes);
    if (!prime_sieve) { fprintf(stderr, "alloc fail prime_sieve\n"); exit(1); }
    memset(prime_sieve, 0xFF, bytes);
    prime_sieve[0] &= ~((1 << 0) | (1 << 1));

    for (long long i = 2; i * i <= limit; i++) {
        if (is_prime(i)) {
            for (long long j = i * i; j <= limit; j += i) {
                prime_sieve[j >> 3] &= ~(1 << (j & 7));
            }
        }
    }
}

int main(void) {
    long long N = 100000000LL;
    long long L = 2 * N;
    long long M = 10000000000000000LL;

    unsigned char *sqf = (unsigned char *)malloc(L);
    if (!sqf) { fprintf(stderr, "alloc fail sqf\n"); return 1; }
    memset(sqf, 1, L);

    sieve_primes(L);

    for (long long p = 2; p < L; p++) {
        if (!is_prime(p)) continue;

        long long first_index = 1;
        long long a = 1 % p, b = 1 % p;

        while (p * first_index < L) {
            if (a == 0) {
                long long step = p * first_index;
                for (long long i = step; i < L; i += step) {
                    sqf[i] = 0;
                }
                break;
            }
            long long new_b = (a + b) % p;
            a = b;
            b = new_b;
            first_index++;
        }
    }

    free(prime_sieve);

    long long index = -1;
    long long count = 0;
    while (count < N) {
        index++;
        if (sqf[index]) {
            count++;
        }
    }
    free(sqf);

    long long fa = 1, fb = 1;
    for (long long i = 0; i < index; i++) {
        long long new_fb = (fa + fb) % M;
        fa = fb;
        fb = new_fb;
    }

    long double phi = (1.0L + sqrtl(5.0L)) / 2.0L;
    long double log_value = (long double)(index + 1) * log10l(phi) - log10l(sqrtl(5.0L));
    long long exponent = (long long)log_value;
    long double mantissa = powl(10.0L, log_value - (long double)exponent);

    printf("%lld,%.1Lfe%lld\n", fa, mantissa, exponent);

    return 0;
}
