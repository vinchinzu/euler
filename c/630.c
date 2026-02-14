/*
 * Project Euler 630: Crossed Lines
 *
 * Given N=2500 points from BBS generator, find the number of crossing
 * pairs among all distinct lines.
 *
 * For each slope, count lines; answer = sum over slopes of
 * (num_lines_with_slope * (total - num_lines_with_slope)).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define NPTS 2500
#define L 2000

int px[NPTS], py[NPTS];

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* Hash map: key = (dx, dy, intercept), value = presence */
/* We need to group lines by slope (dx, dy) and count distinct intercepts per slope */

/* Strategy: for each slope (dx, dy), collect all intercepts in a set.
 * Use a two-level approach:
 * 1. Map slope -> index
 * 2. For each slope index, store set of intercepts
 */

/* Slope hash map */
#define SLOPE_HASH_SIZE (1 << 16)
#define SLOPE_HASH_MASK (SLOPE_HASH_SIZE - 1)

typedef struct {
    int dx, dy;
    int count;       /* number of distinct intercepts */
    int cap;
    ll *intercepts;  /* stored intercepts (for dedup) */
} SlopeEntry;

SlopeEntry slopes[SLOPE_HASH_SIZE];
int slope_used[SLOPE_HASH_SIZE];

void slope_init(void) {
    memset(slope_used, 0, sizeof(slope_used));
}

unsigned int slope_hash(int dx, int dy) {
    unsigned int h = (unsigned int)(dx * 1000003 + dy);
    h ^= h >> 16;
    h *= 0x45d9f3b;
    return h & SLOPE_HASH_MASK;
}

/* Insert an intercept for a given slope; return 1 if new */
int slope_add_intercept(int dx, int dy, ll intercept) {
    unsigned int h = slope_hash(dx, dy);
    while (slope_used[h]) {
        if (slopes[h].dx == dx && slopes[h].dy == dy) {
            /* Found slope, check if intercept exists */
            for (int i = 0; i < slopes[h].count; i++) {
                if (slopes[h].intercepts[i] == intercept) return 0;
            }
            /* Add new intercept */
            if (slopes[h].count >= slopes[h].cap) {
                slopes[h].cap *= 2;
                slopes[h].intercepts = realloc(slopes[h].intercepts,
                    slopes[h].cap * sizeof(ll));
            }
            slopes[h].intercepts[slopes[h].count++] = intercept;
            return 1;
        }
        h = (h + 1) & SLOPE_HASH_MASK;
    }
    /* New slope */
    slope_used[h] = 1;
    slopes[h].dx = dx;
    slopes[h].dy = dy;
    slopes[h].cap = 16;
    slopes[h].intercepts = malloc(16 * sizeof(ll));
    slopes[h].intercepts[0] = intercept;
    slopes[h].count = 1;
    return 1;
}

int main(void) {
    /* Generate points using BBS */
    ll s = 290797;
    for (int i = 0; i < NPTS; i++) {
        s = s * s % 50515093;
        px[i] = (int)(s % L) - 1000;
        s = s * s % 50515093;
        py[i] = (int)(s % L) - 1000;
    }

    slope_init();

    for (int i = 0; i < NPTS; i++) {
        for (int j = 0; j < NPTS; j++) {
            if (i == j) continue;
            int dx = px[j] - px[i];
            int dy = py[j] - py[i];
            /* Normalize: dy > 0, or dy == 0 && dx > 0 */
            if (dy < 0 || (dy == 0 && dx < 0)) {
                dx = -dx;
                dy = -dy;
            }
            if (dy == 0 && dx == 0) continue;

            int g = gcd(abs(dx), abs(dy));
            if (g > 0) {
                dx /= g;
                dy /= g;
            }
            ll intercept = (ll)dy * px[i] - (ll)dx * py[i];
            slope_add_intercept(dx, dy, intercept);
        }
    }

    /* Compute total lines and answer */
    ll total = 0;
    for (int i = 0; i < SLOPE_HASH_SIZE; i++) {
        if (slope_used[i]) total += slopes[i].count;
    }

    ll ans = 0;
    for (int i = 0; i < SLOPE_HASH_SIZE; i++) {
        if (slope_used[i]) {
            ll c = slopes[i].count;
            ans += c * (total - c);
        }
    }

    printf("%lld\n", ans);

    /* Cleanup */
    for (int i = 0; i < SLOPE_HASH_SIZE; i++) {
        if (slope_used[i]) free(slopes[i].intercepts);
    }

    return 0;
}
