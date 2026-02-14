/* Project Euler 193: Squarefree Numbers.
   Count squarefree numbers below 2^50 using Mobius function. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int main(void) {
    long long LIMIT = 1LL << 50;
    int sqrt_limit = (int)sqrt((double)LIMIT);
    /* Make sure sqrt_limit^2 <= LIMIT */
    while ((long long)(sqrt_limit + 1) * (sqrt_limit + 1) <= LIMIT) sqrt_limit++;
    while ((long long)sqrt_limit * sqrt_limit > LIMIT) sqrt_limit--;

    /* Mobius sieve up to sqrt_limit using linear sieve */
    signed char *mu = calloc(sqrt_limit + 1, sizeof(signed char));
    char *composite = calloc(sqrt_limit + 1, 1);
    int *primes = malloc(sizeof(int) * (sqrt_limit / 2 + 100));
    int nprimes = 0;

    mu[1] = 1;
    for (int i = 2; i <= sqrt_limit; i++) {
        if (!composite[i]) {
            primes[nprimes++] = i;
            mu[i] = -1;
        }
        for (int j = 0; j < nprimes; j++) {
            long long ip = (long long)i * primes[j];
            if (ip > sqrt_limit) break;
            composite[(int)ip] = 1;
            if (i % primes[j] == 0) {
                mu[(int)ip] = 0;
                break;
            } else {
                mu[(int)ip] = -mu[i];
            }
        }
    }

    long long total = 0;
    for (int d = 1; d <= sqrt_limit; d++) {
        if (mu[d] == 0) continue;
        total += mu[d] * (LIMIT / ((long long)d * d));
    }

    printf("%lld\n", total);
    free(mu);
    free(composite);
    free(primes);
    return 0;
}
