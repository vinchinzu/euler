/*
 * Project Euler Problem 614: Special partitions II
 *
 * Embedded C extracted from Python wrapper.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;
typedef __int128 i128;

#define N 10000000
#define M 1000000007LL
#define L (1 << 14)

i64 isqrt(i64 n) {
    i64 x = (i64)sqrtl((long double)n);
    while (x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

int isSq(i64 n) {
    i64 r = isqrt(n);
    return r * r == n;
}

i64 tr(int t) {
    return (i64)t * (t + 1) / 2;
}

int parity(int n) {
    return (n % 2 == 0) ? 1 : -1;
}

int main() {
    i64 *P = (i64*)calloc(N + L, sizeof(i64));

    for (int page = 0; page * L < N; page++) {
        /* Process previous pages */
        for (int prevPage = 0; prevPage < page; prevPage++) {
            int min_t = (int)((sqrtl(8.0 * (page - (prevPage + 1)) * L + 1) + 1) / 2);
            for (int t = min_t; tr(t) < (i64)(page + 1 - prevPage) * L; t++) {
                i64 tr_t = tr(t);
                for (int i = (page * L > prevPage * L + (int)tr_t) ? page * L : prevPage * L + (int)tr_t;
                     i < (page + 1) * L && i - tr_t < (prevPage + 1) * L;
                     i++) {
                    P[i] = (P[i] + (i64)parity((t - 1) / 2) * P[i - tr_t] % M + M) % M;
                }
            }
        }

        /* Process current page */
        for (int i = page * L; i <= N && i < (page + 1) * L; i++) {
            i64 res = 4LL * i + 1;
            if (isSq(res)) {
                i64 root = isqrt(res);
                P[i] = (P[i] + parity((int)((root / 2 + 1) / 2)) + M) % M;
            }

            for (int t = 1; tr(t) <= i - page * L; t++) {
                P[i] = (P[i] + (i64)parity((t - 1) / 2) * P[i - tr(t)] % M + M) % M;
            }
            P[i] = (P[i] % M + M) % M;
        }
    }

    i64 ans = 0;
    for (int i = 1; i <= N; i++) {
        ans = (ans + P[i]) % M;
    }

    printf("%lld\n", ans);

    free(P);
    return 0;
}
