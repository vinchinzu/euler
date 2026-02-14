/*
 * Project Euler Problem 580: Hilbert Numbers
 *
 * Count Hilbert squarefree numbers up to N=10^16.
 * A Hilbert number is 4k+1. Hilbert squarefree means not divisible by
 * square of any Hilbert number other than 1.
 * Uses modified Mobius function with sieve of smallest prime factor.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void) {
    long long N = 10000000000000000LL;  /* 10^16 */
    int L = (int)sqrt((double)N);
    /* Ensure L^2 <= N */
    while ((long long)L * L > N) L--;
    while ((long long)(L + 1) * (L + 1) <= N) L++;

    /* Sieve smallest prime factor */
    int *ff = (int *)calloc(L + 1, sizeof(int));
    for (int i = 2; i <= L; i++) {
        if (ff[i] == 0) {
            ff[i] = i;
            if ((long long)i * i <= L) {
                for (long long j = (long long)i * i; j <= L; j += i)
                    if (ff[j] == 0)
                        ff[j] = (int)i;
            }
        }
    }

    /* table[0] = count of 4k+1 prime factors
     * table[1] = count of duplicated 4k+1 prime factors
     * table[2] = count of 4k+3 prime factors
     * table[3] = count of duplicated 4k+3 prime factors
     */
    signed char *table0 = (signed char *)calloc(L + 1, 1);
    signed char *table1 = (signed char *)calloc(L + 1, 1);
    signed char *table2 = (signed char *)calloc(L + 1, 1);
    signed char *table3 = (signed char *)calloc(L + 1, 1);

    for (int i = 3; i <= L; i += 2) {
        int d = ff[i];
        if (d == 0) d = i;
        int prev = i / d;
        table0[i] = table0[prev];
        table1[i] = table1[prev];
        table2[i] = table2[prev];
        table3[i] = table3[prev];

        int rem_type = d % 4;  /* 1 for 4k+1, 3 for 4k+3 */
        int is_square = (i % ((long long)d * d) == 0) ? 1 : 0;

        if (rem_type == 1) {
            if (is_square) table0[i]++;
            else table1[i]++;
        } else {
            if (is_square) table2[i]++;
            else table3[i]++;
        }
    }

    /* Compute Hilbert Mobius and sum */
    long long ans = 0;
    for (int i = 1; i <= L; i += 2) {
        if (table1[i] != 0) continue;  /* Has duplicated 4k+1 prime => coefficient 0 */

        int r = table2[i] + table3[i];  /* Total count of 4k+3 primes */
        int hilbert_mu = 0;

        if (table3[i] == 0) {
            /* No duplicated 4k+3 primes */
            hilbert_mu = ((r % 2 == 0) ? 1 : -1) * (r - 1);
        } else if (table3[i] == 1) {
            /* One duplicated 4k+3 prime */
            hilbert_mu = ((r - 1) % 2 == 0) ? 1 : -1;
        } else {
            continue;
        }

        hilbert_mu *= (table0[i] % 2 == 0) ? 1 : -1;

        if (hilbert_mu == 0) continue;

        /* count = ceil((N / i^2) / 4) */
        long long isq = (long long)i * i;
        long long q = N / isq;
        long long count = (q + 3) / 4;

        ans += (long long)hilbert_mu * count;
    }

    printf("%lld\n", ans);

    free(ff);
    free(table0);
    free(table1);
    free(table2);
    free(table3);
    return 0;
}
