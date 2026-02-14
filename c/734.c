/* Project Euler 734: A Bit of Prime.
 * OR-convolution via Mobius/Zeta transform on bitwise OR.
 * N=10^6, K=999983, M=10^9+7.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    int N = 1000000;
    int K = 999983;

    /* Find L = smallest power of 2 >= N */
    int L = 1;
    while (L < N) L <<= 1;

    /* Sieve of Eratosthenes */
    char *is_prime = calloc(N + 1, 1);
    for (int i = 2; i <= N; i++) is_prime[i] = 1;
    for (int i = 2; (ll)i * i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }

    /* Initialize array */
    ll *A = calloc(L, sizeof(ll));
    int *primes = NULL;
    int nprimes = 0;
    for (int p = 2; p <= N; p++) {
        if (is_prime[p]) {
            A[p] = 1;
            nprimes++;
        }
    }
    primes = malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int p = 2; p <= N; p++)
        if (is_prime[p]) primes[idx++] = p;

    /* Forward Zeta transform for bitwise OR (subset sum) */
    for (int u = 1; u < L; u <<= 1) {
        for (int x = 0; x < L; x++) {
            if ((x & u) == 0) {
                A[x | u] = (A[x | u] + A[x]) % MOD;
            }
        }
    }

    /* Raise to K-th power */
    for (int i = 0; i < L; i++) {
        A[i] = pow_mod(A[i], K, MOD);
    }

    /* Inverse Mobius transform */
    for (int u = 1; u < L; u <<= 1) {
        for (int x = 0; x < L; x++) {
            if ((x & u) == 0) {
                A[x | u] = (A[x | u] - A[x] % MOD + MOD) % MOD;
            }
        }
    }

    /* Sum over primes */
    ll ans = 0;
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        if (p < L)
            ans = (ans + A[p]) % MOD;
    }

    printf("%lld\n", ans);

    free(A);
    free(is_prime);
    free(primes);
    return 0;
}
