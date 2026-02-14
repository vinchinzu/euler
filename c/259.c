/*
 * Project Euler Problem 259: Reachable Numbers
 *
 * Using digits 1-9 concatenated and operations +,-,*,/ find all positive
 * integers reachable. Output their sum.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

static ll my_abs_ll(ll x) { return x < 0 ? -x : x; }

static ll gcd_func(ll a, ll b) {
    a = my_abs_ll(a);
    b = my_abs_ll(b);
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

typedef struct { ll num, den; } Frac;

static Frac make_frac(ll n, ll d) {
    if (d < 0) { n = -n; d = -d; }
    if (n == 0) { d = 1; }
    else { ll g = gcd_func(n, d); n /= g; d /= g; }
    return (Frac){n, d};
}

typedef struct {
    Frac *data;
    int *chain;
    int *buckets;
    int count;
    int capacity;
    int hash_size;
} FracSet;

static FracSet sets[9][10];

static unsigned hash_frac(ll num, ll den, unsigned hash_size) {
    unsigned long long h = (unsigned long long)num * 2654435761ULL
                         ^ (unsigned long long)den * 40503ULL;
    return (unsigned)(h & (hash_size - 1));
}

static void set_init(FracSet *s, int cap) {
    s->count = 0;
    s->capacity = cap;
    s->data = (Frac*)malloc(cap * sizeof(Frac));
    s->chain = (int*)malloc(cap * sizeof(int));
    int hs = 16;
    while (hs < cap * 2) hs *= 2;
    s->hash_size = hs;
    s->buckets = (int*)malloc(hs * sizeof(int));
    memset(s->buckets, -1, hs * sizeof(int));
}

static void set_grow(FracSet *s) {
    int new_cap = s->capacity * 2;
    s->data = (Frac*)realloc(s->data, new_cap * sizeof(Frac));
    s->chain = (int*)realloc(s->chain, new_cap * sizeof(int));
    s->capacity = new_cap;
    int new_hs = 16;
    while (new_hs < new_cap * 2) new_hs *= 2;
    s->hash_size = new_hs;
    s->buckets = (int*)realloc(s->buckets, new_hs * sizeof(int));
    memset(s->buckets, -1, new_hs * sizeof(int));
    for (int i = 0; i < s->count; i++) {
        unsigned h = hash_frac(s->data[i].num, s->data[i].den, new_hs);
        s->chain[i] = s->buckets[h];
        s->buckets[h] = i;
    }
}

static int set_contains(FracSet *s, Frac f) {
    unsigned h = hash_frac(f.num, f.den, s->hash_size);
    int idx = s->buckets[h];
    while (idx != -1) {
        if (s->data[idx].num == f.num && s->data[idx].den == f.den) return 1;
        idx = s->chain[idx];
    }
    return 0;
}

static void set_add(FracSet *s, Frac f) {
    if (set_contains(s, f)) return;
    if (s->count >= s->capacity) set_grow(s);
    int idx = s->count++;
    s->data[idx] = f;
    unsigned h = hash_frac(f.num, f.den, s->hash_size);
    s->chain[idx] = s->buckets[h];
    s->buckets[h] = idx;
}

static void try_add(FracSet *s, lll n, lll d) {
    if (d == 0) return;
    lll an = n < 0 ? -n : n;
    lll ad = d < 0 ? -d : d;
    lll ga = an, gb = ad;
    while (gb) { lll t = gb; gb = ga % gb; ga = t; }
    if (ga > 0) { n /= ga; d /= ga; }
    an = n < 0 ? -n : n;
    ad = d < 0 ? -d : d;
    if (an > 4000000000000000000LL || ad > 4000000000000000000LL) return;
    set_add(s, make_frac((ll)n, (ll)d));
}

static void add_ops(FracSet *s, Frac a, Frac b) {
    lll n, d;

    /* addition */
    n = (lll)a.num * b.den + (lll)b.num * a.den;
    d = (lll)a.den * b.den;
    try_add(s, n, d);

    /* subtraction */
    n = (lll)a.num * b.den - (lll)b.num * a.den;
    d = (lll)a.den * b.den;
    try_add(s, n, d);

    /* multiplication */
    {
        ll g1 = gcd_func(a.num, b.den);
        ll g2 = gcd_func(b.num, a.den);
        n = (lll)(a.num/g1) * (b.num/g2);
        d = (lll)(a.den/g2) * (b.den/g1);
        try_add(s, n, d);
    }

    /* division */
    if (b.num != 0) {
        ll g1 = gcd_func(a.num, b.num);
        ll g2 = gcd_func(a.den, b.den);
        n = (lll)(a.num/g1) * (b.den/g2);
        d = (lll)(a.den/g2) * (b.num/g1);
        try_add(s, n, d);
    }
}

int main(void) {
    int B = 10;

    /* Single digits */
    for (int i = 0; i < B - 1; i++) {
        set_init(&sets[i][i+1], 4);
        set_add(&sets[i][i+1], make_frac(i + 1, 1));
    }

    /* Build up longer sequences */
    for (int length = 2; length < B; length++) {
        for (int start = 0; start < B - length; start++) {
            int end = start + length;
            int cap = 8;
            if (length <= 4) cap = 512;
            else if (length == 5) cap = 2048;
            else if (length == 6) cap = 16384;
            else if (length == 7) cap = 65536;
            else if (length == 8) cap = 500000;
            else cap = 4000000;

            set_init(&sets[start][end], cap);
            FracSet *cur = &sets[start][end];

            /* Concatenated number */
            ll concat = 0;
            for (int d = start; d < end; d++)
                concat = concat * 10 + (d + 1);
            set_add(cur, make_frac(concat, 1));

            /* Try all splits */
            for (int left = 1; left < length; left++) {
                int mid = start + left;
                FracSet *lset = &sets[start][mid];
                FracSet *rset = &sets[mid][end];
                int lc = lset->count, rc = rset->count;
                for (int li = 0; li < lc; li++) {
                    Frac fa = lset->data[li];
                    for (int ri = 0; ri < rc; ri++) {
                        add_ops(cur, fa, rset->data[ri]);
                    }
                }
            }
        }
    }

    /* Sum all positive integers in dp[(0, B-1)] */
    ll ans = 0;
    FracSet *fs = &sets[0][B-1];
    for (int i = 0; i < fs->count; i++) {
        if (fs->data[i].den == 1 && fs->data[i].num > 0)
            ans += fs->data[i].num;
    }

    printf("%lld\n", ans);
    return 0;
}
