/*
 * Project Euler 789 - Minimal Pairing Product
 *
 * Find product of pairs minimizing sum, via bidirectional search
 * over small-cost products of primes, looking for product * inv = -1 (mod N).
 *
 * Uses hash map for value -> cost mapping.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define N_VAL 2000000011LL

static ll powmod(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m; if (base < 0) base += m;
    while (exp > 0) {
        if (exp & 1) result = (__int128)result * base % m;
        base = (__int128)base * base % m;
        exp >>= 1;
    }
    return result;
}

static ll mod_inv(ll a, ll m) {
    return powmod(a, m - 2, m);
}

/* Hash map: key (ll) -> value (int cost) */
#define HM_SIZE (1 << 22)
#define HM_MASK (HM_SIZE - 1)

typedef struct {
    ll key;
    int val;
    char used;
} HMEntry;

static HMEntry hm[HM_SIZE];

static void hm_clear(void) {
    memset(hm, 0, sizeof(hm));
}

static void hm_set(ll key, int val) {
    unsigned int idx = (unsigned int)((ull)key * 2654435761ULL) & HM_MASK;
    while (hm[idx].used) {
        if (hm[idx].key == key) {
            if (val < hm[idx].val) hm[idx].val = val;
            return;
        }
        idx = (idx + 1) & HM_MASK;
    }
    hm[idx].key = key;
    hm[idx].val = val;
    hm[idx].used = 1;
}

static int hm_get(ll key, int *val_out) {
    unsigned int idx = (unsigned int)((ull)key * 2654435761ULL) & HM_MASK;
    while (hm[idx].used) {
        if (hm[idx].key == key) {
            *val_out = hm[idx].val;
            return 1;
        }
        idx = (idx + 1) & HM_MASK;
    }
    return 0;
}

/* Primes up to 100 */
static int primes[] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97};
static int nprimes = 25;

/* Collect all (prod, cost) pairs with recursive DFS */
static void helper(int min_index, int cost, ll prod, int cost_bound) {
    hm_set(prod, cost);

    for (int i = min_index; i < nprimes; i++) {
        int p = primes[i];
        if (cost + p - 1 > cost_bound) return;
        if (prod * p > 10000000000LL) return;
        helper(i, cost + p - 1, prod * p, cost_bound);
    }
}

/* Iterate over all entries in hashmap */
typedef struct { ll prod; int cost; } Entry;

static Entry *entries;
static int nentries;

static void collect_entries(void) {
    nentries = 0;
    for (int i = 0; i < HM_SIZE; i++) {
        if (hm[i].used) {
            entries[nentries].prod = hm[i].key;
            entries[nentries].cost = hm[i].val;
            nentries++;
        }
    }
}

int main(void) {
    entries = (Entry *)malloc(HM_SIZE * sizeof(Entry));

    ll ans_prod = -1;
    int cost_bound = 1;

    while (ans_prod < 0) {
        hm_clear();
        helper(0, 0, 1, cost_bound);
        collect_entries();

        int min_cost = 1 << 30;
        ll best_prod = -1;

        for (int i = 0; i < nentries; i++) {
            ll prod = entries[i].prod;
            int c1 = entries[i].cost;
            ll inv = mod_inv((((-prod) % N_VAL) + N_VAL) % N_VAL, N_VAL);
            int c2;
            if (hm_get(inv, &c2)) {
                int total = c1 + c2;
                if (total < min_cost) {
                    min_cost = total;
                    best_prod = (__int128)prod * inv % N_VAL;
                }
            }
        }

        if (best_prod >= 0) ans_prod = best_prod;
        cost_bound *= 2;
    }

    printf("%lld\n", ans_prod);

    free(entries);
    return 0;
}
