/*
 * Project Euler Problem 820: Nth digit of reciprocal.
 *
 * d_n(x) = nth decimal digit of fractional part of x.
 * Find sum_{k=1}^N d_n(1/k).
 *
 * Compute 10^{N-1} mod k, then digit = floor(10 * (10^{N-1} mod k) / k).
 * Optimization: for k <= N/2, use 10^{N-1} mod 2k then reduce mod k.
 */
#include <stdio.h>
#include <stdlib.h>

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base = base % mod;
    while (exp > 0) {
        if (exp & 1)
            result = (result * base) % mod;
        base = (base * base) % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    long long N = 10000000LL;
    long long B = 10;

    long long *pows = (long long *)calloc(N + 1, sizeof(long long));
    if (!pows) { fprintf(stderr, "alloc fail\n"); return 1; }

    long long k;
    for (k = N; k >= 1; k--) {
        if (2 * k <= N) {
            pows[k] = pows[2 * k] % k;
        } else {
            pows[k] = pow_mod(B, N - 1, k);
        }
    }

    long long ans = 0;
    for (k = 1; k <= N; k++) {
        ans += (pows[k] * B) / k;
    }

    printf("%lld\n", ans);
    free(pows);
    return 0;
}
