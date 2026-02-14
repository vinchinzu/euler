/*
 * Project Euler 664 - An Infinite Game of Tag
 *
 * Conway's soldiers variant. Sum score = sum_{d=1}^inf d^N * phi^(4-d).
 * Answer = floor(log_phi(total_score)).
 */
#include <stdio.h>
#include <math.h>

static double log_sum(double log_a, double log_b) {
    if (log_a == -INFINITY) return log_b;
    if (log_b == -INFINITY) return log_a;
    if (log_a >= log_b)
        return log_a + log1p(exp(log_b - log_a));
    else
        return log_b + log1p(exp(log_a - log_b));
}

int main() {
    int N = 1234567;
    double PHI = (1.0 + sqrt(5.0)) / 2.0;
    double log_phi = log(PHI);

    double log_sum_val = -INFINITY;
    int d = 1;
    while (1) {
        double log_val = (double)N * log((double)d) - (double)(d - 4) * log_phi;
        double new_log_sum = log_sum(log_sum_val, log_val);
        if (fabs(log_sum_val - new_log_sum) < 1e-10)
            break;
        log_sum_val = new_log_sum;
        d++;
    }

    log_sum_val /= log_phi;
    printf("%lld\n", (long long)log_sum_val);
    return 0;
}
