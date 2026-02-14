/*
 * Project Euler 214: Totient Chains
 *
 * Find sum of all primes p < 40,000,000 such that the totient chain
 * p -> phi(p) -> phi(phi(p)) -> ... -> 1 has exactly 25 terms.
 */
#include <stdio.h>
#include <stdlib.h>

#define LIMIT 40000000

int main(void) {
    /* Euler's totient via sieve */
    int *phi = malloc((LIMIT + 1) * sizeof(int));
    for (int i = 0; i <= LIMIT; i++) phi[i] = i;
    for (int i = 2; i <= LIMIT; i++) {
        if (phi[i] == i) {  /* i is prime */
            for (int j = i; j <= LIMIT; j += i)
                phi[j] = phi[j] / i * (i - 1);
        }
    }

    /* Chain lengths */
    int *chain = malloc(LIMIT * sizeof(int));
    chain[0] = 0;
    chain[1] = 1;

    long long ans = 0;
    for (int i = 2; i < LIMIT; i++) {
        chain[i] = chain[phi[i]] + 1;
        if (phi[i] == i - 1 && chain[i] == 25)  /* i is prime and chain length is 25 */
            ans += i;
    }

    printf("%lld\n", ans);
    free(phi);
    free(chain);
    return 0;
}
