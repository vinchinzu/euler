/*
 * Project Euler Problem 313: Sliding game
 *
 * Count grids (m,n) where S(m,n) = p^2 for primes p < 10^6.
 * For each odd prime p, S = p^2, t = (p^2+13)/2, count integer a in [t/4+1, (t-2)/3].
 * Each such a gives 2 grids (m,n) and (n,m).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAX_PRIME 1000000

int main(void) {
    /* Sieve primes up to MAX_PRIME */
    char *sieve = malloc(MAX_PRIME);
    memset(sieve, 1, MAX_PRIME);
    sieve[0] = sieve[1] = 0;
    int sq = (int)sqrt((double)MAX_PRIME);
    for (int i = 2; i <= sq; i++) {
        if (sieve[i]) {
            for (int j = i * i; j < MAX_PRIME; j += i)
                sieve[j] = 0;
        }
    }

    long long total = 0;
    for (int p = 3; p < MAX_PRIME; p++) {
        if (!sieve[p]) continue;
        long long sq = (long long)p * p;
        long long t = (sq + 13) / 2;
        long long a_min = t / 4 + 1;
        long long a_max = (t - 2) / 3;
        if (a_max >= a_min)
            total += 2 * (a_max - a_min + 1);
    }

    printf("%lld\n", total);
    free(sieve);
    return 0;
}
