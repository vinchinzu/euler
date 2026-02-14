/* Project Euler 077 - Prime summations */
#include <stdio.h>
#include <stdbool.h>

#define N_LIMIT 100
#define TARGET_WAYS 5000

int main(void) {
    /* Generate primes using sieve */
    bool is_prime[N_LIMIT + 1];
    for (int i = 0; i <= N_LIMIT; i++) is_prime[i] = true;
    is_prime[0] = is_prime[1] = false;
    for (int i = 2; i * i <= N_LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N_LIMIT; j += i) {
                is_prime[j] = false;
            }
        }
    }

    int primes[30];
    int nprimes = 0;
    for (int i = 2; i <= N_LIMIT; i++) {
        if (is_prime[i]) primes[nprimes++] = i;
    }

    /* Dynamic programming */
    long long ways[N_LIMIT + 1];
    for (int i = 0; i <= N_LIMIT; i++) ways[i] = 0;
    ways[0] = 1;

    for (int pi = 0; pi < nprimes; pi++) {
        int p = primes[pi];
        for (int s = p; s <= N_LIMIT; s++) {
            ways[s] += ways[s - p];
        }
    }

    for (int i = 2; i <= N_LIMIT; i++) {
        if (ways[i] > TARGET_WAYS) {
            printf("%d\n", i);
            return 0;
        }
    }

    return 1;
}
