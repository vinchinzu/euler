/*
 * Project Euler 869 - Prime Bit Guessing Game
 *
 * E(N) = expected score when optimally guessing binary of random prime <= N.
 *
 * Algorithm: Process bit-by-bit from LSB. At each level, maintain groups
 * of primes with same observed lower bits. For each group, optimal guess
 * is majority bit. Score contribution = max(count_0, count_1).
 * Stable partition by bit to form next level's groups.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 100000000

static unsigned char *sieve;

static void build_sieve(void) {
    long long n = LIMIT;
    sieve = calloc(n / 16 + 2, 1);
    /* Bit k of byte i represents number 2*(8*i + k) + 1 = 16*i + 2*k + 1 */
    /* Actually simpler: just use n/2 bits for odd numbers */
    /* sieve[i] = 1 means 2*i+1 is composite (for i >= 1) */

    /* Let's use a simpler sieve for odd numbers */
    free(sieve);
    sieve = calloc(n / 2 + 1, 1);
    /* sieve[i] represents 2*i+1. sieve[0] = 1 (which is not prime) */
    sieve[0] = 1; /* 1 is not prime */

    int lim = (int)sqrt((double)n);
    for (int i = 1; 2 * i + 1 <= lim; i++) {
        if (!sieve[i]) {
            int p = 2 * i + 1;
            /* Mark multiples of p starting from p*p */
            for (long long j = (long long)p * p; j <= n; j += 2 * p) {
                int idx = (int)((j - 1) / 2);
                sieve[idx] = 1;
            }
        }
    }
}

static inline int bitlen(int x) {
    return 32 - __builtin_clz(x);
}

int main(void) {
    build_sieve();

    /* Collect primes */
    int *primes = malloc(6000000 * sizeof(int));
    int nprimes = 0;
    primes[nprimes++] = 2;
    for (int i = 1; 2 * i + 1 <= LIMIT; i++) {
        if (!sieve[i])
            primes[nprimes++] = 2 * i + 1;
    }

    int max_bits = bitlen(LIMIT);

    /* Process level by level */
    int *arr = malloc(nprimes * sizeof(int));
    int *tmp = malloc(nprimes * sizeof(int));
    memcpy(arr, primes, nprimes * sizeof(int));

    /* Group boundaries */
    int *gs = malloc((nprimes + 2) * sizeof(int));
    int *ge = malloc((nprimes + 2) * sizeof(int));
    int *ngs = malloc((2 * nprimes + 2) * sizeof(int));
    int *nge = malloc((2 * nprimes + 2) * sizeof(int));

    int ng = 1;
    gs[0] = 0; ge[0] = nprimes;

    double total_score = 0.0;

    for (int level = 0; level < max_bits; level++) {
        int nng = 0;

        for (int g = 0; g < ng; g++) {
            int s = gs[g], e = ge[g];

            /* Count 0s and 1s at this bit, and count continuing primes */
            int c0 = 0, c1 = 0;
            int cont0 = 0, cont1 = 0;

            for (int i = s; i < e; i++) {
                int p = arr[i];
                int bit = (p >> level) & 1;
                if (bit == 0) c0++; else c1++;
                if (bitlen(p) > level + 1) {
                    if (bit == 0) cont0++; else cont1++;
                }
            }

            total_score += (c0 > c1) ? c0 : c1;

            /* Stable partition continuing primes by bit */
            int pos0 = s;
            int pos1 = s + cont0;

            for (int i = s; i < e; i++) {
                int p = arr[i];
                if (bitlen(p) <= level + 1) continue;
                int bit = (p >> level) & 1;
                if (bit == 0) tmp[pos0++] = p;
                else tmp[pos1++] = p;
            }

            if (cont0 > 0) {
                ngs[nng] = s;
                nge[nng] = s + cont0;
                nng++;
            }
            if (cont1 > 0) {
                ngs[nng] = s + cont0;
                nge[nng] = s + cont0 + cont1;
                nng++;
            }
        }

        /* Copy tmp to arr for active regions */
        for (int g = 0; g < nng; g++)
            memcpy(arr + ngs[g], tmp + ngs[g], (nge[g] - ngs[g]) * sizeof(int));

        int *t;
        t = gs; gs = ngs; ngs = t;
        t = ge; ge = nge; nge = t;
        ng = nng;

        if (ng == 0) break;
    }

    printf("%.8f\n", total_score / nprimes);

    free(arr); free(tmp); free(gs); free(ge); free(ngs); free(nge);
    free(primes); free(sieve);
    return 0;
}
