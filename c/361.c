/*
 * Project Euler Problem 361 - Thue-Morse based sequence A_n.
 *
 * Find sum_{k=1}^18 A(10^k) mod 10^9, where A(n) is the nth number (sorted)
 * whose binary representation appears as a contiguous subsequence of the
 * Thue-Morse sequence.
 *
 * Uses recurrences to compute firstIndexWithLen(l), then binary search
 * to find the length of A(n), recursive position finding, and prefix computation.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD 1000000000LL
#define CACHE_SIZE (1 << 20)  /* hash table size for memoization */

/* ---------- Memoization hash tables ---------- */

typedef struct {
    ll key;
    ll value;
    int occupied;
} CacheEntry;

static CacheEntry fiwl_cache[CACHE_SIZE];
static CacheEntry prefix_cache[CACHE_SIZE];

static unsigned int hash_ll(ll key) {
    ull k = (ull)key;
    k ^= k >> 33;
    k *= 0xff51afd7ed558ccdULL;
    k ^= k >> 33;
    k *= 0xc4ceb9fe1a85ec53ULL;
    k ^= k >> 33;
    return (unsigned int)(k & (CACHE_SIZE - 1));
}

static int cache_get(CacheEntry *cache, ll key, ll *value) {
    unsigned int idx = hash_ll(key);
    for (int probe = 0; probe < 256; probe++) {
        unsigned int i = (idx + probe) & (CACHE_SIZE - 1);
        if (!cache[i].occupied) return 0;
        if (cache[i].key == key) { *value = cache[i].value; return 1; }
    }
    return 0;
}

static void cache_put(CacheEntry *cache, ll key, ll value) {
    unsigned int idx = hash_ll(key);
    for (int probe = 0; probe < 256; probe++) {
        unsigned int i = (idx + probe) & (CACHE_SIZE - 1);
        if (!cache[i].occupied || cache[i].key == key) {
            cache[i].key = key;
            cache[i].value = value;
            cache[i].occupied = 1;
            return;
        }
    }
    /* Overwrite first slot if full */
    cache[idx].key = key;
    cache[idx].value = value;
    cache[idx].occupied = 1;
}

/* ---------- Core functions ---------- */

static ll firstIndexWithLen(ll length) {
    if (length <= 3) return 1LL << (length - 1);
    ll cached;
    if (cache_get(fiwl_cache, length, &cached)) return cached;
    ll result;
    if (length % 2 == 0) {
        result = firstIndexWithLen(length / 2) + 3 * firstIndexWithLen(length / 2 + 1) - 7;
    } else {
        result = 3 * firstIndexWithLen(length / 2 + 1) + firstIndexWithLen(length / 2 + 2) - 7;
    }
    cache_put(fiwl_cache, length, result);
    return result;
}

static ll numValuesWithLen(ll length) {
    return firstIndexWithLen(length + 1) - firstIndexWithLen(length);
}

static ll positionInT(ll length, ll index) {
    if (length <= 1) return length;
    ll nv = numValuesWithLen((length + 1) / 2);
    if (index < nv) {
        ll position = positionInT((length + 1) / 2, index);
        return position * 2;
    } else {
        ll position = positionInT(length / 2 + 1, numValuesWithLen(length) - index - 1);
        /* highestOneBit equivalent */
        ll val = position + length - 1;
        ll highest = 0;
        if (val > 0) {
            highest = 1;
            while (highest * 2 <= val) highest *= 2;
        }
        return (position + highest * 2) * 2 + 1;
    }
}

static ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Compute highest power of 2 <= n (for n > 0) */
static ll highest_pow2_leq(ll n) {
    ll h = 1;
    while (h * 2 <= n) h *= 2;
    return h;
}

static ll prefixOfT(ll length) {
    if (length <= 0) return 0;
    if (length == 1) return 0;
    ll cached;
    if (cache_get(prefix_cache, length, &cached)) return cached;
    ll half = highest_pow2_leq(length - 1);
    ll result = ((prefixOfT(half) + 1) % MOD * mod_pow(2, length - half, MOD) % MOD
                 - prefixOfT(length - half) - 1 + 2 * MOD) % MOD;
    cache_put(prefix_cache, length, result);
    return result;
}

static ll computeA(ll n) {
    /* Binary search for the length of A(n) */
    ll low = 0, high = 10000000000LL;  /* 10^10 */
    while (low + 1 < high) {
        ll mid = (low + high) / 2;
        if (firstIndexWithLen(mid) > n) high = mid;
        else low = mid;
    }
    ll length = low;
    ll position = positionInT(length, n - firstIndexWithLen(length));
    return (prefixOfT(position + length) - prefixOfT(position) % MOD * mod_pow(2, length, MOD) % MOD + 2 * MOD * MOD) % MOD;
}

int main(void) {
    memset(fiwl_cache, 0, sizeof(fiwl_cache));
    memset(prefix_cache, 0, sizeof(prefix_cache));

    ll ans = 0;
    ll power = 10;
    for (int k = 1; k <= 18; k++) {
        ans = (ans + computeA(power)) % MOD;
        power *= 10;
    }

    printf("%lld\n", ans);
    return 0;
}
