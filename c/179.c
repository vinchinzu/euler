/*
 * Project Euler Problem 179: Consecutive positive divisors
 *
 * Count n in [2, 10^7-1] where tau(n) == tau(n+1).
 * Uses linear sieve to compute tau (number of divisors).
 */
#include <stdio.h>
#include <stdlib.h>

#define LIMIT 10000000

int main(void) {
    int *spf = calloc(LIMIT + 1, sizeof(int));     /* smallest prime factor */
    int *tau = malloc((LIMIT + 1) * sizeof(int));   /* number of divisors */
    int *exponent = calloc(LIMIT + 1, sizeof(int)); /* exponent of spf in n */
    int *primes = malloc((LIMIT + 1) * sizeof(int));
    int nprimes = 0;

    for (int i = 0; i <= LIMIT; i++) tau[i] = 1;

    for (int i = 2; i <= LIMIT; i++) {
        if (spf[i] == 0) {
            spf[i] = i;
            primes[nprimes++] = i;
            tau[i] = 2;
            exponent[i] = 1;
        }

        for (int j = 0; j < nprimes; j++) {
            int p = primes[j];
            long long pi = (long long)p * i;
            if (pi > LIMIT) break;
            int idx = (int)pi;
            spf[idx] = p;

            if (p == spf[i]) {
                exponent[idx] = exponent[i] + 1;
                tau[idx] = tau[i] / (exponent[i] + 1) * (exponent[idx] + 1);
                break;
            } else {
                exponent[idx] = 1;
                tau[idx] = tau[i] * 2;
            }
        }
    }

    int count = 0;
    for (int n = 2; n < LIMIT; n++) {
        if (tau[n] == tau[n + 1])
            count++;
    }

    printf("%d\n", count);

    free(spf); free(tau); free(exponent); free(primes);
    return 0;
}
