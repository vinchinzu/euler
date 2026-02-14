/*
 * Project Euler 159 - Digital root sums of factorisations
 *
 * For n from 2 to 999999, find max digital root sum over all factorisations.
 * dp[n] = max over divisors d of n (2 <= d <= sqrt(n)) of dr[d] + dp[n/d].
 */
#include <stdio.h>
#include <math.h>

#define MAX 1000000

static int dr[MAX]; /* digital root */
static int dp_val[MAX]; /* max digital root sum */

int main(void) {
    /* Compute digital roots */
    for (int i = 1; i < MAX; i++) {
        dr[i] = (i % 9 == 0) ? 9 : (i % 9);
    }

    /* DP using sieve-like approach */
    for (int n = 2; n < MAX; n++) {
        dp_val[n] = dr[n];
    }

    /* For each possible divisor d >= 2, update multiples */
    for (int d = 2; d < MAX; d++) {
        for (long long m = (long long)d * d; m < MAX; m += d) {
            int n = (int)m;
            int q = n / d;
            int v1 = dr[d] + dp_val[q];
            int v2 = dr[q] + dp_val[d];
            int v = (v1 > v2) ? v1 : v2;
            if (v > dp_val[n]) dp_val[n] = v;
        }
    }

    long long sum = 0;
    for (int n = 2; n < MAX; n++) {
        sum += dp_val[n];
    }
    printf("%lld\n", sum);
    return 0;
}
