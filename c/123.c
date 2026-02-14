/* Project Euler Problem 123: Prime square remainders.
 *
 * For odd n: remainder = 2*n*p_n mod p_n^2 = 2*n*p_n (when 2n < p_n).
 * Find smallest odd n such that 2*n*p_n > 10^10.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define SIEVE_LIMIT 300000

int main(void) {
    /* Sieve of Eratosthenes */
    bool *is_prime = calloc(SIEVE_LIMIT + 1, sizeof(bool));
    if (!is_prime) return 1;

    for (int i = 2; i <= SIEVE_LIMIT; i++) is_prime[i] = true;
    for (int i = 2; (long long)i * i <= SIEVE_LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= SIEVE_LIMIT; j += i)
                is_prime[j] = false;
        }
    }

    /* Collect primes */
    int *primes = malloc(30000 * sizeof(int));
    int nprimes = 0;
    for (int i = 2; i <= SIEVE_LIMIT; i++) {
        if (is_prime[i]) primes[nprimes++] = i;
    }

    long long target = 10000000000LL; /* 10^10 */

    for (int n = 1; n <= nprimes; n += 2) { /* odd n only */
        long long p_n = primes[n - 1]; /* 1-indexed */
        long long remainder = 2LL * n * p_n;
        if (remainder > target) {
            printf("%d\n", n);
            free(is_prime);
            free(primes);
            return 0;
        }
    }

    free(is_prime);
    free(primes);
    return 1;
}
