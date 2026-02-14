/*
 * Project Euler 783 - Urns Balls Black White
 *
 * Start with kn white balls. Over n turns: add k black, remove 2k random.
 * Track E[C_t] and E[C_t^2] through hypergeometric recurrence.
 * Uses long double for sufficient precision.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int n = 1000000;
    int k = 10;

    long double mu = 0.0L;   /* E[C_t] */
    long double nu = 0.0L;   /* E[C_t^2] */
    long double total = 0.0L;
    long double kf = (long double)k;
    long double k2 = kf * kf;

    for (int t = 1; t <= n; t++) {
        long double alpha = (long double)(n - t + 2);
        long double M = kf * alpha;

        long double P = mu + kf;           /* E[C_t + k] */
        long double Q = nu + 2.0L * kf * mu + k2;  /* E[(C_t + k)^2] */

        /* E[B_t^2] = 2/[alpha*(M-1)] * [k*(alpha-2)*P + (2k-1)*Q] */
        long double eb2 = 2.0L / (alpha * (M - 1.0L)) *
                          (kf * (alpha - 2.0L) * P + (2.0L * kf - 1.0L) * Q);
        total += eb2;

        /* Update recurrences */
        if (alpha > 2.0L) {
            mu = P * (alpha - 2.0L) / alpha;
            nu = (alpha - 2.0L) / (alpha * (M - 1.0L)) *
                 ((M - 2.0L * kf - 1.0L) * Q + 2.0L * kf * P);
        } else {
            mu = 0.0L;
            nu = 0.0L;
        }
    }

    printf("%lld\n", (long long)roundl(total));
    return 0;
}
