/*
 * Project Euler 211: Divisor Square Sum
 *
 * Find the sum of all k < 64,000,000 such that sigma_2(k) is a perfect square.
 * sigma_2(k) = sum of d^2 for all divisors d of k.
 *
 * Use multiplicative sieve: compute sigma_2 via smallest prime factor decomposition.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define LIMIT 64000000

int main(void) {
    /* Build smallest prime factor sieve */
    int *spf = malloc((LIMIT + 1) * sizeof(int));
    for (int i = 0; i <= LIMIT; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= LIMIT; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= LIMIT; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }

    /* Compute sigma_2 using SPF */
    long long *sig2 = malloc((LIMIT + 1) * sizeof(long long));
    sig2[0] = 0;
    sig2[1] = 1;

    for (int i = 2; i <= LIMIT; i++) {
        int p = spf[i];
        int n = i;
        long long mult = 1;
        while (n % p == 0) {
            n /= p;
            mult = mult * (long long)p * p + 1;
        }
        sig2[i] = sig2[n] * mult;
    }

    free(spf);

    long long ans = 0;
    for (int k = 1; k < LIMIT; k++) {
        long long s = sig2[k];
        long long r = (long long)sqrt((double)s);
        /* Correct the integer square root */
        while (r * r < s) r++;
        while (r * r > s) r--;
        if (r * r == s) ans += k;
    }

    printf("%lld\n", ans);
    free(sig2);
    return 0;
}
