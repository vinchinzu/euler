/*
 * Project Euler Problem 705: Total Inversion Count of Divisibility.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define N 100000000
#define M 1000000007ULL

/* Sieve of Eratosthenes */
int *sieve(int limit, int *count) {
    char *is_prime = calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_prime[i] = 1;
    for (int i = 2; i * i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }
    *count = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) (*count)++;
    }
    int *primes = malloc(*count * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) primes[idx++] = i;
    }
    free(is_prime);
    return primes;
}

/* Get digits of a number in REVERSE order (rightmost first) */
int digits(int n, int *arr) {
    int len = 0;
    while (n > 0) {
        arr[len++] = n % 10;
        n /= 10;
    }
    return len;
}

/* Divisors for digits 0-9 */
int divisors[10][5] = {
    {},
    {1},
    {1, 2},
    {1, 3},
    {1, 2, 4},
    {1, 5},
    {1, 2, 3, 6},
    {1, 7},
    {1, 2, 4, 8},
    {1, 3, 9},
};
int num_divisors[10] = {0, 1, 2, 2, 3, 2, 4, 2, 4, 3};

/* Modular inverse using Fermat's little theorem */
unsigned long long mod_inv(unsigned long long a) {
    unsigned long long result = 1;
    unsigned long long exp = M - 2;
    a %= M;
    while (exp > 0) {
        if (exp & 1) result = result * a % M;
        a = a * a % M;
        exp >>= 1;
    }
    return result;
}

int main() {
    int prime_count;
    int *primes = sieve(N - 1, &prime_count);

    /* Compute numSequences */
    unsigned long long numSequences = 1;
    for (int pi = prime_count - 1; pi >= 0; pi--) {
        int p = primes[pi];
        int d[10], len = digits(p, d);
        for (int i = 0; i < len; i++) {
            int k = d[i];
            if (k > 0) {
                numSequences = numSequences * num_divisors[k] % M;
            }
        }
    }

    /* Precompute mod inverses */
    unsigned long long mod_invs[10];
    for (int i = 1; i < 10; i++) {
        mod_invs[i] = mod_inv(i);
    }

    /* Main computation */
    unsigned long long counts[10] = {0};
    unsigned long long ans = 0;

    for (int pi = prime_count - 1; pi >= 0; pi--) {
        int p = primes[pi];
        int digs[10], len = digits(p, digs);
        for (int di = 0; di < len; di++) {
            int k = digs[di];
            if (k > 0) {
                int nd = num_divisors[k];
                unsigned long long inv_nd = mod_invs[nd];

                /* Count inversions */
                for (int j = 0; j < nd; j++) {
                    int div = divisors[k][j];
                    for (int i = 1; i < div; i++) {
                        ans = (ans + counts[i] * inv_nd) % M;
                    }
                }

                /* Update counts */
                for (int j = 0; j < nd; j++) {
                    int div = divisors[k][j];
                    counts[div] = (counts[div] + numSequences * inv_nd) % M;
                }
            }
        }
    }

    printf("%llu\n", ans);

    free(primes);
    return 0;
}
