/*
 * Project Euler Problem 231: Prime Factorisation of Binomial Coefficients
 *
 * Find the sum of prime factors (with multiplicity) of C(20000000, 15000000).
 * Sum = sum of spf decomposition of numerator - denominator.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define NLIMIT 20000000

static int spf[NLIMIT + 1];

int main(void) {
    int N = NLIMIT;
    int K = 15000000;

    /* Build smallest prime factor sieve */
    for (int i = 0; i <= N; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= N; j += i) {
                if (spf[j] == j)
                    spf[j] = i;
            }
        }
    }

    long long ans = 0;

    /* Numerator: N * (N-1) * ... * (N-K+1) = product of (K+1) to N */
    /* Denominator: K! = 1 * 2 * ... * K */
    /* Actually C(N,K) = N! / (K! * (N-K)!) */
    /* Sum of prime factors = sum_pf(N!) - sum_pf(K!) - sum_pf((N-K)!) */

    /* For each i from 2 to N, decompose and add to ans based on ranges */
    /* Simpler: iterate i from 1 to N-K (which is 5000000) */
    /* numerator factor: N - i + 1 for i=1..N-K => N, N-1, ..., K+1 */
    /* denominator factor: i for i=1..N-K */

    for (int i = 0; i < N - K; i++) {
        /* Numerator: N - i */
        int n = N - i;
        while (n > 1) {
            ans += spf[n];
            n /= spf[n];
        }

        /* Denominator: i + 1 */
        n = i + 1;
        while (n > 1) {
            ans -= spf[n];
            n /= spf[n];
        }
    }

    printf("%lld\n", ans);
    return 0;
}
