/*
 * Project Euler 278: Linear Combinations of Semiprimes
 *
 * For primes p < q < r < 5000:
 *   f(pq, pr, qr) = 2pqr - (pq + pr + qr)
 * Sum over all such triples.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

#define LIMIT 5000

static char is_prime[LIMIT + 1];
static int primes[700];
static int num_primes = 0;

int main(void) {
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i <= LIMIT; i++)
        if (is_prime[i])
            for (int j = i * i; j <= LIMIT; j += i)
                is_prime[j] = 0;

    for (int i = 2; i <= LIMIT; i++)
        if (is_prime[i])
            primes[num_primes++] = i;

    /*
     * Sum = sum_{i<j<k} [2*p*q*r - p*q - p*r - q*r]
     *     = 2 * sum(pqr) - sum(pq+pr+qr)
     *
     * Use precomputed sums for efficiency:
     *   S1 = sum of all primes
     *   S2 = sum of all pairs products
     *   S3 = sum of all triple products
     *
     * sum(pq + pr + qr) over i<j<k:
     *   For a pair (p,q), it appears in (num_primes - j - 1) triples for the pr+qr terms,
     *   plus as pq it appears in triples with any r > q.
     *   Actually: sum_{i<j<k} (p_i*p_j + p_i*p_k + p_j*p_k)
     *   = sum_{i<j<k} p_i*p_j + sum_{i<j<k} p_i*p_k + sum_{i<j<k} p_j*p_k
     *
     * This gets complex. Better to just use the O(n^2) approach since n=669:
     * For each pair (i,j), accumulate contributions from all k > j.
     */

    typedef __int128 i128;
    i128 ans = 0;

    /* Precompute suffix sums of primes */
    long long *suffix = (long long *)malloc((num_primes + 1) * sizeof(long long));
    suffix[num_primes] = 0;
    for (int i = num_primes - 1; i >= 0; i--)
        suffix[i] = suffix[i + 1] + primes[i];

    for (int i = 0; i < num_primes; i++) {
        for (int j = i + 1; j < num_primes; j++) {
            long long p = primes[i];
            long long q = primes[j];
            long long sum_r = suffix[j + 1]; /* sum of primes[j+1..end] */
            int count_r = num_primes - j - 1;
            if (count_r <= 0) continue;

            /* sum_{k>j} [2pqr - pq - pr - qr]
             * = 2pq * sum_r - pq * count_r - p * sum_r - q * sum_r
             * = (2pq - p - q) * sum_r - pq * count_r
             */
            i128 contrib = (i128)(2 * p * q - p - q) * sum_r - (i128)(p * q) * count_r;
            ans += contrib;
        }
    }

    /* Print __int128 */
    char buf[50];
    int pos = 0;
    i128 v = ans;
    if (v == 0) { printf("0\n"); free(suffix); return 0; }
    while (v > 0) { buf[pos++] = '0' + (int)(v % 10); v /= 10; }
    for (int i = pos - 1; i >= 0; i--) putchar(buf[i]);
    putchar('\n');

    free(suffix);
    return 0;
}
