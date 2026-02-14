/*
 * Project Euler 867 - Dodecagon Tilings
 *
 * Count tilings of dodecagonal regions using profile DP and memoization.
 * N = 10.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL

/* Hash map for memoization */
#define HT_SIZE (1 << 22)
#define HT_MASK (HT_SIZE - 1)

typedef struct hentry {
    unsigned long long key;
    long long val;
    struct hentry *next;
} hentry;

/* Use chained hashing */
static hentry *htable[HT_SIZE];
static hentry pool[8000000];
static int pool_idx = 0;

static void ht_clear(void) {
    memset(htable, 0, sizeof(htable));
    pool_idx = 0;
}

static long long ht_get(unsigned long long key, int *found) {
    unsigned int h = (unsigned int)(key ^ (key >> 22) ^ (key >> 44)) & HT_MASK;
    for (hentry *e = htable[h]; e; e = e->next) {
        if (e->key == key) { *found = 1; return e->val; }
    }
    *found = 0;
    return 0;
}

static void ht_set(unsigned long long key, long long val) {
    unsigned int h = (unsigned int)(key ^ (key >> 22) ^ (key >> 44)) & HT_MASK;
    for (hentry *e = htable[h]; e; e = e->next) {
        if (e->key == key) { e->val = val; return; }
    }
    hentry *e = &pool[pool_idx++];
    e->key = key;
    e->val = val;
    e->next = htable[h];
    htable[h] = e;
}

static long long mod_pow(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Points for hexagon and trapezoid */
#define MAX_POINTS 256
static int pts_x[MAX_POINTS], pts_y[MAX_POINTS];
static int num_pts;

/* tilings_with_tri_hex using DP */
/* type_key encoded as integer, index, prev_bitset truncated to window_len */
/* Key: type_key * MAX * (1 << window) + index * (1 << window) + (prev_bitset % (1 << window)) */

static long long tilings_with_tri_hex(int px[], int py[], int npts, int type_key, int window_len) {
    /* DP with memoization on (index, prev_bitset mod (1 << window_len)) */
    /* Use a flat table since states are bounded */
    int mask_size = 1 << window_len;
    int total_states = (npts + 1) * mask_size;

    long long *dp = (long long *)calloc(total_states, sizeof(long long));
    if (!dp) { fprintf(stderr, "alloc failed\n"); exit(1); }

    /* dp[index * mask_size + (prev_bitset % mask_size)] */
    /* Base case: index == npts -> 1 */
    for (int m = 0; m < mask_size; m++)
        dp[npts * mask_size + m] = 1;

    /* Fill backwards */
    for (int index = npts - 1; index >= 0; index--) {
        for (int prev = 0; prev < mask_size; prev++) {
            int good = 1;
            int pxi = px[index], pyi = py[index];
            int lim = window_len < index ? window_len : index;
            for (int i = 0; i < lim; i++) {
                if ((prev >> i) & 1) {
                    int qi = index - i - 1;
                    int dy = pyi - py[qi]; if (dy < 0) dy = -dy;
                    int dx = pxi - px[qi]; if (dx < 0) dx = -dx;
                    if (dy <= 1 && dx + dy <= 2) {
                        good = 0;
                        break;
                    }
                }
            }

            long long res = dp[(index + 1) * mask_size + ((prev * 2) % mask_size)];
            if (good) {
                res += dp[(index + 1) * mask_size + ((prev * 2 + 1) % mask_size)];
                res %= MOD;
            }
            dp[index * mask_size + prev] = res;
        }
    }

    long long result = dp[0 * mask_size + 0];
    free(dp);
    return result;
}

/* Cache for hexagon and trapezoid tilings */
static long long hex_cache[12]; /* hex_cache[size] for size 0..11 */
static int hex_computed[12];

static long long trap_cache[12][12]; /* trap_cache[base][height] */
static int trap_computed[12][12];

static long long tilings_for_hexagon(int size) {
    if (size <= 0) return 1;
    if (hex_computed[size]) return hex_cache[size];

    int px[MAX_POINTS], py[MAX_POINTS];
    int n = 0;
    for (int y = -size + 1; y < size; y++) {
        int ay = y < 0 ? -y : y;
        for (int x = -2 * size + ay + 2; x < 2 * size - ay; x += 2) {
            px[n] = x;
            py[n] = y;
            n++;
        }
    }

    long long val = tilings_with_tri_hex(px, py, n, 0, 2 * size - 1);
    hex_cache[size] = val;
    hex_computed[size] = 1;
    return val;
}

static long long tilings_for_trapezoid(int base, int height) {
    if (height <= 0 || base <= 0) return 1;
    if (trap_computed[base][height]) return trap_cache[base][height];

    int px[MAX_POINTS], py[MAX_POINTS];
    int n = 0;
    for (int y = base - height; y < base - 1; y++) {
        for (int x = 1 - y; x < y; x += 2) {
            px[n] = x;
            py[n] = y;
            n++;
        }
    }

    long long val = tilings_with_tri_hex(px, py, n, 0, base - 1);
    trap_cache[base][height] = val;
    trap_computed[base][height] = 1;
    return val;
}

/* Dodecagon memoization */
/* Key: a * 100 + b * 4 + allow_a * 2 + allow_b */
static long long dodec_cache[11 * 100 + 11 * 4 + 4];
static int dodec_computed[11 * 100 + 11 * 4 + 4];

static int dodec_key(int a, int b, int allow_a, int allow_b) {
    return a * 100 + b * 4 + allow_a * 2 + allow_b;
}

static long long tilings_for_dodecagon(int a, int b, int allow_a, int allow_b) {
    if (a == 0) return tilings_for_hexagon(b);
    if (b == 0) return tilings_for_hexagon(a);

    int key = dodec_key(a, b, allow_a, allow_b);
    if (dodec_computed[key]) return dodec_cache[key];

    long long res = 0;

    if (allow_a) {
        for (int h = 1; h <= b; h++) {
            long long t = tilings_for_trapezoid(b, h);
            long long t6 = mod_pow(t, 6, MOD);
            long long sub = tilings_for_dodecagon(a, b - h, 0, 1);
            res = (res + t6 * sub) % MOD;
        }
    }

    if (allow_b) {
        for (int h = 1; h <= a; h++) {
            long long t = tilings_for_trapezoid(a, h);
            long long t6 = mod_pow(t, 6, MOD);
            long long sub = tilings_for_dodecagon(a - h, b, 1, 0);
            res = (res + t6 * sub) % MOD;
        }
    }

    if (a == 1 && b == 1)
        res = (res + 1) % MOD;

    dodec_cache[key] = res;
    dodec_computed[key] = 1;
    return res;
}

int main(void) {
    memset(hex_computed, 0, sizeof(hex_computed));
    memset(trap_computed, 0, sizeof(trap_computed));
    memset(dodec_computed, 0, sizeof(dodec_computed));

    long long ans = tilings_for_dodecagon(10, 10, 1, 1);
    printf("%lld\n", ans);
    return 0;
}
