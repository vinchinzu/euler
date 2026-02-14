/*
 * Project Euler Problem 318: Leading Digits of Power Sums
 *
 * For pairs (p,q) with p < q, p+q <= 2011, sqrt(q)-sqrt(p) < 1,
 * find N(p,q) = minimal n such that fractional part of (sqrt(p)+sqrt(q))^(2n)
 * has >= 2011 leading nines.
 *
 * N(p,q) = ceil(2011 / (-2 * log10(sqrt(q) - sqrt(p))))
 *
 * Sum all N(p,q).
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int LIMIT = 2011;
    int EXPONENT = 2011;

    long long total = 0;

    for (int p = 1; p < LIMIT; p++) {
        double sp = sqrt((double)p);
        /* q_max from sqrt(q) - sqrt(p) < 1 => sqrt(q) < sqrt(p)+1 => q < (sqrt(p)+1)^2 */
        int q_max_beta = (int)((sp + 1.0) * (sp + 1.0));
        int q_max_sum = LIMIT - p;
        int q_end = q_max_beta < q_max_sum ? q_max_beta : q_max_sum;

        if (q_end <= p) continue;

        for (int q = p + 1; q <= q_end; q++) {
            double sq = sqrt((double)q);
            double beta = sq - sp;
            if (beta >= 1.0) continue;

            /* N = ceil(EXPONENT / (-2 * log10(beta))) */
            double log_beta = log10(beta);
            double denom = -2.0 * log_beta;
            if (denom <= 0.0) continue;

            double n_val = (double)EXPONENT / denom;
            long long n = (long long)ceil(n_val);
            if (n < 1) n = 1;
            total += n;
        }
    }

    printf("%lld\n", total);
    return 0;
}
