/*
 * Project Euler 203: Squarefree Binomial Coefficients
 *
 * Find the sum of distinct squarefree numbers in first 51 rows of Pascal's triangle.
 */
#include <stdio.h>
#include <math.h>

static long long binom[51][51];

static int is_squarefree(long long n) {
    if (n <= 1) return 1;
    for (long long d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            n /= d;
            if (n % d == 0) return 0;
        }
    }
    return 1;
}

int main(void) {
    int N = 51;

    /* Build Pascal's triangle */
    for (int i = 0; i < N; i++) {
        binom[i][0] = binom[i][i] = 1;
        for (int j = 1; j < i; j++)
            binom[i][j] = binom[i-1][j-1] + binom[i-1][j];
    }

    /* Collect distinct squarefree values using a sorted list (values are small enough) */
    /* Max value is C(50,25) ~ 1.26e14, but there are at most ~51*26 ~ 1326 entries */
    long long vals[2000];
    int nv = 0;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j <= i / 2; j++) {
            long long v = binom[i][j];
            if (!is_squarefree(v)) continue;
            /* Check if already in list */
            int found = 0;
            for (int k = 0; k < nv; k++) {
                if (vals[k] == v) { found = 1; break; }
            }
            if (!found) vals[nv++] = v;
        }
    }

    long long ans = 0;
    for (int k = 0; k < nv; k++) ans += vals[k];

    printf("%lld\n", ans);
    return 0;
}
