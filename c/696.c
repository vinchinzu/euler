/*
 * Project Euler 696 - Mahjong
 *
 * NFA-to-DFA DP for single-suit counting, interpolation to large n,
 * polynomial exponentiation across suits.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
#define MOD 1000000007LL
#define T_VAL 30
#define BIG_N 100000000LL
#define BIG_S 100000000LL
#define NUM_STATES 18

#define START_N (3*T_VAL + 3)
#define NPTS (2*T_VAL + 3)
#define MAX_N (START_N + NPTS)

static int encode(int prev, int curr, int pair) {
    return prev * 6 + curr * 2 + pair;
}
static void decode(int code, int *prev, int *curr, int *pair) {
    *pair = code % 2;
    *curr = (code / 2) % 3;
    *prev = code / 6;
}

static int dfa_transition(int old_mask, int k) {
    int new_mask = 0;
    for (int si = 0; si < NUM_STATES; si++) {
        if (!(old_mask & (1 << si))) continue;
        int prev, curr, pair_used;
        decode(si, &prev, &curr, &pair_used);
        int carry = prev + curr;
        int remain = k - carry;
        if (remain < 0) continue;
        for (int nc = 0; nc <= 2; nc++) {
            int r2 = remain - nc;
            if (r2 < 0) break;
            for (int pu = 0; pu <= 1 && 3*pu <= r2; pu++) {
                int r3 = r2 - 3*pu;
                if (r3 == 0)
                    new_mask |= (1 << encode(curr, nc, pair_used));
                else if (r3 == 2 && !pair_used)
                    new_mask |= (1 << encode(curr, nc, 1));
            }
        }
    }
    return new_mask;
}

#define HASH_SIZE (1 << 16)
#define HMASK (HASH_SIZE - 1)

typedef struct HEntry {
    int tiles;
    int mask;
    ll count;
    int next;
} HEntry;

static int heads_a[HASH_SIZE], heads_b[HASH_SIZE];
static HEntry pool_a[500000], pool_b[500000];
static int pa, pb;

static int hfunc(int tiles, int mask) {
    return ((unsigned)(tiles * 262147 ^ mask)) & HMASK;
}

static void add_b(int tiles, int mask, ll count) {
    int h = hfunc(tiles, mask);
    for (int idx = heads_b[h]; idx >= 0; idx = pool_b[idx].next) {
        if (pool_b[idx].tiles == tiles && pool_b[idx].mask == mask) {
            pool_b[idx].count = (pool_b[idx].count + count) % MOD;
            return;
        }
    }
    int ni = pb++;
    pool_b[ni].tiles = tiles;
    pool_b[ni].mask = mask;
    pool_b[ni].count = count % MOD;
    pool_b[ni].next = heads_b[h];
    heads_b[h] = ni;
}

static ll f[2][T_VAL+1][NPTS+1];

static ll power(ll base, ll exp, ll m) {
    ll result = 1;
    base = ((base % m) + m) % m;
    while (exp > 0) {
        if (exp & 1) result = (__int128)result * base % m;
        base = (__int128)base * base % m;
        exp >>= 1;
    }
    return result;
}

static ll inv_mod(ll a) { return power(a, MOD-2, MOD); }

static ll interpolate(ll *y, int npts, ll x_start, ll target) {
    int n = npts;
    ll *prefix = (ll *)malloc((n+1) * sizeof(ll));
    ll *suffix = (ll *)malloc((n+1) * sizeof(ll));

    prefix[0] = 1;
    for (int i = 0; i < n; i++) {
        ll xi = x_start + i;
        prefix[i+1] = (__int128)prefix[i] * (((target - xi) % MOD + MOD) % MOD) % MOD;
    }

    suffix[n] = 1;
    for (int i = n-1; i >= 0; i--) {
        ll xi = x_start + i;
        suffix[i] = (__int128)suffix[i+1] * (((target - xi) % MOD + MOD) % MOD) % MOD;
    }

    ll *fact_inv = (ll *)malloc(n * sizeof(ll));
    ll fv = 1;
    for (int i = 1; i < n; i++) fv = fv * i % MOD;
    fact_inv[n-1] = inv_mod(fv);
    for (int i = n-2; i >= 0; i--)
        fact_inv[i] = fact_inv[i+1] * (i+1) % MOD;

    ll result = 0;
    for (int i = 0; i < n; i++) {
        ll num = (__int128)prefix[i] * suffix[i+1] % MOD;
        ll den = (__int128)fact_inv[i] * fact_inv[n-1-i] % MOD;
        if ((n-1-i) % 2 == 1) den = (MOD - den) % MOD;
        result = (result + (__int128)(y[i] % MOD) * num % MOD * den) % MOD;
    }

    free(prefix); free(suffix); free(fact_inv);
    return result;
}

int main(void) {
    memset(f, 0, sizeof(f));

    int max_tiles = 3 * T_VAL + 2;

    memset(heads_a, -1, sizeof(heads_a));
    pa = 1;
    pool_a[0].tiles = 0;
    pool_a[0].mask = 1 << encode(0,0,0);
    pool_a[0].count = 1;
    pool_a[0].next = -1;
    heads_a[hfunc(0, 1 << encode(0,0,0))] = 0;

    for (int step = 1; step <= MAX_N; step++) {
        memset(heads_b, -1, sizeof(heads_b));
        pb = 0;

        for (int i = 0; i < pa; i++) {
            int tiles = pool_a[i].tiles;
            int mask = pool_a[i].mask;
            ll count = pool_a[i].count;
            if (count == 0) continue;

            for (int k = 0; k <= 4; k++) {
                if (tiles + k > max_tiles) break;
                int nm = dfa_transition(mask, k);
                if (nm) add_b(tiles + k, nm, count);
            }
        }

        pa = pb;
        memcpy(pool_a, pool_b, pb * sizeof(HEntry));
        memcpy(heads_a, heads_b, sizeof(heads_a));

        if (step >= START_N && step < START_N + NPTS) {
            int idx = step - START_N;
            int fs0 = encode(0,0,0), fs1 = encode(0,0,1);
            for (int i = 0; i < pa; i++) {
                int tiles = pool_a[i].tiles;
                int mask = pool_a[i].mask;
                ll count = pool_a[i].count;

                if (mask & (1 << fs1)) {
                    if (tiles >= 2 && (tiles-2)%3 == 0) {
                        int t = (tiles-2)/3;
                        if (t <= T_VAL)
                            f[1][t][idx] = (f[1][t][idx] + count) % MOD;
                    }
                }
                if (mask & (1 << fs0)) {
                    if (tiles%3 == 0) {
                        int t = tiles/3;
                        if (t <= T_VAL)
                            f[0][t][idx] = (f[0][t][idx] + count) % MOD;
                    }
                }
            }
        }
    }

    ll g0[T_VAL+1], g1[T_VAL+1];
    for (int t = 0; t <= T_VAL; t++) {
        int npts0 = 2*t + 1;
        if (npts0 > NPTS) npts0 = NPTS;
        if (npts0 < 1) npts0 = 1;
        g0[t] = interpolate(f[0][t], npts0, (ll)START_N, BIG_N);

        int npts1 = 2*t + 3;
        if (npts1 > NPTS) npts1 = NPTS;
        if (npts1 < 1) npts1 = 1;
        g1[t] = interpolate(f[1][t], npts1, (ll)START_N, BIG_N);
    }

    ll poly[T_VAL+1], base[T_VAL+1], temp[T_VAL+1];
    memset(poly, 0, sizeof(poly));
    poly[0] = 1;
    for (int i = 0; i <= T_VAL; i++) base[i] = g0[i];

    ll exp = BIG_S - 1;
    while (exp > 0) {
        if (exp & 1) {
            memset(temp, 0, sizeof(temp));
            for (int i = 0; i <= T_VAL; i++) {
                if (poly[i] == 0) continue;
                for (int j = 0; j <= T_VAL - i; j++)
                    temp[i+j] = (temp[i+j] + (__int128)poly[i] * base[j]) % MOD;
            }
            memcpy(poly, temp, sizeof(poly));
        }
        memset(temp, 0, sizeof(temp));
        for (int i = 0; i <= T_VAL; i++) {
            if (base[i] == 0) continue;
            for (int j = 0; j <= T_VAL - i; j++)
                temp[i+j] = (temp[i+j] + (__int128)base[i] * base[j]) % MOD;
        }
        memcpy(base, temp, sizeof(base));
        exp >>= 1;
    }

    ll ans = 0;
    for (int i = 0; i <= T_VAL; i++) {
        int j = T_VAL - i;
        if (j >= 0 && j <= T_VAL)
            ans = (ans + (__int128)poly[i] * g1[j]) % MOD;
    }
    ans = (__int128)ans * (BIG_S % MOD) % MOD;

    printf("%lld\n", ans);
    return 0;
}
