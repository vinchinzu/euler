/*
 * Project Euler 632: Square prime factors
 *
 * C_k(N) = number of integers 1..N divisible by p^2 for exactly k primes p.
 * Find product of all nonzero C_k(N) mod 10^9+7.
 *
 * N = 10^16, sieve Mobius/omega up to sqrt(N) = 10^8.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define N 10000000000000000LL
#define M 1000000007LL
#define L 100000000

int8_t *mobius;
int8_t *omega;

void sieve() {
    mobius = (int8_t*)malloc((L + 1) * sizeof(int8_t));
    omega = (int8_t*)malloc((L + 1) * sizeof(int8_t));

    for (int i = 0; i <= L; i++) {
        mobius[i] = 1;
        omega[i] = 0;
    }

    char *is_prime = (char*)malloc((L + 1) * sizeof(char));
    memset(is_prime, 1, L + 1);

    for (int i = 2; i <= L; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= L; j += i) {
                is_prime[j] = 0;
                mobius[j] *= -1;
                omega[j]++;
            }
            for (ll j = (ll)i * i; j <= L; j += (ll)i * i) {
                mobius[j] = 0;
            }
        }
    }

    free(is_prime);
}

int ilog2(int n) {
    int result = 0;
    while (n > 1) {
        n /= 2;
        result++;
    }
    return result;
}

int main() {
    sieve();

    int maxK = ilog2(L);

    ll **nCr = (ll**)malloc((maxK + 1) * sizeof(ll*));
    for (int i = 0; i <= maxK; i++) {
        nCr[i] = (ll*)calloc(maxK + 1, sizeof(ll));
        nCr[i][0] = 1;
        for (int j = 1; j <= i; j++) {
            nCr[i][j] = (nCr[i-1][j-1] + nCr[i-1][j]) % M;
        }
    }

    ll *C = (ll*)calloc(maxK + 1, sizeof(ll));
    C[0] = N % M;

    for (int n = 2; n <= L; n++) {
        if (mobius[n] == 0) continue;
        int k = omega[n];
        ll count = (N / ((ll)n * n)) % M;

        for (int i = 0; i <= k; i++) {
            int parity = (i % 2 == 0) ? 1 : -1;
            C[k - i] = (C[k - i] + parity * (lll)nCr[k][i] * count % M + M) % M;
        }
    }

    ll ans = 1;
    for (int i = 0; i <= maxK; i++) {
        if (C[i] != 0) {
            ans = (lll)ans * C[i] % M;
        }
    }

    printf("%lld\n", ans);

    free(mobius);
    free(omega);
    for (int i = 0; i <= maxK; i++) free(nCr[i]);
    free(nCr);
    free(C);

    return 0;
}
