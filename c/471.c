/*
 * Project Euler Problem 471: Triangle inscribed in ellipse
 *
 * G(N) = N(2N-1)(3N+4)/24 - N(N+1)(2N+1)[H(N)-H(N/2)]/6
 * where H(n) is the n-th harmonic number.
 *
 * N = 10^11. Output in scientific notation: mantissa with 9 decimal digits + exponent.
 */
#include <stdio.h>
#include <math.h>

/*
 * Compute H(n) using asymptotic expansion:
 * H(n) ~ ln(n) + gamma + 1/(2n) - 1/(12n^2) + 1/(120n^4) - 1/(252n^6) + ...
 */
static double harmonic(long long n) {
    if (n <= 0) return 0.0;
    double nd = (double)n;
    double gamma = 0.5772156649015328606;
    double ln_n = log(nd);
    double inv_n = 1.0 / nd;
    double inv_n2 = inv_n * inv_n;

    return ln_n + gamma
        + 0.5 * inv_n
        - inv_n2 / 12.0
        + inv_n2 * inv_n2 / 120.0
        - inv_n2 * inv_n2 * inv_n2 / 252.0;
}

int main(void) {
    long long N = 100000000000LL; /* 10^11 */

    /* Compute in log space to avoid overflow:
     * term1 = N*(2N-1)*(3N+4)/24
     * term2 = N*(N+1)*(2N+1)*(H(N)-H(N/2))/6
     * ans = term1 - term2
     *
     * These are ~10^33 magnitude, use logarithmic computation.
     */
    double log_N = log10((double)N);

    /* term1 = N*(2N-1)*(3N+4)/24 */
    double t1 = (double)N * (2.0 * N - 1.0) / 24.0 * (3.0 * N + 4.0);
    /* This overflows double precision for the exact value but we need the
     * full result. Let's compute in log10 space. */

    /* log10(term1) = log10(N) + log10(2N-1) + log10(3N+4) - log10(24) */
    double log_term1 = log10((double)N) + log10(2.0 * N - 1.0) + log10(3.0 * N + 4.0) - log10(24.0);

    /* log10(term2) = log10(N) + log10(N+1) + log10(2N+1) + log10(H(N)-H(N/2)) - log10(6) */
    double h_diff = harmonic(N) - harmonic(N / 2);
    double log_term2 = log10((double)N) + log10((double)N + 1.0) + log10(2.0 * N + 1.0) + log10(h_diff) - log10(6.0);

    /* ans = 10^log_term1 - 10^log_term2
     * = 10^log_term1 * (1 - 10^(log_term2 - log_term1))
     */
    double log_ans;
    double diff_log = log_term2 - log_term1;
    double ratio = pow(10.0, diff_log);
    log_ans = log_term1 + log10(1.0 - ratio);

    int exp_part = (int)floor(log_ans);
    double mantissa = pow(10.0, log_ans - exp_part);

    printf("%.9fe%d\n", mantissa, exp_part);
    return 0;
}
