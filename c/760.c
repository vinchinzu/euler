/* Project Euler Problem 760: Sum over Bitwise Operators.
 * Translated from python/760.py
 *
 * G(n) = sum_{i=0}^n sum_{k=0}^i g(k, i-k)
 * with recurrence G(n) = 2G(n/2) + 2G(n/2-1) + 2*tr(n/2)
 *                       + 2*(2G(ceil(n/2)-1) + 2*tr(ceil(n/2)))
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

#define HASH_SIZE (1 << 18)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct {
    ll key;
    ll val;
    int used;
} Entry;

static Entry hmap[HASH_SIZE];

int hm_get(ll n, ll *val) {
    unsigned h = (unsigned)((n ^ (n >> 17)) * 0x9e3779b97f4a7c15ULL) & HASH_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!hmap[idx].used) return 0;
        if (hmap[idx].key == n) {
            *val = hmap[idx].val;
            return 1;
        }
    }
    return 0;
}

void hm_put(ll n, ll val) {
    unsigned h = (unsigned)((n ^ (n >> 17)) * 0x9e3779b97f4a7c15ULL) & HASH_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!hmap[idx].used) {
            hmap[idx].key = n;
            hmap[idx].val = val;
            hmap[idx].used = 1;
            return;
        }
    }
}

ll tr(ll n) {
    if (n <= 0) return 0;
    /* n*(n+1)/2 mod MOD */
    ll nm = n % MOD;
    ll np1m = (n + 1) % MOD;
    /* Use modular inverse of 2 */
    ll inv2 = (MOD + 1) / 2;
    return (lll)nm * np1m % MOD * inv2 % MOD;
}

ll G(ll n) {
    if (n <= 0) return 0;

    ll cached;
    if (hm_get(n, &cached))
        return cached;

    ll half = n / 2;
    ll ceil_half = (n + 1) / 2;

    ll result = (
        2 * G(half) % MOD
        + 2 * G(half - 1) % MOD
        + 2 * tr(half) % MOD
        + 2 * (2 * G(ceil_half - 1) % MOD + 2 * tr(ceil_half) % MOD) % MOD
    ) % MOD;

    result = (result + MOD) % MOD;

    hm_put(n, result);
    return result;
}

int main() {
    ll N = 1000000000000000000LL; /* 10^18 */
    memset(hmap, 0, sizeof(hmap));

    printf("%lld\n", G(N));
    return 0;
}
