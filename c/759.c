/* Project Euler Problem 759: A Squared Recurrence.
 * Translated from python/759.py
 *
 * f(n) = n * popcount(n). Compute sum_{n=1}^N f(n)^2 = sum n^2 * popcount(n)^2.
 * Using the recurrence S(n,K,L) with memoization.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define MAX_K 2
#define MAX_L 2

/* Hash map for memoization of S(n, K, L) */
/* Key = n, since K and L are small (0..2), we have 9 separate hash maps */

#define HASH_SIZE (1 << 18)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct {
    ll key;
    ll val;
    int used;
} Entry;

static Entry maps[3][3][HASH_SIZE];

int hm_get(int K, int L, ll n, ll *val) {
    unsigned h = (unsigned)((n ^ (n >> 17)) * 0x9e3779b97f4a7c15ULL) & HASH_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!maps[K][L][idx].used) return 0;
        if (maps[K][L][idx].key == n) {
            *val = maps[K][L][idx].val;
            return 1;
        }
    }
    return 0;
}

void hm_put(int K, int L, ll n, ll val) {
    unsigned h = (unsigned)((n ^ (n >> 17)) * 0x9e3779b97f4a7c15ULL) & HASH_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!maps[K][L][idx].used) {
            maps[K][L][idx].key = n;
            maps[K][L][idx].val = val;
            maps[K][L][idx].used = 1;
            return;
        }
    }
}

/* Precomputed binomial coefficients (small) */
int nCr[3][3];

/* Precomputed 2^k mod MOD for k=0..2 */
ll pow2[3];

void init(void) {
    for (int n = 0; n <= 2; n++)
        for (int k = 0; k <= 2; k++) {
            if (k > n) nCr[n][k] = 0;
            else if (k == 0 || k == n) nCr[n][k] = 1;
            else nCr[n][k] = nCr[n-1][k-1] + nCr[n-1][k];
        }
    pow2[0] = 1;
    pow2[1] = 2;
    pow2[2] = 4;
}

ll S(ll n, int K, int L) {
    if (n == 0) {
        return (K == 0 && L == 0) ? 1 : 0;
    }

    ll cached;
    if (hm_get(K, L, n, &cached))
        return cached;

    /* result = 2^K * S(n/2, K, L) */
    ll result = pow2[K] % MOD * S(n / 2, K, L) % MOD;

    /* + sum_{k=0}^K sum_{l=0}^L 2^k * C(K,k) * C(L,l) * S((n-1)/2, k, l) */
    for (int k = 0; k <= K; k++) {
        for (int l = 0; l <= L; l++) {
            ll term = pow2[k] * nCr[K][k] % MOD * nCr[L][l] % MOD;
            term = term * S((n - 1) / 2, k, l) % MOD;
            result = (result + term) % MOD;
        }
    }

    hm_put(K, L, n, result);
    return result;
}

int main() {
    ll N = 10000000000000000LL; /* 10^16 */

    init();
    memset(maps, 0, sizeof(maps));

    printf("%lld\n", S(N, 2, 2));
    return 0;
}
