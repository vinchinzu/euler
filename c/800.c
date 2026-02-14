/*
 * Project Euler 800 - Hybrid Integers
 *
 * Count pairs (p,q) with p<q both prime where p*log(q) + q*log(p) < N*log(N),
 * with N = 800800.
 */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

int main(void) {
    int N = 800800;
    int max_prime = N * 21;

    char *is_prime = (char *)malloc((size_t)(max_prime + 1));
    if (!is_prime) return 1;
    memset(is_prime, 1, (size_t)(max_prime + 1));
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; (long long)i * i <= max_prime; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= max_prime; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    int num_primes = 0;
    for (int i = 2; i <= max_prime; i++) {
        if (is_prime[i]) num_primes++;
    }

    int *primes = (int *)malloc((size_t)num_primes * sizeof(int));
    if (!primes) return 1;
    int idx = 0;
    for (int i = 2; i <= max_prime; i++) {
        if (is_prime[i]) primes[idx++] = i;
    }
    free(is_prime);

    double *log_primes = (double *)malloc((size_t)num_primes * sizeof(double));
    if (!log_primes) return 1;
    for (int i = 0; i < num_primes; i++) {
        log_primes[i] = log((double)primes[i]);
    }

    double limit = (double)N * log((double)N);

    long long ans = 0;
    for (int i = 0; i < num_primes; i++) {
        int p = primes[i];
        double lp = log_primes[i];

        int low = i;
        int high = num_primes;
        while (low + 1 < high) {
            int mid = (low + high) / 2;
            double val = (double)p * log_primes[mid] + (double)primes[mid] * lp;
            if (val < limit) {
                low = mid;
            } else {
                high = mid;
            }
        }
        ans += low - i;
    }

    printf("%lld\n", ans);

    free(primes);
    free(log_primes);
    return 0;
}
