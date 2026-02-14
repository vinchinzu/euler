/*
 * Project Euler 676 - Matching Digit Sums
 *
 * Digit DP with LCM-based chunking.
 * For each (k, l) pair, compute M(10^16, k, l) using digit DP
 * in base 2^lcm(k,l).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

static int gcd_f(int a, int b) { while (b) { int t = b; b = a % b; a = t; } return a; }

/* Hash map for (diff -> (count, sum)) */
#define HM_SIZE 8192
#define HM_MASK (HM_SIZE - 1)

typedef struct {
    int diff;
    ll count;
    ll sum;
    int occupied;
} HMEntry;

typedef struct {
    HMEntry entries[HM_SIZE];
    int keys[HM_SIZE]; /* for iteration */
    int nkeys;
} HashMap;

static void hm_init(HashMap *h) {
    memset(h->entries, 0, sizeof(h->entries));
    h->nkeys = 0;
}

static int hm_hash(int diff) {
    unsigned int u = (unsigned int)(diff + 10000);
    u = (u ^ (u >> 16)) * 0x45d9f3b;
    return u & HM_MASK;
}

static void hm_add(HashMap *h, int diff, ll count, ll sum) {
    int idx = hm_hash(diff);
    for (;;) {
        if (!h->entries[idx].occupied) {
            h->entries[idx].diff = diff;
            h->entries[idx].count = count;
            h->entries[idx].sum = sum;
            h->entries[idx].occupied = 1;
            h->keys[h->nkeys++] = idx;
            return;
        }
        if (h->entries[idx].diff == diff) {
            h->entries[idx].count += count;
            h->entries[idx].sum += sum;
            return;
        }
        idx = (idx + 1) & HM_MASK;
    }
}

static int hm_get(HashMap *h, int diff, ll *count, ll *sum) {
    int idx = hm_hash(diff);
    for (;;) {
        if (!h->entries[idx].occupied) return 0;
        if (h->entries[idx].diff == diff) {
            *count = h->entries[idx].count;
            *sum = h->entries[idx].sum;
            return 1;
        }
        idx = (idx + 1) & HM_MASK;
    }
}

/* Precompute diff_lookup and delta statistics */
static int diff_lookup_buf[1 << 12]; /* max lcm = 12 for (6,4), 2^12 = 4096 */

static int get_diff(int v, int k, int l, int lcm_val) {
    int mask_k = (1 << k) - 1;
    int mask_l = (1 << l) - 1;
    int sk = 0, sl = 0;
    for (int i = 0; i < lcm_val / k; i++) sk += (v >> (i * k)) & mask_k;
    for (int i = 0; i < lcm_val / l; i++) sl += (v >> (i * l)) & mask_l;
    return sl - sk;
}

static ll M_func(ll N, int k, int l) {
    int g = gcd_f(k, l);
    int lcm_val = k * l / g;
    int max_v = 1 << lcm_val;

    for (int v = 0; v < max_v; v++) {
        diff_lookup_buf[v] = get_diff(v, k, l, lcm_val);
    }

    /* Delta statistics for free transitions */
    /* diff range: max delta ~ lcm_val * max_v, but actually bounded */
    HashMap delta_stats;
    hm_init(&delta_stats);
    for (int d = 0; d < max_v; d++) {
        int delta = diff_lookup_buf[d];
        ll cnt_old = 0, sum_old = 0;
        hm_get(&delta_stats, delta, &cnt_old, &sum_old);
        /* Re-add with updated values */
        /* Since hm_add accumulates, we just add */
        /* But we need to set, not accumulate for first init */
    }
    /* Redo: just use arrays for delta stats */
    /* Max possible diff per digit: bounded by max digit sum difference */
    /* For lcm=12, max digit sum in base k is (2^k-1)*(12/k), similar for l */
    /* Let's use hash map properly */
    hm_init(&delta_stats);
    for (int d = 0; d < max_v; d++) {
        int delta = diff_lookup_buf[d];
        hm_add(&delta_stats, delta, 1, (ll)d);
    }

    /* Convert N to base max_v */
    int digits[80];
    int ndigits = 0;
    ull n = (ull)N;
    while (n > 0) {
        digits[ndigits++] = (int)(n % (unsigned)max_v);
        n /= (unsigned)max_v;
    }
    /* Reverse */
    for (int i = 0; i < ndigits / 2; i++) {
        int tmp = digits[i]; digits[i] = digits[ndigits - 1 - i]; digits[ndigits - 1 - i] = tmp;
    }

    if (ndigits == 0) return 0;

    /* Digit DP */
    HashMap tight, free_map, new_tight, new_free;
    hm_init(&tight);
    hm_init(&free_map);

    hm_add(&tight, 0, 1, 0);

    ll pos_val = 1;
    for (int i = 0; i < ndigits - 1; i++) pos_val *= max_v;

    for (int pos = 0; pos < ndigits; pos++) {
        int limit = digits[pos];

        hm_init(&new_tight);
        hm_init(&new_free);

        /* Tight transitions */
        for (int ki = 0; ki < tight.nkeys; ki++) {
            int idx = tight.keys[ki];
            if (!tight.entries[idx].occupied) continue;
            int old_diff = tight.entries[idx].diff;
            ll cnt = tight.entries[idx].count;
            ll sm = tight.entries[idx].sum;
            if (cnt == 0) continue;

            for (int d = 0; d <= limit; d++) {
                int new_diff = old_diff + diff_lookup_buf[d];
                ll contrib = (ll)d * pos_val;
                if (d < limit) {
                    hm_add(&new_free, new_diff, cnt, sm + cnt * contrib);
                } else {
                    hm_add(&new_tight, new_diff, cnt, sm + cnt * contrib);
                }
            }
        }

        /* Free transitions */
        for (int ki = 0; ki < free_map.nkeys; ki++) {
            int idx = free_map.keys[ki];
            if (!free_map.entries[idx].occupied) continue;
            int old_diff = free_map.entries[idx].diff;
            ll cnt = free_map.entries[idx].count;
            ll sm = free_map.entries[idx].sum;
            if (cnt == 0) continue;

            for (int di = 0; di < delta_stats.nkeys; di++) {
                int didx = delta_stats.keys[di];
                if (!delta_stats.entries[didx].occupied) continue;
                int delta = delta_stats.entries[didx].diff;
                ll dc = delta_stats.entries[didx].count;
                ll ds = delta_stats.entries[didx].sum;

                int new_diff = old_diff + delta;
                hm_add(&new_free, new_diff, cnt * dc, dc * sm + cnt * ds * pos_val);
            }
        }

        tight = new_tight;
        free_map = new_free;
        pos_val /= max_v;
    }

    ll total = 0;
    ll cnt, sm;
    if (hm_get(&tight, 0, &cnt, &sm)) total += sm;
    if (hm_get(&free_map, 0, &cnt, &sm)) total += sm;

    return total;
}

int main() {
    ll total = 0;
    for (int k = 3; k <= 6; k++) {
        for (int l = 1; l < k - 1; l++) {
            total += M_func(10000000000000000LL, k, l);
        }
    }
    /* The Python does total % 10^16 */
    ll mod16 = 10000000000000000LL;
    total = ((total % mod16) + mod16) % mod16;
    printf("%lld\n", total);
    return 0;
}
