/*
 * Project Euler 152 - Writing 1/2 as a sum of inverse squares
 *
 * Uses exact rational arithmetic with long long numerator/denominator.
 * Meet-in-the-middle: enumerate subsets of "large" candidates, then
 * recursive search over "small" candidates.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

typedef long long ll;

static ll gcd_ll(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

typedef struct { ll num; ll den; } Frac;

static Frac frac_new(ll n, ll d) {
    if (d < 0) { n = -n; d = -d; }
    ll g = gcd_ll(n < 0 ? -n : n, d);
    return (Frac){n / g, d / g};
}

static Frac frac_add(Frac a, Frac b) {
    /* Use __int128 to avoid overflow */
    __int128 n = (__int128)a.num * b.den + (__int128)b.num * a.den;
    __int128 d = (__int128)a.den * b.den;
    __int128 g;
    {
        __int128 x = n < 0 ? -n : n;
        __int128 y = d < 0 ? -d : d;
        while (y) { __int128 t = y; y = x % y; x = t; }
        g = x;
    }
    return (Frac){(ll)(n / g), (ll)(d / g)};
}

static bool frac_le(Frac a, Frac b) {
    /* a <= b ? */
    __int128 lhs = (__int128)a.num * b.den;
    __int128 rhs = (__int128)b.num * a.den;
    return lhs <= rhs;
}

static bool frac_lt(Frac a, Frac b) {
    __int128 lhs = (__int128)a.num * b.den;
    __int128 rhs = (__int128)b.num * a.den;
    return lhs < rhs;
}

static bool frac_eq(Frac a, Frac b) {
    return a.num == b.num && a.den == b.den;
}

static int candidates[] = {
    2,3,4,5,6,7,8,9,10,12,13,14,15,16,18,20,21,24,27,28,30,32,35,36,39,40,42,45,48,52,54,56,60,63,64,65,70,72,80
};
static int ncandidates = 39;
static int threshold_val = 40;

/* Hash map for fraction -> count */
#define HASH_SIZE (1 << 18)
#define HASH_MASK (HASH_SIZE - 1)
typedef struct hnode { Frac key; int count; struct hnode *next; } hnode;
static hnode *htable[HASH_SIZE];

static unsigned int frac_hash(Frac f) {
    unsigned long long h = (unsigned long long)f.num * 2654435761ULL ^ (unsigned long long)f.den * 2246822519ULL;
    return (unsigned int)(h & HASH_MASK);
}

static int hash_lookup(Frac f) {
    unsigned int h = frac_hash(f);
    for (hnode *n = htable[h]; n; n = n->next) {
        if (frac_eq(n->key, f)) return n->count;
    }
    return 0;
}

static void hash_add(Frac f, int count) {
    unsigned int h = frac_hash(f);
    for (hnode *n = htable[h]; n; n = n->next) {
        if (frac_eq(n->key, f)) { n->count += count; return; }
    }
    hnode *n = malloc(sizeof(hnode));
    n->key = f;
    n->count = count;
    n->next = htable[h];
    htable[h] = n;
}

/* Suffix sums for pruning */
static Frac suffix_sum[40]; /* ncandidates + 1 */

/* Large candidates and their subset sums */
static int large[40];
static int nlarge = 0;

static Frac target;

static int search(int idx, Frac current) {
    if (frac_eq(current, target)) return 1;
    if (idx >= ncandidates) return 0;
    if (frac_lt(target, current)) return 0;

    /* Pruning: if current + remaining suffix < target, no solution */
    Frac max_possible = frac_add(current, suffix_sum[idx]);
    if (frac_lt(max_possible, target)) return 0;

    int number = candidates[idx];
    if (number >= threshold_val) {
        /* Look up remaining in hash table */
        Frac diff = frac_add(target, frac_new(-current.num, current.den));
        return hash_lookup(diff);
    }

    int res = 0;
    /* Without current */
    res += search(idx + 1, current);
    /* With current */
    Frac add = frac_new(1, (ll)number * number);
    Frac next = frac_add(current, add);
    if (frac_le(next, target)) {
        res += search(idx + 1, next);
    }
    return res;
}

int main(void) {
    target = frac_new(1, 2);

    /* Compute suffix sums */
    suffix_sum[ncandidates] = frac_new(0, 1);
    for (int i = ncandidates - 1; i >= 0; i--) {
        Frac add = frac_new(1, (ll)candidates[i] * candidates[i]);
        suffix_sum[i] = frac_add(add, suffix_sum[i + 1]);
    }

    /* Identify large numbers */
    nlarge = 0;
    for (int i = 0; i < ncandidates; i++) {
        if (candidates[i] >= threshold_val) {
            large[nlarge++] = candidates[i];
        }
    }

    /* Enumerate all subsets of large numbers */
    memset(htable, 0, sizeof(htable));
    for (int mask = 0; mask < (1 << nlarge); mask++) {
        Frac sum_r = frac_new(0, 1);
        for (int j = 0; j < nlarge; j++) {
            if (mask & (1 << j)) {
                sum_r = frac_add(sum_r, frac_new(1, (ll)large[j] * large[j]));
            }
        }
        hash_add(sum_r, 1);
    }

    int count = search(0, frac_new(0, 1));
    printf("%d\n", count);
    return 0;
}
