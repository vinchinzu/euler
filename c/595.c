/*
 * Project Euler Problem 595: Incremental Random Sort.
 *
 * Find the expected number of shuffles required to sort numbers 1 to N.
 * Uses inclusion-exclusion to compute g(k), the number of sequences with
 * exactly k consecutive increasing subsequences.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 53

/* Use long double for better precision with large factorials */
typedef long double ld;

int main() {
    int N = 52;

    /* Precompute comb(n, k) as doubles */
    /* comb[n][k] */
    ld comb[MAXN][MAXN];
    memset(comb, 0, sizeof(comb));
    for (int n = 0; n < MAXN; n++) {
        comb[n][0] = 1.0L;
        for (int k = 1; k <= n; k++) {
            comb[n][k] = comb[n-1][k-1] + comb[n-1][k];
        }
    }

    /* Precompute factorials */
    ld fact[MAXN];
    fact[0] = 1.0L;
    for (int i = 1; i < MAXN; i++)
        fact[i] = fact[i-1] * i;

    ld f[MAXN];
    memset(f, 0, sizeof(f));

    for (int n = 2; n <= N; n++) {
        ld g[MAXN];
        memset(g, 0, sizeof(g));

        for (int k = 1; k <= n; k++) {
            g[k] = comb[n-1][k-1] * fact[k];
            for (int i = 1; i < k; i++) {
                g[k] -= comb[n-i][k-i] * g[i];
            }
        }

        f[n] = g[n];
        for (int j = 2; j < n; j++) {
            f[n] += g[j] * (1.0L + f[j]);
        }
        f[n] /= (fact[n] - g[n]);
    }

    printf("%.8Lf\n", f[N]);
    return 0;
}
