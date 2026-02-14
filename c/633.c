/*
 * Project Euler 633: Square prime factors II
 *
 * Find the limit of C_K(N)/N as N -> infinity for K=7.
 * DP over primes: c[k+1] += c[k]/p^2; c[k] *= (1-1/p^2).
 * Double primes until convergence.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define K_VAL 7

/* Simple prime sieve */
int *sieve_primes(int limit, int *count) {
    char *is_p = calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_p[i] = 1;
    for (int i = 2; (long long)i * i <= limit; i++)
        if (is_p[i])
            for (int j = i * i; j <= limit; j += i)
                is_p[j] = 0;
    *count = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) (*count)++;
    int *primes = malloc(*count * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) primes[idx++] = i;
    free(is_p);
    return primes;
}

int main() {
    double ans = 0.0;
    int num_primes_limit = K_VAL;

    while (1) {
        double c[K_VAL + 2];
        memset(c, 0, sizeof(c));
        c[0] = 1.0;

        int nprimes;
        int *primes = sieve_primes(num_primes_limit, &nprimes);

        for (int pi = 0; pi < nprimes; pi++) {
            double p_sq = (double)primes[pi] * primes[pi];
            for (int k = K_VAL; k >= 0; k--) {
                c[k + 1] += c[k] / p_sq;
                c[k] *= 1.0 - 1.0 / p_sq;
            }
        }
        free(primes);

        if (c[K_VAL] != 0.0 && fabs(ans / c[K_VAL] - 1.0) < 1e-5) {
            ans = c[K_VAL];
            break;
        }
        ans = c[K_VAL];
        num_primes_limit *= 2;
    }

    printf("%.4e\n", ans);
    return 0;
}
