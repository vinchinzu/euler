/* Project Euler 738: Counting Ordered Factorisations.
 * Memoized recursion: num_products(min_val, count, n_val)
 * n=10^10, mod 10^9+7.
 *
 * Memoization via hash map. States are (min_val, count, n_val).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

/* Simple hash map for memoization */
#define HASH_SIZE (1 << 22)  /* ~4M buckets */
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    ll min_val;
    int count;
    ll n_val;
    ll result;
    struct Entry *next;
} Entry;

static Entry *table[HASH_SIZE];

static unsigned int hash_key(ll min_val, int count, ll n_val) {
    unsigned long long h = (unsigned long long)min_val * 1000000007ULL;
    h ^= (unsigned long long)count * 999999937ULL;
    h ^= (unsigned long long)n_val * 998244353ULL;
    h ^= (h >> 16);
    return (unsigned int)(h & HASH_MASK);
}

static int lookup(ll min_val, int count, ll n_val, ll *result) {
    unsigned int h = hash_key(min_val, count, n_val);
    Entry *e = table[h];
    while (e) {
        if (e->min_val == min_val && e->count == count && e->n_val == n_val) {
            *result = e->result;
            return 1;
        }
        e = e->next;
    }
    return 0;
}

static void insert(ll min_val, int count, ll n_val, ll result) {
    unsigned int h = hash_key(min_val, count, n_val);
    Entry *e = malloc(sizeof(Entry));
    e->min_val = min_val;
    e->count = count;
    e->n_val = n_val;
    e->result = result;
    e->next = table[h];
    table[h] = e;
}

ll num_products(ll min_val, int count, ll n_val) {
    if (count == 1) {
        ll v = n_val - min_val + 1;
        if (v < 0) return 0;
        return v % MOD;
    }

    ll cached;
    if (lookup(min_val, count, n_val, &cached))
        return cached;

    ll result = 0;
    for (ll i = min_val; ; i++) {
        /* Check i^count <= n_val */
        ll pw = 1;
        int overflow = 0;
        for (int j = 0; j < count; j++) {
            if (pw > n_val / i + 1) { overflow = 1; break; }
            pw *= i;
            if (pw > n_val) { overflow = 1; break; }
        }
        if (overflow) break;

        result = (result + num_products(i, count - 1, n_val / i)) % MOD;
    }

    insert(min_val, count, n_val, result);
    return result;
}

int main() {
    ll n = 10000000000LL; /* 10^10 */

    memset(table, 0, sizeof(table));

    ll ans = n % MOD;
    int k = 1;
    while ((1LL << k) <= n) {
        ll np = num_products(2, k, n);
        ll mult = (n - k + 1) % MOD;
        ans = (ans + mult * np % MOD) % MOD;
        k++;
    }

    printf("%lld\n", ans);
    return 0;
}
