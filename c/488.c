/* Project Euler 488 - Unbalanced Nim
 * Translated from python/488.py
 *
 * Digit DP on binary representation for XOR triples.
 * N = 10^18, M = 10^9, MOD = 6*10^9
 */
#include <stdio.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD 6000000000LL  /* 6 * 10^9 */
#define M   1000000000LL

/* Hash map for memoization */
#define HSIZE (1 << 18)
#define HMASK (HSIZE - 1)

typedef struct HNode {
    ull n;
    int g;  /* packed g as 3-bit: g0 | g1<<1 | g2<<2 */
    ll count;
    ll total;
    struct HNode *next;
} HNode;

HNode *htable[HSIZE];
HNode pool[200000];
int pool_idx = 0;

HNode* alloc_node(void) {
    return &pool[pool_idx++];
}

unsigned hash_key(ull n, int g) {
    unsigned h = (unsigned)(n ^ (n >> 32)) * 2654435761u;
    h ^= (unsigned)g * 2246822519u;
    return h & HMASK;
}

int cache_lookup(ull n, int g, ll *count, ll *total) {
    unsigned h = hash_key(n, g);
    for (HNode *p = htable[h]; p; p = p->next) {
        if (p->n == n && p->g == g) {
            *count = p->count;
            *total = p->total;
            return 1;
        }
    }
    return 0;
}

void cache_store(ull n, int g, ll count, ll total) {
    unsigned h = hash_key(n, g);
    HNode *nd = alloc_node();
    nd->n = n; nd->g = g; nd->count = count; nd->total = total;
    nd->next = htable[h];
    htable[h] = nd;
}

/* f(n, g) -> (count, total) of triples (a,b,c) with 0<=a,b,c<=n, a^b^c=0 */
void f(ull n, int g, ll *out_count, ll *out_total) {
    if (n == 0) {
        if (g != 0) {
            *out_count = 0; *out_total = 0;
            return;
        }
        *out_count = 1; *out_total = 0;
        return;
    }

    if (cache_lookup(n, g, out_count, out_total)) return;

    ll count = 0, total = 0;
    int nbit = (int)(n & 1);

    /* Iterate over all valid bit combinations for 3 values */
    /* bits[0..2] each 0 or 1, XOR constraint: sum must be even */
    int bits[8][3] = {{0,0,0},{0,1,1},{1,0,1},{1,1,0},
                      {0,0,0},{0,0,0},{0,0,0},{0,0,0}};
    /* Actually only 4 valid combos: (0,0,0),(0,1,1),(1,0,1),(1,1,0) */
    for (int ci = 0; ci < 4; ci++) {
        int b0 = bits[ci][0], b1 = bits[ci][1], b2 = bits[ci][2];
        int new_g = 0;
        for (int i = 0; i < 3; i++) {
            int bi = (i == 0) ? b0 : ((i == 1) ? b1 : b2);
            int gi = (g >> i) & 1;
            int ng = (bi > nbit) || (bi == nbit && gi);
            new_g |= (ng << i);
        }

        ll sub_count, sub_total;
        f(n / 2, new_g, &sub_count, &sub_total);
        int sum_bits = b0 + b1 + b2;
        count = (count + sub_count) % MOD;
        total = (total + 2 * sub_total % MOD + (ll)sum_bits * sub_count % MOD) % MOD;
    }

    count = ((count % MOD) + MOD) % MOD;
    total = ((total % MOD) + MOD) % MOD;

    cache_store(n, g, count, total);
    *out_count = count;
    *out_total = total;
}

int main() {
    ull N = 1000000000000000000ULL;  /* 10^18 */

    memset(htable, 0, sizeof(htable));

    ll f0, f1;
    f(N, 0, &f0, &f1);

    /* Formula: 6S = f1 - 3*f0 - 6*N^2 + 15*N - 3 */
    ll Nsq = 1;
    {
        /* N^2 mod MOD */
        ull nm = N % (ull)MOD;
        Nsq = (ll)((unsigned __int128)nm * nm % (ull)MOD);
    }
    ll Nmod = (ll)(N % (ull)MOD);

    ll numerator = (f1 - 3 * f0 % MOD - 6 * Nsq % MOD + 15 * Nmod % MOD - 3 + 10 * MOD) % MOD;
    numerator = ((numerator % MOD) + MOD) % MOD;
    ll ans = (numerator / 6) % M;

    printf("%lld\n", ans);
    return 0;
}
