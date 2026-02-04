"""Project Euler Problem 681: Maximal Area.

Find SP(10^6): sum of a+b+c+d over all a<=b<=c<=d such that the maximum
area M(a,b,c,d) of a quadrilateral with those sides is a positive integer <= N.

M(a,b,c,d) = sqrt((s-a)(s-b)(s-c)(s-d)) where s=(a+b+c+d)/2 (Brahmagupta).
Let w=s-d, x=s-c, y=s-b, z=s-a with w<=x<=y<=z, w>=1, z<w+x+y.
Then wxyz = K^2 where K = M, and a+b+c+d = w+x+y+z.

Split into two cases:
- D=0 (z=y): wx must be a perfect square. Closed-form sum per (w,x) pair.
- D>0 (z>y): For each K, enumerate divisor triples of K^2. Parallelized.

Implemented in C with OpenMP for performance.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve() -> int:
    """Solve Problem 681 using compiled C."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <omp.h>

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

    /* Case 1: D=0 (z=y). Then wxyz = wxy^2 must be perfect square, */
    /* so wx must be perfect square. Sum = w+x+2y, must be even => w+x even. */
    /* K = y*sqrt(wx) <= N => y <= N/sqrt(wx). Closed form sum. */
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

    /* Case 2: D>0 (z>y). For each K, enumerate (w,x,y) divisor triples */
    /* of K^2 with w<=x<=y, z=K^2/(wxy) > y, z < w+x+y. */
    long long ans_dpos = 0;

    #pragma omp parallel reduction(+:ans_dpos)
    {
        long long divs[4096];

        #pragma omp for schedule(dynamic, 32) nowait
        for (int K = 4; K <= N; K++) {
            if (spf[K] == K) continue; /* skip primes: they contribute nothing */

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
    }

    printf("%lld\n", ans + ans_dpos);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O3', '-march=native', '-fopenmp',
                       '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                check=True, timeout=27)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
