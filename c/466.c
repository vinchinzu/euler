/*
 * Project Euler Problem 466: Distinct products
 *
 * For m from 1 to K=64, count how many n <= N=10^16 have a unique
 * representation as n*m among {m*1, m*2, ..., m*K}.
 * Uses recursive inclusion-exclusion with factored coprime splitting.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef int64_t i64;

#define N 10000000000000000LL  /* 10^16 */
#define K 64

int ff[K + 1];  /* Largest prime factor sieve */

void preff(void) {
    for (int i = 0; i <= K; i++) ff[i] = 0;
    for (int i = 2; i <= K; i++) {
        if (ff[i] == 0) {
            for (int j = i; j <= K; j += i) {
                ff[j] = i;
            }
        }
    }
}

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

i64 numNotDivisibleBy(i64 n, int *factors, int nf) {
    for (int i = 0; i < nf; i++) {
        if (factors[i] == 1) return 0;
    }

    for (int i = 0; i < nf; i++) {
        for (int j = i + 1; j < nf; j++) {
            int p = ff[gcd(factors[i], factors[j])];
            if (p > 1) {
                int newFactors1[K], newFactors2[K];
                int nf1 = 0, nf2 = 0;
                newFactors2[nf2++] = p;
                for (int k = 0; k < nf; k++) {
                    if (factors[k] % p == 0) {
                        newFactors1[nf1++] = factors[k] / p;
                    } else {
                        newFactors1[nf1++] = factors[k];
                        newFactors2[nf2++] = factors[k];
                    }
                }
                return numNotDivisibleBy(n / p, newFactors1, nf1) +
                       numNotDivisibleBy(n, newFactors2, nf2);
            }
        }
    }

    /* Inclusion-exclusion: all remaining factors are coprime */
    i64 result = 0;
    for (int subset = 0; subset < (1 << nf); subset++) {
        i64 count = n;
        for (int i = 0; i < nf; i++) {
            if (subset & (1 << i)) {
                count /= -factors[i];
            }
        }
        result += count;
    }
    return result;
}

int main(void) {
    preff();

    i64 ans = 0;
    for (int m = 1; m <= K; m++) {
        int factors[K];
        int nf = 0;
        for (int i = m + 1; i <= K; i++) {
            factors[nf++] = i / gcd(i, m);
        }
        ans += numNotDivisibleBy(N, factors, nf);
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
