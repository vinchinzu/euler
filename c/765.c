/*
 * Project Euler 765 - Gambling Probability
 *
 * Find probability of reaching K gold in N rounds with optimal play.
 * Uses log-space arithmetic to handle large binomial coefficients.
 */
#include <stdio.h>
#include <math.h>
#include <float.h>

static double log_factorials[1001];

static void precompute_log_fact(int n) {
    log_factorials[0] = 0.0;
    for (int i = 1; i <= n; i++)
        log_factorials[i] = log_factorials[i - 1] + log((double)i);
}

static double log_sum(double log_x, double log_y) {
    if (log_x == -INFINITY) return log_y;
    if (log_y == -INFINITY) return log_x;
    if (log_x > log_y)
        return log_x + log(1.0 + exp(log_y - log_x));
    else
        return log_y + log(1.0 + exp(log_x - log_y));
}

static double log_diff(double log_x, double log_y) {
    return log_x + log(1.0 - exp(log_y - log_x));
}

int main(void) {
    int N = 1000;
    long long K = 1000000000000LL; /* 10^12 */
    double P = 0.6;

    precompute_log_fact(N);
    double max_count = N * log(2.0) - log((double)K);
    double ans = -INFINITY;

    int k = 0;
    while (1) {
        double count = log_factorials[N] - log_factorials[k] - log_factorials[N - k];
        double term = fmin(count, max_count) + (N - k) * log(P) + k * log(1.0 - P);
        ans = log_sum(ans, term);
        if (count > max_count) {
            ans = exp(ans);
            break;
        }
        max_count = log_diff(max_count, count);
        k++;
    }

    printf("%.10f\n", ans);
    return 0;
}
