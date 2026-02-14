/*
 * Project Euler 441 - The inverse summation of coprime pairs
 *
 * Compute sum_{i=2}^N R(i), where R(i) involves coprime pairs.
 * Formula: ans = (sum_{g=1}^N mu(g) * (H(N/g)/g)^2 + N - 3) / 2
 * where H(k) is the k-th harmonic number.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000000

static int mu[N + 1];
static double harmonics[N + 1];

int main(void) {
    /* Sieve Mobius function */
    static char is_prime[N + 1];
    memset(is_prime, 1, N + 1);
    is_prime[0] = is_prime[1] = 0;

    for (int i = 0; i <= N; i++) mu[i] = 1;

    for (int i = 2; i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= N; j += i) {
                is_prime[j] = 0;
                mu[j] = -mu[j];
            }
            for (long long j = (long long)i * i; j <= N; j += (long long)i * i) {
                mu[j] = 0;
            }
        }
    }

    /* Compute harmonic numbers */
    harmonics[0] = 0.0;
    for (int i = 1; i <= N; i++) {
        harmonics[i] = harmonics[i - 1] + 1.0 / i;
    }

    /* Main computation */
    double ans = 0.0;
    for (int g = 1; g <= N; g++) {
        if (mu[g] != 0) {
            double h_val = harmonics[N / g] / g;
            ans += mu[g] * h_val * h_val;
        }
    }

    ans = (ans + N - 3) / 2.0;
    printf("%.4f\n", ans);
    return 0;
}
