/*
 * Project Euler 779 - Prime Factor and Order
 *
 * Compute sum over primes p of 1/(p*(p-1)^2) * product_{q<p} (1-1/q).
 * Uses sieve of Eratosthenes for primes up to 100000.
 */
#include <stdio.h>
#include <math.h>

#define LIMIT 100000

static char is_prime[LIMIT + 1];
static int primes[10000];
static int nprimes;

static void sieve(void) {
    for (int i = 0; i <= LIMIT; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i <= LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= LIMIT; j += i)
                is_prime[j] = 0;
        }
    }
    nprimes = 0;
    for (int i = 2; i <= LIMIT; i++)
        if (is_prime[i])
            primes[nprimes++] = i;
}

int main(void) {
    sieve();

    double ans = 0.0;
    double prod = 1.0; /* product of (1-1/q) for q < p */

    for (int i = 0; i < nprimes; i++) {
        double p = primes[i];
        double res = prod / (p * (p - 1.0) * (p - 1.0));
        ans += res;
        prod *= (1.0 - 1.0 / p);
    }

    printf("%.12f\n", ans);
    return 0;
}
