/*
 * Project Euler 774 - Conjunctive Sequences
 *
 * Computes c(n, b) mod 998244353 using Sum Over Subsets (SOS) DP.
 *
 * Algorithm:
 * - F_{m+1}(S) = w(S) * (total_m - SOS(F_m)(complement(S)))
 * - Single array, in-place pair-wise update to avoid second array.
 *
 * Time: O(n * k * 2^k). Memory: O(2^k) single int64 array.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MOD 998244353LL

int main(int argc, char *argv[]) {
    int n = 123;
    long long b_val = 123456789LL;

    if (argc >= 3) {
        n = atoi(argv[1]);
        b_val = atoll(argv[2]);
    }

    int k = 0;
    { long long t = b_val; while (t > 0) { k++; t >>= 1; } }
    if (k == 0) k = 1;

    long long size = 1LL << k;
    long long full_mask = size - 1;
    long long half_size = size >> 1;

    int64_t *A = (int64_t *)calloc(size, sizeof(int64_t));
    if (!A) { fprintf(stderr, "Alloc failed\n"); return 1; }

    for (long long S = 1; S <= b_val; S++)
        A[S] = 1;

    for (int step = 0; step < n - 1; step++) {
        /* SOS transform in-place */
        for (int i = 0; i < k; i++) {
            long long half = 1LL << i;
            long long st = half << 1;
            /* Process blocks. Inner loop is tight for auto-vectorization. */
            for (long long j = 0; j < size; j += st) {
                int64_t *lo = A + j;
                int64_t *hi = A + j + half;
                for (long long l = 0; l < half; l++)
                    hi[l] += lo[l];
            }
        }

        /* Mod entire array */
        for (long long j = 0; j < size; j++)
            A[j] %= MOD;

        long long total = A[full_mask];

        /* In-place pair-wise update.
         * For each pair (S, Sc) with S < Sc (where Sc = full_mask ^ S):
         * - Read g_s = A[S], g_sc = A[Sc]
         * - Compute new values based on w(S) and w(Sc)
         * - Write both
         * S ranges from 0 to half_size - 1 (since S < Sc = full_mask ^ S
         * iff the highest bit of S is 0, i.e., S < 2^(k-1)).
         * Note: S == Sc is impossible since full_mask is odd.
         */
        for (long long S = 0; S < half_size; S++) {
            long long Sc = full_mask ^ S;
            int64_t g_s = A[S];
            int64_t g_sc = A[Sc];
            int64_t new_s, new_sc;

            /* w(S) = (S >= 1 && S <= b_val) */
            if (S >= 1 && S <= b_val)
                new_s = (total - g_sc + MOD) % MOD;
            else
                new_s = 0;

            /* w(Sc) = (Sc >= 1 && Sc <= b_val) */
            if (Sc >= 1 && Sc <= b_val)
                new_sc = (total - g_s + MOD) % MOD;
            else
                new_sc = 0;

            A[S] = new_s;
            A[Sc] = new_sc;
        }
    }

    long long answer = 0;
    for (long long S = 1; S <= b_val; S++)
        answer = (answer + A[S]) % MOD;

    printf("%lld\n", answer);
    free(A);
    return 0;
}
