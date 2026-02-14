/*
 * Project Euler Problem 802: Iterated Composition.
 *
 * S = sum_{d=1}^N mu(d) * 2^floor(N/d) mod M
 * Uses linear sieve for Mobius function with N=10^7.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

static ll power(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    const int N = 10000000;
    const ll M = 1020340567LL;

    /* Sieve Mobius function */
    signed char *mu = (signed char *)malloc((N+1) * sizeof(signed char));
    int *smallest_prime = (int *)calloc(N+1, sizeof(int));
    if (!mu || !smallest_prime) { fprintf(stderr, "malloc failed\n"); return 1; }

    for (int i = 0; i <= N; i++) mu[i] = 1;
    mu[0] = 0;

    /* Linear sieve for Mobius */
    int *primes = (int *)malloc((N+1) * sizeof(int));
    int prime_count = 0;

    for (int i = 2; i <= N; i++) {
        if (smallest_prime[i] == 0) {
            smallest_prime[i] = i;
            primes[prime_count++] = i;
            mu[i] = -1;
        }
        for (int j = 0; j < prime_count && primes[j] <= smallest_prime[i] && (ll)i * primes[j] <= N; j++) {
            int v = i * primes[j];
            smallest_prime[v] = primes[j];
            if (i % primes[j] == 0) {
                mu[v] = 0;
            } else {
                mu[v] = -mu[i];
            }
        }
    }

    /* Compute S = sum_{d=1}^N mu(d) * 2^floor(N/d) mod M */
    ll ans = 0;
    for (int d = 1; d <= N; d++) {
        if (mu[d] == 0) continue;
        ll p = power(2, N / d, M);
        if (mu[d] == 1)
            ans = (ans + p) % M;
        else
            ans = (ans - p + M) % M;
    }

    printf("%lld\n", ans);

    free(mu);
    free(smallest_prime);
    free(primes);
    return 0;
}
