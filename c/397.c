/*
 * Project Euler Problem 397: Triangle on Parabola
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define K_MAX 1000000
#define N_VAL 1000000000LL
#define SIEVE_MAX (2 * K_MAX + 1)

static int smallest_prime[SIEVE_MAX];

/* Divisor storage */
static long long divs[4096];
static int ndivs;

static void gen_divisors(long long n, int val) {
    divs[0] = 1;
    ndivs = 1;

    int tmp = val;
    while (tmp > 1) {
        int p = smallest_prime[tmp];
        while (tmp % p == 0) tmp /= p;

        if (n % p != 0) continue;
        int e = 0;
        long long t = n;
        while (t % p == 0) { e++; t /= p; }

        int old = ndivs;
        long long pk = 1;
        for (int j = 0; j < e; j++) {
            pk *= p;
            for (int i = 0; i < old; i++) {
                divs[ndivs++] = divs[i] * pk;
            }
        }
    }
}

int main(void) {
    memset(smallest_prime, 0, sizeof(smallest_prime));
    for (int i = 2; i < SIEVE_MAX; i++) {
        if (smallest_prime[i] == 0) {
            for (int j = i; j < SIEVE_MAX; j += i) {
                if (smallest_prime[j] == 0)
                    smallest_prime[j] = i;
            }
        }
    }

    long long ans = 0;

    for (long long k = 1; k <= K_MAX; k++) {
        long long prod = 2 * k * k;
        gen_divisors(prod, (int)(2 * k));

        for (int di = 0; di < ndivs; di++) {
            long long d = divs[di];

            /* Case 1 */
            {
                long long apb = -(k + d);
                long long bpc = prod / d + k;
                long long lo = (apb + 1) / 2;
                long long hi = (bpc - 1) / 2;
                long long lo2 = bpc - N_VAL;
                long long hi2 = apb + N_VAL;
                if (lo2 > lo) lo = lo2;
                if (hi2 < hi) hi = hi2;
                if (lo <= hi)
                    ans += hi - lo + 1;

                long long den = d + 2 * k;
                if (prod % den == 0) {
                    long long apc = prod / den - k;
                    long long s = apb + bpc + apc;
                    if (s % 2 == 0) {
                        long long a = s / 2 - bpc;
                        long long c = s / 2 - apb;
                        if (a >= -N_VAL && c <= N_VAL)
                            ans -= 2;
                    }
                }
            }

            /* Case 2 */
            {
                long long apb = k - d;
                long long bpc = prod / d - k;
                long long lo;
                if (bpc >= 0)
                    lo = bpc / 2 + 1;
                else
                    lo = (bpc - 1) / 2 + 1;
                long long lo2 = bpc - N_VAL;
                if (lo2 > lo) lo = lo2;
                long long hi = apb + N_VAL;
                if (N_VAL < hi) hi = N_VAL;
                if (lo <= hi)
                    ans += 2 * (hi - lo + 1);

                if (d != 2 * k) {
                    long long den = 2 * k - d;
                    if (den > 0 ? (prod % den == 0) : (prod % (-den) == 0)) {
                        long long apc = k - prod / den;
                        long long s = apb + bpc + apc;
                        if (s % 2 == 0) {
                            long long a = s / 2 - bpc;
                            long long b = s / 2 - apc;
                            long long c = s / 2 - apb;
                            if (a >= -N_VAL && c < b && b <= N_VAL)
                                ans--;
                        }
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
