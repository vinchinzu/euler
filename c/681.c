/*
 * Project Euler 681 - Maximal Area
 *
 * Enumerate (w,x,y,z) such that wxyz = K^2 for some K <= N,
 * w<=x<=y<=z, z < w+x+y, and w+x+y+z is even.
 * Sum all such w+x+y+z.
 *
 * Case 1: z==y => wx must be a perfect square.
 * Case 2: z>y  => enumerate divisor triples of K^2.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAXN 1000001
static int spf[MAXN];

void sieve(void) {
    for (int i = 0; i < MAXN; i++) spf[i] = i;
    for (int i = 2; i * i < MAXN; i++)
        if (spf[i] == i)
            for (int j = i * i; j < MAXN; j += i)
                if (spf[j] == j) spf[j] = i;
}

int main(void) {
    long long N = 1000000;
    long long N2 = N * N;
    sieve();

    /* Case 1: D=0 (z=y). Then wxyz = wxy^2 must be perfect square,
       so wx must be perfect square. */
    long long ans = 0;
    for (long long w = 1; w*w*w*w <= N2; w++) {
        for (long long x = w; w*x*x*x <= N2; x++) {
            long long wx = w * x;
            long long swx = (long long)sqrt((double)wx);
            if (swx * swx != wx) continue;
            if ((w + x) & 1) continue;
            long long y_max = N / swx;
            if (y_max < x) continue;
            long long count = y_max - x + 1;
            ans += count * (w + x + x + y_max);
        }
    }

    /* Case 2: D>0 (z>y). For each K, enumerate (w,x,y) divisor triples
       of K^2 with w<=x<=y, z=K^2/(wxy) > y, z < w+x+y. */
    long long ans_dpos = 0;

    long long divs[4096];

    for (int K = 4; K <= N; K++) {
        if (spf[K] == K) continue; /* skip primes */

        int prms[20], exps[20], np = 0;
        int tmp = K;
        while (tmp > 1) {
            int p = spf[tmp];
            prms[np] = p; exps[np] = 0;
            while (tmp % p == 0) { tmp /= p; exps[np]++; }
            exps[np] *= 2;
            np++;
        }

        long long k2 = (long long)K * K;
        int nd = 0;
        divs[nd++] = 1;
        for (int i = 0; i < np; i++) {
            int old = nd;
            long long pp = 1;
            for (int e = 0; e < exps[i]; e++) {
                pp *= prms[i];
                for (int j = 0; j < old; j++)
                    divs[nd++] = divs[j] * pp;
            }
        }
        /* Insertion sort */
        for (int i = 1; i < nd; i++) {
            long long key = divs[i];
            int j = i - 1;
            while (j >= 0 && divs[j] > key) { divs[j+1] = divs[j]; j--; }
            divs[j+1] = key;
        }

        for (int i = 0; i < nd; i++) {
            long long w = divs[i];
            if (w*w*w*w > k2) break;
            long long r1 = k2 / w;
            for (int j = i; j < nd; j++) {
                long long x = divs[j];
                if (x*x*x > r1) break;
                if (r1 % x != 0) continue;
                long long r2 = r1 / x;
                for (int l = j; l < nd; l++) {
                    long long y = divs[l];
                    if (y*y > r2) break;
                    if (r2 % y != 0) continue;
                    long long z = r2 / y;
                    if (z <= y) continue;
                    if (z >= w+x+y) continue;
                    long long total = w+x+y+z;
                    if (total & 1) continue;
                    ans_dpos += total;
                }
            }
        }
    }

    printf("%lld\n", ans + ans_dpos);
    return 0;
}
