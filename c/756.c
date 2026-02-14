/* Project Euler Problem 756: Approximating a Sum.
 * Translated from python/756.py (which uses GMP).
 *
 * We use long double for the accumulation since the answer needs 6 decimal
 * digits of precision. The key computation is:
 *   d starts at K/N, and iteratively:
 *     ans -= d * tail(i)
 *     d *= (N-K-i+1) / (N-i)
 * where tail(i) = sum of phi(j) for j=i..N.
 *
 * We use integer arithmetic for phi sieve, and long double for the
 * accumulation.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main() {
    int N = 12345678;
    int K = 12345;

    /* Sieve for Euler's totient */
    int *phi = (int *)malloc((N + 1) * sizeof(int));
    for (int i = 0; i <= N; i++) phi[i] = i;
    for (int i = 2; i <= N; i++) {
        if (phi[i] == i) {  /* i is prime */
            for (int j = i; j <= N; j += i)
                phi[j] = phi[j] / i * (i - 1);
        }
    }

    /* Prefix sums */
    long long *sum_phis = (long long *)malloc((N + 1) * sizeof(long long));
    sum_phis[0] = 0;
    for (int i = 1; i <= N; i++)
        sum_phis[i] = sum_phis[i - 1] + phi[i];
    free(phi);

    /* Accumulation using long double for precision */
    long double d = (long double)K / (long double)N;
    long double ans = (long double)sum_phis[N];

    for (int i = 1; i <= N; i++) {
        long long tail = sum_phis[N] - sum_phis[i - 1];
        long double diff = d * (long double)tail;

        if (diff == 0.0L) break;

        ans -= diff;

        long long nr = (long long)N - K - i + 1;
        if (nr <= 0) break;
        long long dn = (long long)N - i;
        if (dn <= 0) break;

        d = d * (long double)nr / (long double)dn;
    }

    printf("%.6Lf\n", ans);

    free(sum_phis);
    return 0;
}
