/*
 * Project Euler Problem 323 - Bitwise-OR operations on random integers
 *
 * Expected number of steps for cumulative OR of random 32-bit ints to reach 2^32-1.
 * DP on number of unset bits with binomial coefficients.
 */
#include <stdio.h>

#define BITS 32

int main(void) {
    /* Precompute binomial coefficients C(n,k) for n up to 32 */
    long long binom[BITS + 1][BITS + 1];
    for (int n = 0; n <= BITS; n++) {
        binom[n][0] = 1;
        for (int k = 1; k <= n; k++)
            binom[n][k] = binom[n - 1][k - 1] + binom[n - 1][k];
        for (int k = n + 1; k <= BITS; k++)
            binom[n][k] = 0;
    }

    /* expected[k] = expected steps to go from k unset bits to 0 */
    double expected[BITS + 1];
    expected[0] = 0.0;

    for (int k = 1; k <= BITS; k++) {
        double inv_two_k = 1.0 / (double)(1LL << k);
        double p_stay = inv_two_k; /* probability all k bits stay unset */

        double sum_transitions = 0.0;
        for (int j = 1; j < k; j++) {
            double p_j = (double)binom[k][j] * inv_two_k;
            sum_transitions += p_j * expected[k - j];
        }

        expected[k] = (1.0 + sum_transitions) / (1.0 - p_stay);
    }

    printf("%.10f\n", expected[BITS]);
    return 0;
}
