/*
 * Project Euler Problem 810: XOR-Primes.
 *
 * Sieve for XOR-primes (analogous to normal primes but with XOR multiplication).
 * Find the 5,000,000th XOR-prime.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define N 5000000
#define L (1 << 27)  /* 134217728 */

int main() {
    /* Sieve for XOR-primes */
    char *sieve = calloc(L, 1);
    for (int i = 2; i < L; i++) sieve[i] = 1;

    for (int i = 2; i < L; i++) {
        if (sieve[i]) {
            /* Mark composites: for j >= i, mark xor_product(i, j) */
            for (long long j = i; j < L; j++) {
                /* XOR product: m = XOR of j*(k & -k) for each set bit k in i */
                long long m = 0;
                for (int k = i; k > 0; k -= k & -k)
                    m ^= j * (k & -k);
                if (m >= L) break;
                sieve[(int)m] = 0;
            }
        }
    }

    /* Count XOR-primes */
    int count = 0;
    int ans = 0;
    for (int i = 2; count < N; i++) {
        if (sieve[i]) {
            count++;
            ans = i;
        }
    }

    printf("%d\n", ans);
    free(sieve);
    return 0;
}
