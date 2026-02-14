/*
 * Project Euler 687 - Shuffling Cards
 *
 * DP with memoization: compute probability that the number of "perfect"
 * ranks in a random shuffle of 52 cards is prime.
 *
 * State: (r0, r1, r2, r3, r4, i, u, p) where r_j = number of perfect ranks
 * with j cards remaining, i = category of top card, u = imperfect cards left,
 * p = number of perfect ranks.
 *
 * We use a hash table for memoization.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N_RANKS 13
#define K_CARDS 4

/* Primes up to 13 */
static int is_prime[14] = {0,0,1,1,0,1,0,1,0,0,0,1,0,1};

/* Hash table for memoization */
#define HTSIZE (1 << 22)
#define HTMASK (HTSIZE - 1)

typedef unsigned long long ull;

typedef struct {
    ull key;
    double val;
    int used;
} HEntry;

static HEntry htable[HTSIZE];

static ull make_key(int r0, int r1, int r2, int r3, int r4, int i, int u, int p) {
    /* Pack into 64-bit key. Each r_j < 14, i < 5, u < 53, p < 14. */
    ull k = 0;
    k = r0;
    k = k * 14 + r1;
    k = k * 14 + r2;
    k = k * 14 + r3;
    k = k * 14 + r4;
    k = k * 5 + i;
    k = k * 53 + u;
    k = k * 14 + p;
    return k;
}

static double ht_get(ull key, int *found) {
    unsigned h = (unsigned)(key ^ (key >> 22)) & HTMASK;
    for (int step = 0; step < 32; step++) {
        unsigned idx = (h + step) & HTMASK;
        if (!htable[idx].used) { *found = 0; return 0.0; }
        if (htable[idx].key == key) { *found = 1; return htable[idx].val; }
    }
    *found = 0;
    return 0.0;
}

static void ht_put(ull key, double val) {
    unsigned h = (unsigned)(key ^ (key >> 22)) & HTMASK;
    for (int step = 0; step < 32; step++) {
        unsigned idx = (h + step) & HTMASK;
        if (!htable[idx].used) {
            htable[idx].key = key;
            htable[idx].val = val;
            htable[idx].used = 1;
            return;
        }
        if (htable[idx].key == key) {
            htable[idx].val = val;
            return;
        }
    }
    /* Table full - shouldn't happen with proper sizing */
}

static double helper(int r0, int r1, int r2, int r3, int r4, int i, int u, int p) {
    int r[5] = {r0, r1, r2, r3, r4};

    if (p == r[0] && u == 0 && i == 0) {
        return is_prime[p] ? 1.0 : 0.0;
    }

    ull key = make_key(r0, r1, r2, r3, r4, i, u, p);
    int found;
    double cached = ht_get(key, &found);
    if (found) return cached;

    double res = 0.0;

    /* Case: next card is from imperfect ranks */
    if (u > 0) {
        res += u * helper(r0, r1, r2, r3, r4, 0, u - 1, p);
    }

    /* Case: next card has same rank as top (category j == i) */
    for (int j = 1; j <= 4; j++) {
        if (r[j] > 0 && j == i) {
            /* Same rank as top: rank becomes imperfect */
            int nr[5] = {r0, r1, r2, r3, r4};
            nr[j]--;
            res += j * helper(nr[0], nr[1], nr[2], nr[3], nr[4], 0, u + j - 1, p - 1);
        }
    }

    /* Case: next card has different rank, category j != i or j == i with r_j - 1 */
    for (int j = 1; j <= 4; j++) {
        if (r[j] > 0) {
            int nr[5] = {r0, r1, r2, r3, r4};
            nr[j]--;
            nr[j-1]++;
            int mult = j * ((j == i) ? (r[j] - 1) : r[j]);
            if (mult > 0) {
                res += mult * helper(nr[0], nr[1], nr[2], nr[3], nr[4], j - 1, u, p);
            }
        }
    }

    ht_put(key, res);
    return res;
}

int main(void) {
    memset(htable, 0, sizeof(htable));

    double result = helper(0, 0, 0, 0, N_RANKS, 0, 0, N_RANKS);

    /* Divide by 52! */
    double total_perm = 1.0;
    for (int i = 1; i <= N_RANKS * K_CARDS; i++)
        total_perm *= i;

    double ans = result / total_perm;
    printf("%.10f\n", ans);
    return 0;
}
