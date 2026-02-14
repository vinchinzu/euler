/*
 * Project Euler Problem 381
 * Sum of S(p) for primes 5 <= p < 10^8.
 * S(p) = sum_{k=1}^{5} (p-k)! mod p = (p-3)*inv(8,p) mod p by Wilson's theorem.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 100000000

static unsigned char *sieve;

static void make_sieve(int n) {
    sieve = (unsigned char *)calloc(n, 1);
    sieve[0] = sieve[1] = 1; /* not prime */
    for (int i = 2; (long long)i * i < n; i++) {
        if (!sieve[i]) {
            for (int j = i * i; j < n; j += i)
                sieve[j] = 1;
        }
    }
}

static long long mod_inverse(long long a, long long p) {
    /* Extended Euclidean: a^{-1} mod p */
    long long t = 0, new_t = 1, r = p, new_r = a % p;
    while (new_r != 0) {
        long long q = r / new_r;
        long long tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += p;
    return t;
}

int main(void) {
    make_sieve(LIMIT);
    long long total = 0;
    for (int p = 5; p < LIMIT; p++) {
        if (!sieve[p]) {
            long long inv8 = mod_inverse(8, p);
            total += ((long long)(p - 3) * inv8) % p;
        }
    }
    free(sieve);
    printf("%lld\n", total);
    return 0;
}
