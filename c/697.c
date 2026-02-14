/*
 * Project Euler 697 - Randomly Decaying Sequence
 *
 * Binary search for log(c) such that the product of N uniform random
 * variables scaled by c has 25% chance of being < 1.
 *
 * Uses log-space arithmetic to avoid overflow.
 */
#include <stdio.h>
#include <math.h>

#define N 10000000

static double log_facts[N + 1];

static double log_sum(double log_a, double log_b) {
    if (log_a == -HUGE_VAL) return log_b;
    if (log_b == -HUGE_VAL) return log_a;
    if (log_a > log_b)
        return log_a + log1p(exp(log_b - log_a));
    return log_b + log1p(exp(log_a - log_b));
}

int main(void) {
    double R = 0.25;

    /* Precompute log factorials */
    log_facts[0] = 0.0;
    for (int i = 1; i <= N; i++)
        log_facts[i] = log_facts[i - 1] + log((double)i);

    double low = 0.0;
    double high = 2.0 * N;

    while (low + 1e-3 < high) {
        double log_c = (low + high) / 2.0;
        double log_prob = -HUGE_VAL;
        double log_log_c = log(log_c);

        for (int k = N - 1; k >= 0; k--) {
            double new_log_prob = log_sum(
                log_prob, k * log_log_c - log_facts[k]
            );
            if (fabs(log_prob - new_log_prob) < 1e-15 && k < N - 10)
                break;
            log_prob = new_log_prob;
        }

        if (log_prob > log(R) + log_c)
            low = log_c;
        else
            high = log_c;
    }

    double ans = low * log10(M_E);
    printf("%.2f\n", ans);
    return 0;
}
