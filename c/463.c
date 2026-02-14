/*
 * Project Euler Problem 463: A weird recurrence relation
 *
 * f(1)=1, f(3)=3, f(2n)=n, f(4n+1)=2f(2n+1)-f(n), f(4n+3)=3f(2n+1)-2f(n)
 * Find sum_{k=1}^{3^37} f(k) mod 10^9.
 *
 * Uses memoized recursive computation of sum_f and sum_odd_f.
 * Hash map for caching since keys can be up to 3^37.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long long ull;
typedef long long ll;

#define M 1000000000LL
#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    ull key;
    ll value;
    struct Entry *next;
} Entry;

typedef struct {
    Entry *buckets[HASH_SIZE];
} HashMap;

static HashMap cache_sf, cache_sof;

static void hm_init(HashMap *hm) {
    memset(hm->buckets, 0, sizeof(hm->buckets));
}

static ll hm_get(HashMap *hm, ull key, int *found) {
    unsigned h = (unsigned)(key * 2654435761ULL) & HASH_MASK;
    Entry *e = hm->buckets[h];
    while (e) {
        if (e->key == key) {
            *found = 1;
            return e->value;
        }
        e = e->next;
    }
    *found = 0;
    return 0;
}

static void hm_put(HashMap *hm, ull key, ll value) {
    unsigned h = (unsigned)(key * 2654435761ULL) & HASH_MASK;
    Entry *e = malloc(sizeof(Entry));
    e->key = key;
    e->value = value;
    e->next = hm->buckets[h];
    hm->buckets[h] = e;
}

static ll sum_odd_f(ull n);

static ll sum_f(ull n) {
    if (n == 0) return 0;
    int found;
    ll val = hm_get(&cache_sf, n, &found);
    if (found) return val;

    ll result = 0;
    if (n >= 1) result += 1;
    if (n >= 2) result += 1;
    if (n >= 3) result += 3;
    if (n >= 4) {
        result += sum_f(n / 4);
        result += 2 * sum_odd_f((n - 1) / 4);
        result -= sum_f((n - 1) / 4);
        result += sum_odd_f((n - 2) / 4);
        result += 3 * sum_odd_f((n - 3) / 4);
        result -= 2 * sum_f((n - 3) / 4);
    }
    result = ((result % M) + M) % M;
    hm_put(&cache_sf, n, result);
    return result;
}

static ll sum_odd_f(ull n) {
    if (n == 0) return 0;
    int found;
    ll val = hm_get(&cache_sof, n, &found);
    if (found) return val;

    ll result = 0;
    if (n >= 1) result += 3;
    if (n >= 2) {
        result += 2 * sum_odd_f(n / 2);
        result -= sum_f(n / 2);
        result += 3 * sum_odd_f((n - 1) / 2);
        result -= 2 * sum_f((n - 1) / 2);
    }
    result = ((result % M) + M) % M;
    hm_put(&cache_sof, n, result);
    return result;
}

int main(void) {
    /* N = 3^37 */
    ull N = 1;
    for (int i = 0; i < 37; i++) N *= 3;

    hm_init(&cache_sf);
    hm_init(&cache_sof);

    ll ans = sum_f(N);
    printf("%lld\n", ans);
    return 0;
}
