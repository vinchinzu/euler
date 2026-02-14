/* Project Euler 351 - Hexagonal Orchards
 *
 * Hidden = 3*n*(n+1) - 6*sum(phi(k) for k=1..n), n=10^8.
 * Compute Euler's totient function via sieve.
 */

#include <stdio.h>
#include <stdlib.h>

#define LIMIT 100000000

int main(void) {
    long long n = LIMIT;

    /* Allocate phi array */
    int *phi = (int*)malloc((n + 1) * sizeof(int));
    if (!phi) return 1;

    /* Initialize phi[i] = i */
    for (long long i = 0; i <= n; i++)
        phi[i] = (int)i;

    /* Sieve to compute totient */
    for (long long p = 2; p <= n; p++) {
        if (phi[p] == (int)p) {
            /* p is prime */
            phi[p] = (int)(p - 1);
            for (long long j = 2 * p; j <= n; j += p) {
                phi[j] -= phi[j] / (int)p;
            }
        }
    }

    /* Sum phi */
    long long totient_sum = 0;
    for (long long i = 1; i <= n; i++)
        totient_sum += phi[i];

    long long hidden = 3LL * n * (n + 1) - 6LL * totient_sum;
    printf("%lld\n", hidden);

    free(phi);
    return 0;
}
