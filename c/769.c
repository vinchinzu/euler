/*
 * Project Euler 769 - Binary Quadratic Form
 *
 * Find number of representations of z^2 as x^2+5xy+3y^2 with z <= N.
 * Uses Mobius function and enumeration of (g, h, n, m) tuples.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdint.h>

#define N 100000000000000LL  /* 10^14 */

int* mobius;
int64_t sqrt_N;

void pre_mobius(int64_t limit) {
    mobius = (int*)calloc(limit + 1, sizeof(int));
    char* is_prime = (char*)calloc(limit + 1, 1);

    for (int64_t i = 0; i <= limit; i++) {
        mobius[i] = 1;
        is_prime[i] = 1;
    }
    is_prime[0] = is_prime[1] = 0;

    for (int64_t i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int64_t j = i; j <= limit; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((j / i) % i == 0)
                    mobius[j] = 0;
                else
                    mobius[j] = -mobius[j];
            }
        }
    }
    free(is_prime);
}

int main() {
    sqrt_N = (int64_t)sqrt((double)N);
    pre_mobius(sqrt_N);

    int64_t ans = 0;
    double sqrt3 = sqrt(3.0);

    for (int64_t g = 1; g * g <= N; g++) {
        int64_t g_sq = g * g;
        for (int h_idx = 0; h_idx < 2; h_idx++) {
            int64_t h = (h_idx == 0) ? 1 : 13;
            for (int64_t n = 1; g_sq * n * n <= h * N; n++) {
                double term1 = (double)n / sqrt3;
                double inner = 13.0 * (double)(n * n) + 12.0 * (double)(h * N) / (double)g_sq;
                double term2 = (sqrt(inner) - 5.0 * n) / 6.0;
                int64_t max_m = (int64_t)(term1 < term2 ? term1 : term2);

                if ((g % 13 == 0) == (h == 13))
                    ans += mobius[g] * max_m;

                if (g % 13 != 0) {
                    int64_t sign = (h == 1) ? -1 : 1;
                    ans += sign * mobius[g] * (max_m + (3 * n) % 13) / 13;
                }
            }
        }
    }

    free(mobius);
    printf("%lld\n", ans);
    return 0;
}
