/*
 * Project Euler Problem 307: Chip Defects
 *
 * k=20000 defects randomly distributed among n=1000000 chips.
 * p(k,n) = probability at least one chip has 3+ defects.
 *
 * P(no 3+) = sum_{x=0}^{k/2} P(x), where P(x) = prob exactly x chips have 2 defects.
 * P(0) = falling(n,k) / n^k = prod_{i=0}^{k-1} (1-i/n)
 * P(x)/P(x-1) = (k-2x+2)*(k-2x+1) / (2*x*(n-k+x))
 *
 * Work in log-space, accumulate using log-sum-exp.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int k = 20000;
    long long n = 1000000;

    /* Compute ln(P(0)) */
    long double ln_p0 = 0.0L;
    for (int i = 0; i < k; i++) {
        ln_p0 += logl(1.0L - (long double)i / (long double)n);
    }

    /* Accumulate sum of P(x) using log-sum-exp.
     * We keep a running log of P(x) and accumulate into a sum.
     * Since the values span many orders of magnitude, we use a scaled approach:
     * maintain the sum as (log_scale, mantissa) where sum = mantissa * exp(log_scale).
     */

    /* Actually, since the peak P(x) ~ 0.008, and total ~ 0.269, we can just
     * accumulate in long double directly, but we need to handle the varying
     * magnitudes. Use the running ratio approach with a renormalization:
     * start from the peak and go both ways. Or use compensated summation.
     */

    /* Better approach: find the peak x, then compute P(peak) directly,
     * and accumulate ratios from there in both directions. */

    /* Find peak: P(x)/P(x-1) > 1 when (k-2x+2)(k-2x+1) > 2x(n-k+x) */
    /* For our params: this is when the ratio > 1.
     * At x=0 to peak, ratios > 1; after peak, ratios < 1. */

    long double ln_px = ln_p0;
    long double max_ln_px = ln_p0;
    int peak_x = 0;

    for (int x = 1; x <= k/2; x++) {
        long double num = (long double)(k - 2*x + 2) * (long double)(k - 2*x + 1);
        long double den = 2.0L * (long double)x * (long double)(n - k + x);
        long double ratio = num / den;
        ln_px += logl(ratio);
        if (ln_px > max_ln_px) {
            max_ln_px = ln_px;
            peak_x = x;
        }
        if (ratio < 0.5L && x > peak_x + 100) break;
    }

    /* Now compute the sum centered around the peak.
     * sum = exp(max_ln_px) * sum_i(exp(ln_P(i) - max_ln_px))
     * Since we know max_ln_px, all terms exp(ln_P(i) - max_ln_px) <= 1.
     */

    /* Recompute all terms */
    ln_px = ln_p0;
    long double sum_scaled = expl(ln_p0 - max_ln_px);

    for (int x = 1; x <= k/2; x++) {
        long double num = (long double)(k - 2*x + 2) * (long double)(k - 2*x + 1);
        long double den = 2.0L * (long double)x * (long double)(n - k + x);
        ln_px += logl(num / den);
        long double contrib = expl(ln_px - max_ln_px);
        sum_scaled += contrib;
        if (x > peak_x + 500 && contrib < 1e-18L) break;
    }

    long double prob_no_3plus = expl(max_ln_px) * sum_scaled;
    long double answer = 1.0L - prob_no_3plus;

    printf("%.10Lf\n", answer);
    return 0;
}
