/*
 * Project Euler Problem 573: Unfair Race
 *
 * E_N = sum_{k=1}^{N} C(N,k) * k^k * (N-k)^(N-k) / N^N
 * Compute using log-space to avoid overflow.
 * N = 1000000.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void) {
    int N = 1000000;

    /* Precompute log factorials */
    double *log_fact = (double *)malloc((N + 1) * sizeof(double));
    log_fact[0] = 0.0;
    for (int i = 1; i <= N; i++)
        log_fact[i] = log_fact[i - 1] + log((double)i);

    double ans = 1.0;  /* k = N case: C(N,N)*N^N*0^0/N^N = 1 */

    for (int k = 1; k < N; k++) {
        double log_term = log_fact[N] - log_fact[k] - log_fact[N - k]
                          + (double)k * log((double)k)
                          + (double)(N - k) * log((double)(N - k))
                          - (double)N * log((double)N);
        ans += exp(log_term);
    }

    printf("%.4f\n", ans);

    free(log_fact);
    return 0;
}
