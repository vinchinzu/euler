/*
 * Project Euler 444 - The Roundtable Lottery
 *
 * S_k(N) = C(N+K, K) * (H(N+K) - H(K))
 * where H(n) is the n-th harmonic number, K=20, N=10^14.
 *
 * Output in scientific notation with 9 decimal places.
 */
#include <stdio.h>
#include <math.h>
#include <stdint.h>

typedef long long ll;

static double harmonic(ll n) {
    if (n == 0) return 0.0;
    if (n < 2000) {
        double s = 0.0;
        for (ll i = 1; i <= n; i++) s += 1.0 / i;
        return s;
    }
    /* Asymptotic expansion */
    double gamma = 0.57721566490153286060651209;
    double dn = (double)n;
    return gamma + log(dn) + 1.0 / (2.0 * dn) - 1.0 / (12.0 * dn * dn);
}

int main(void) {
    ll N = 100000000000000LL; /* 10^14 */
    int K = 20;

    /* Compute log of C(N+K, K) = product_{i=1}^{K} (N+i)/i */
    double log_binom = 0.0;
    for (int i = 1; i <= K; i++) {
        log_binom += log((double)(N + i)) - log((double)i);
    }

    double h_diff = harmonic(N + K) - harmonic(K);
    /* ans = C(N+K,K) * h_diff */
    /* log(ans) = log_binom + log(h_diff) */
    double log_ans = log_binom + log(h_diff);

    /* Convert to a.bbbbbbbbbe+XXX format */
    double log10_ans = log_ans / log(10.0);
    int exponent = (int)floor(log10_ans);
    double mantissa = pow(10.0, log10_ans - exponent);

    /* Adjust if mantissa rounds up */
    if (mantissa >= 10.0) {
        mantissa /= 10.0;
        exponent++;
    }

    printf("%.9fe%d\n", mantissa, exponent);
    return 0;
}
