/*
 * Project Euler Problem 229: Four Representations using Squares
 *
 * Count n <= 2*10^9 representable as a^2+b^2, a^2+2b^2, a^2+3b^2, a^2+7b^2
 * simultaneously (a,b > 0 for each).
 *
 * Use bit-sieve: for each k in {1,2,3,7}, mark all n = a^2 + k*b^2.
 * Then count n marked in all four sieves.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 2000000000LL

int main(void) {
    /* We use 4 bit arrays. Each needs LIMIT/8 bytes = 250 MB.
     * That's too much (1 GB total). Instead, process in chunks.
     *
     * Actually, let's use a single pass approach:
     * For each k, create a bit sieve, then AND them together.
     * But 250MB per sieve * 4 = 1GB. Too much.
     *
     * Better: process in chunks of size CHUNK. For each chunk,
     * build the 4 sieves, AND them, count bits.
     */
    long long N = LIMIT;
    int CHUNK = 50000000; /* 50M per chunk, ~6.25 MB per bit sieve */
    int ks[] = {1, 2, 3, 7};

    /* Allocate bit arrays */
    int sieve_bytes = (CHUNK + 7) / 8;
    unsigned char *combined = (unsigned char *)malloc(sieve_bytes);
    unsigned char *temp = (unsigned char *)malloc(sieve_bytes);

    long long total = 0;

    for (long long base = 1; base <= N; base += CHUNK) {
        long long hi = base + CHUNK - 1;
        if (hi > N) hi = N;
        int len = (int)(hi - base + 1);
        int bytes = (len + 7) / 8;

        memset(combined, 0xFF, bytes);
        /* Clear excess bits in last byte */
        if (len % 8 != 0)
            combined[bytes - 1] = (1 << (len % 8)) - 1;

        for (int ki = 0; ki < 4; ki++) {
            int k = ks[ki];
            memset(temp, 0, bytes);

            /* Mark all n = a^2 + k*b^2 in [base, hi] with a,b > 0 */
            long long max_b2 = hi / k;
            long long max_b = (long long)sqrt((double)max_b2);
            while ((max_b + 1) * (max_b + 1) * k + 1 <= hi) max_b++;

            for (long long b = 1; b * b * k < hi; b++) {
                long long kb2 = k * b * b;
                /* a^2 = n - kb2, need a >= 1, so n >= kb2 + 1 */
                long long min_n = kb2 + 1;
                if (min_n > hi) break;
                if (min_n < base) min_n = base;

                /* a^2 = min_n - kb2, need a = ceil(sqrt(min_n - kb2)) */
                long long diff_lo = min_n - kb2;
                long long a_lo = (long long)sqrt((double)diff_lo);
                if (a_lo * a_lo < diff_lo) a_lo++;
                if (a_lo < 1) a_lo = 1;

                long long diff_hi = hi - kb2;
                long long a_hi = (long long)sqrt((double)diff_hi);
                /* Ensure a_hi^2 <= diff_hi */
                while (a_hi * a_hi > diff_hi) a_hi--;

                for (long long a = a_lo; a <= a_hi; a++) {
                    long long n = a * a + kb2;
                    long long idx = n - base;
                    temp[idx >> 3] |= (1 << (idx & 7));
                }
            }

            /* AND with combined */
            for (int i = 0; i < bytes; i++)
                combined[i] &= temp[i];
        }

        /* Count set bits */
        for (int i = 0; i < bytes; i++) {
            total += __builtin_popcount(combined[i]);
        }
    }

    printf("%lld\n", total);

    free(combined);
    free(temp);
    return 0;
}
