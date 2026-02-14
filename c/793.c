/*
 * Project Euler 793 - Median of Products
 *
 * Binary search with two-pointer counting over sorted BBS sequence.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>

#define N 1000003

static long long S[N];

int cmp_ll(const void *a, const void *b) {
    long long x = *(const long long *)a;
    long long y = *(const long long *)b;
    return (x > y) - (x < y);
}

int main(void) {
    /* Generate BBS sequence */
    S[0] = 290797;
    for (int i = 1; i < N; i++) {
        S[i] = (S[i-1] * S[i-1]) % 50515093LL;
    }

    /* Sort */
    qsort(S, N, sizeof(long long), cmp_ll);

    long long low = 0;
    long long high = S[N-1] * S[N-1];
    long long target = (long long)(N - 1) * (long long)N / 2;

    while (low + 1 < high) {
        long long mid = low + (high - low) / 2;
        long long rank = 0;
        int row_count = N - 1;

        for (int i = 0; i < N; i++) {
            long long s = S[i];
            while (row_count >= 0 && s * S[row_count] >= mid) {
                row_count--;
            }
            rank += row_count + (s * s < mid ? 0 : 1);
        }

        if (rank > target) {
            high = mid;
        } else {
            low = mid;
        }
    }

    printf("%lld\n", low);
    return 0;
}
