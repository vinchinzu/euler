/*
 * Project Euler Problem 548: Gozinta Chains.
 * Let g(n) = number of gozinta chains for n (sequences {1, a, b, ..., n}
 * where each term properly divides the next).
 * Find the sum of all n <= 10^16 such that g(n) = n.
 *
 * g depends only on the exponent signature. We enumerate all signatures
 * and check if g(signature) equals the number with those exponents
 * assigned to the first few primes.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

#define MAX_PRIMES 20

static int primes[MAX_PRIMES] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71};

/* Hash table for g(exponents) cache */
#define HT_SIZE (1 << 18)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    ll key;  /* encoded exponents */
    ll val;
    int used;
} GEntry;

static GEntry g_cache[1 << 18];

static ll encode_exps(int *es, int n) {
    ll key = n;
    for (int i = 0; i < n; i++)
        key = key * 60 + es[i];
    return key;
}

static int g_cache_get(ll key, ll *val) {
    unsigned int idx = (unsigned int)((unsigned long long)key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 256; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!g_cache[pos].used) return 0;
        if (g_cache[pos].key == key) { *val = g_cache[pos].val; return 1; }
    }
    return 0;
}

static void g_cache_set(ll key, ll val) {
    unsigned int idx = (unsigned int)((unsigned long long)key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 256; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!g_cache[pos].used || g_cache[pos].key == key) {
            g_cache[pos].key = key;
            g_cache[pos].val = val;
            g_cache[pos].used = 1;
            return;
        }
    }
}

static int parity(int n) {
    return (n % 2 == 0) ? 1 : -1;
}

/* Compute g for given exponents (sorted descending) */
static ll g_func(int *es, int n) {
    if (n == 0) return 1;

    ll key = encode_exps(es, n);
    ll cached;
    if (g_cache_get(key, &cached)) return cached;

    ll result = 0;
    int n_subsets = 1 << n;

    for (int subset = 1; subset < n_subsets; subset++) {
        int fs[MAX_PRIMES];
        int fn = 0;
        for (int i = 0; i < n; i++) {
            int e = es[i] - ((subset >> i) & 1);
            if (e > 0) fs[fn++] = e;
        }

        /* Sort fs descending */
        for (int i = 0; i < fn; i++)
            for (int j = i + 1; j < fn; j++)
                if (fs[i] < fs[j]) { int t = fs[i]; fs[i] = fs[j]; fs[j] = t; }

        int bit_count = __builtin_popcount(subset);
        if (fn == 0) {
            result -= parity(bit_count);
        } else {
            result -= parity(bit_count) * 2 * g_func(fs, fn);
        }
    }

    g_cache_set(key, result);
    return result;
}

/* Check if n has the given exponent signature */
static int has_exponents(ll n, int *es, int ne) {
    int es_copy[MAX_PRIMES];
    int nc = ne;
    for (int i = 0; i < ne; i++) es_copy[i] = es[i];

    for (ll factor = 2; factor * factor <= n && nc > 0; factor++) {
        int e = 0;
        while (n % factor == 0) {
            n /= factor;
            e++;
        }
        if (e > 0) {
            /* Find and remove e from es_copy */
            int found = -1;
            for (int i = 0; i < nc; i++) {
                if (es_copy[i] == e) { found = i; break; }
            }
            if (found < 0) return 0;
            es_copy[found] = es_copy[nc - 1];
            nc--;
        }
    }
    if (n > 1) {
        int found = -1;
        for (int i = 0; i < nc; i++) {
            if (es_copy[i] == 1) { found = i; break; }
        }
        if (found < 0) return 0;
        es_copy[found] = es_copy[nc - 1];
        nc--;
    }
    return nc == 0;
}

static ll ipow(ll base, int exp) {
    ll result = 1;
    for (int i = 0; i < exp; i++) {
        if (result > (ll)1e17 / (base + 1)) return (ll)1e17; /* overflow guard */
        result *= base;
    }
    return result;
}

static ll ans = 0;
static ll N_LIMIT;

static int ilog2(ll n) {
    int r = 0;
    while (n > 1) { n >>= 1; r++; }
    return r;
}

/* Recursive helper: enumerate exponent signatures */
static void helper(int *es, int ne, ll n) {
    if (ne > 0) {
        /* Sort es descending for g computation */
        int sorted[MAX_PRIMES];
        for (int i = 0; i < ne; i++) sorted[i] = es[i];
        for (int i = 0; i < ne; i++)
            for (int j = i + 1; j < ne; j++)
                if (sorted[i] < sorted[j]) { int t = sorted[i]; sorted[i] = sorted[j]; sorted[j] = t; }

        ll g_val = g_func(sorted, ne);
        if (g_val > 0 && g_val <= N_LIMIT && has_exponents(g_val, sorted, ne)) {
            ans += g_val;
        }
    }

    int max_c = (ne > 0) ? es[ne - 1] : ilog2(N_LIMIT);
    if (ne >= MAX_PRIMES) return;

    for (int c = 1; c <= max_c; c++) {
        ll new_n = n * ipow(primes[ne], c);
        if (new_n > N_LIMIT) break;
        es[ne] = c;
        helper(es, ne + 1, new_n);
    }
}

int main(void) {
    N_LIMIT = 10000000000000000LL; /* 10^16 */
    memset(g_cache, 0, sizeof(g_cache));

    int es[MAX_PRIMES];
    helper(es, 0, 1);

    printf("%lld\n", ans);
    return 0;
}
