/* Project Euler 187: Semiprimes. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 100000000

int main(void) {
    /* We need primes up to LIMIT/2 for the second factor */
    int sieve_limit = LIMIT / 2 + 1;

    /* Sieve of Eratosthenes */
    char *sieve = calloc(sieve_limit + 1, 1);
    if (!sieve) return 1;
    sieve[0] = sieve[1] = 1;
    for (int i = 2; (long long)i * i <= sieve_limit; i++) {
        if (!sieve[i]) {
            for (int j = i * i; j <= sieve_limit; j += i)
                sieve[j] = 1;
        }
    }

    /* Collect primes */
    int *primes = malloc(sizeof(int) * (sieve_limit / 2 + 100));
    int nprimes = 0;
    for (int i = 2; i <= sieve_limit; i++) {
        if (!sieve[i]) primes[nprimes++] = i;
    }

    /* Count semiprimes: p * q < LIMIT where p <= q and both prime */
    long long count = 0;
    for (int i = 0; i < nprimes; i++) {
        long long p = primes[i];
        if (p * p >= LIMIT) break;
        long long max_q = (LIMIT - 1) / p;
        /* Binary search for largest prime <= max_q */
        int lo = i, hi = nprimes - 1, best = -1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (primes[mid] <= max_q) {
                best = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        if (best >= i) {
            count += best - i + 1;
        }
    }

    printf("%lld\n", count);
    free(sieve);
    free(primes);
    return 0;
}
