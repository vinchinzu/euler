/*
 * Project Euler Problem 609: pi sequences
 *
 * For each starting value, compute iterated prime-counting function,
 * count non-prime elements, accumulate into bins, then multiply.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

#define NLIMIT 100000000
#define MOD 1000000007LL

int main(void) {
    /* Sieve of Eratosthenes */
    char *is_prime = (char*)malloc(NLIMIT + 1);
    memset(is_prime, 1, NLIMIT + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= NLIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= NLIMIT; j += i)
                is_prime[j] = 0;
        }
    }

    /* Compute prime counting function */
    int *pi = (int*)malloc((NLIMIT + 1) * sizeof(int));
    int count = 0;
    for (int i = 0; i <= NLIMIT; i++) {
        if (is_prime[i]) count++;
        pi[i] = count;
    }

    /* Find max chain length */
    int max_len = 0;
    int n = NLIMIT;
    while (n > 0) {
        max_len++;
        n = pi[n];
    }

    ll *ps = (ll*)calloc(max_len + 2, sizeof(ll));

    /* Collect primes */
    int *primes = (int*)malloc((count + 1) * sizeof(int));
    int nprimes = 0;
    for (int i = 2; i <= NLIMIT; i++) {
        if (is_prime[i]) primes[nprimes++] = i;
    }

    for (int idx = 0; idx < nprimes; idx++) {
        int p = primes[idx];
        /* count = number of integers from p to next_prime-1 (or N) that start at p */
        int cnt = (idx == nprimes - 1 ? NLIMIT : primes[idx + 1] - 1) - p;

        n = pi[p];
        int c = 0;
        while (n > 0) {
            if (!is_prime[n]) c++;
            ps[c] += 1;
            ps[c + 1] += cnt;
            n = pi[n];
        }
    }

    ll ans = 1;
    for (int i = 0; i < max_len + 2; i++) {
        if (ps[i] != 0) {
            ans = (i128)ans * (ps[i] % MOD) % MOD;
        }
    }

    printf("%lld\n", ans);

    free(is_prime);
    free(pi);
    free(ps);
    free(primes);
    return 0;
}
