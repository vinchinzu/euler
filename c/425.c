/*
 * Project Euler 425 - Prime connection
 *
 * A prime P is 2's relative if there's a chain of connected primes from 2 to P,
 * none exceeding P. Uses Union-Find. Translated from python/425.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 10000000

static char is_prime[N + 1];
static int parent[N + 1];
static int sz[N + 1];

int find(int x) {
    while (parent[x] != x) {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    return x;
}

void unite(int a, int b) {
    int ra = find(a), rb = find(b);
    if (ra == rb) return;
    if (sz[ra] < sz[rb]) { int t = ra; ra = rb; rb = t; }
    parent[rb] = ra;
    sz[ra] += sz[rb];
}

int main(void) {
    /* Sieve */
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i <= N; i++)
        if (is_prime[i])
            for (long long j = (long long)i * i; j <= N; j += i)
                is_prime[j] = 0;

    /* Collect primes */
    int *primes = malloc((N + 1) * sizeof(int));
    int n_primes = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime[i])
            primes[n_primes++] = i;

    /* Init union-find */
    for (int i = 0; i <= N; i++) { parent[i] = i; sz[i] = 1; }

    /* Precompute powers of 10 */
    int pow_10[8];
    pow_10[0] = 1;
    for (int i = 1; i < 8; i++) pow_10[i] = pow_10[i-1] * 10;

    /* Get digits of a number */
    long long ans = 0;
    for (int pi = 0; pi < n_primes; pi++) {
        int p = primes[pi];

        /* Get digits */
        int digits[8], n_digits = 0;
        int tmp = p;
        while (tmp > 0) { digits[n_digits++] = tmp % 10; tmp /= 10; }
        /* digits are in reverse order: digits[0] = least significant */

        for (int i = 0; i < n_digits; i++) {
            int d = digits[i]; /* digit at position i (from right) */
            for (int sub = 1; sub <= d; sub++) {
                int relative = p - sub * pow_10[i];
                /* Check that leading digit isn't zero */
                if (relative >= pow_10[n_digits - 1] / 10 && relative >= 2 && is_prime[relative]) {
                    unite(p, relative);
                }
            }
        }

        if (find(2) != find(p))
            ans += p;
    }

    printf("%lld\n", ans);
    free(primes);
    return 0;
}
