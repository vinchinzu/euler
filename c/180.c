/*
 * Project Euler Problem 180: Rational zeros of a function of three variables.
 *
 * For reduced fractions 0 < x < 1 with denominator <= 35, find all (x,y,z)
 * satisfying certain power-sum equations. Sum distinct s = x+y+z values
 * and output numerator + denominator.
 *
 * Uses __int128 throughout for exact rational arithmetic.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long i64;
typedef __int128 i128;

static i128 abs128(i128 x) { return x < 0 ? -x : x; }

static i128 gcd128(i128 a, i128 b) {
    a = abs128(a);
    b = abs128(b);
    while (b) { i128 t = b; b = a % b; a = t; }
    return a;
}

typedef struct {
    i128 num, den;
} Frac;

static Frac frac_new(i128 n, i128 d) {
    if (d < 0) { n = -n; d = -d; }
    if (d == 0) { Frac f = {0, 0}; return f; }
    i128 g = gcd128(n, d);
    Frac f = {n / g, d / g};
    return f;
}

static Frac frac_add(Frac a, Frac b) {
    /* Pre-reduce to minimize overflow risk */
    i128 g = gcd128(a.den, b.den);
    i128 da = a.den / g, db = b.den / g;
    return frac_new(a.num * db + b.num * da, da * b.den);
}

static Frac frac_sub(Frac a, Frac b) {
    i128 g = gcd128(a.den, b.den);
    i128 da = a.den / g, db = b.den / g;
    return frac_new(a.num * db - b.num * da, da * b.den);
}

static Frac frac_mul(Frac a, Frac b) {
    i128 g1 = gcd128(abs128(a.num), b.den);
    i128 g2 = gcd128(abs128(b.num), a.den);
    return frac_new((a.num / g1) * (b.num / g2), (a.den / g2) * (b.den / g1));
}

static Frac frac_div(Frac a, Frac b) {
    Frac b_inv = {b.den, b.num};
    return frac_mul(a, b_inv);
}

static int frac_eq(Frac a, Frac b) {
    return a.num == b.num && a.den == b.den;
}

static int frac_lt(Frac a, Frac b) {
    /* Compare a.num/a.den < b.num/b.den */
    /* Both dens positive after reduction */
    return a.num * b.den < b.num * a.den;
}

static int frac_gt(Frac a, Frac b) {
    return a.num * b.den > b.num * a.den;
}

#define MAX_DEN 35
#define MAX_FRACS 500

static Frac fractions[MAX_FRACS];
static int nfracs;
static Frac sq[MAX_FRACS];

static i128 isqrt128(i128 n) {
    if (n <= 0) return 0;
    i128 x = (i128)sqrt((double)(i64)n);
    if (x < 0) x = 0;
    while (x * x < n) x++;
    while (x * x > n) x--;
    return x;
}

static Frac frac_sqrt_val(Frac r) {
    if (r.num < 0 || r.den == 0) return (Frac){0, 0};
    if (r.num == 0) return (Frac){0, 1};
    i128 sn = isqrt128(r.num);
    if (sn * sn != r.num) return (Frac){0, 0};
    i128 sd = isqrt128(r.den);
    if (sd * sd != r.den) return (Frac){0, 0};
    return frac_new(sn, sd);
}

/* Hash set for Frac - we use a simple approach:
 * Store as (i64 num, i64 den) pairs for hashing since individual
 * fractions have small values */
#define SUM_HASH_SIZE (1 << 16)
#define SUM_HASH_MASK (SUM_HASH_SIZE - 1)

typedef struct { i64 num, den; int used; } SHEntry;
static SHEntry sum_table[SUM_HASH_SIZE];

static Frac sum_array[50000];
static int sum_arr_count;

static unsigned int sh_hash(i64 n, i64 d) {
    unsigned long long h = (unsigned long long)(n + 1000000LL) * 1000003ULL;
    h ^= (unsigned long long)(d + 1000000LL) * 999983ULL;
    h ^= h >> 17;
    h *= 0xFF51AFD7ED558CCDULL;
    h ^= h >> 33;
    return (unsigned int)(h & SUM_HASH_MASK);
}

static void sum_insert(Frac f) {
    /* Individual sums x+y+z have small denominators, fit in i64 */
    i64 fn = (i64)f.num, fd = (i64)f.den;
    unsigned int idx = sh_hash(fn, fd);
    for (int i = 0; i < SUM_HASH_SIZE; i++) {
        unsigned int pos = (idx + i) & SUM_HASH_MASK;
        if (!sum_table[pos].used) {
            sum_table[pos].num = fn;
            sum_table[pos].den = fd;
            sum_table[pos].used = 1;
            sum_array[sum_arr_count++] = f;
            return;
        }
        if (sum_table[pos].num == fn && sum_table[pos].den == fd) return;
    }
}

/* Fraction set for membership testing */
#define FRAC_HASH_SIZE (1 << 12)
#define FRAC_HASH_MASK (FRAC_HASH_SIZE - 1)

static SHEntry frac_set_table[FRAC_HASH_SIZE];

static void frac_set_insert(Frac f) {
    i64 fn = (i64)f.num, fd = (i64)f.den;
    unsigned int idx = sh_hash(fn, fd);
    for (int i = 0; i < FRAC_HASH_SIZE; i++) {
        unsigned int pos = (idx + i) & FRAC_HASH_MASK;
        if (!frac_set_table[pos].used) {
            frac_set_table[pos].num = fn;
            frac_set_table[pos].den = fd;
            frac_set_table[pos].used = 1;
            return;
        }
        if (frac_set_table[pos].num == fn && frac_set_table[pos].den == fd) return;
    }
}

static int frac_set_contains(Frac f) {
    i64 fn = (i64)f.num, fd = (i64)f.den;
    unsigned int idx = sh_hash(fn, fd);
    for (int i = 0; i < FRAC_HASH_SIZE; i++) {
        unsigned int pos = (idx + i) & FRAC_HASH_MASK;
        if (!frac_set_table[pos].used) return 0;
        if (frac_set_table[pos].num == fn && frac_set_table[pos].den == fd) return 1;
    }
    return 0;
}

static Frac FZERO = {0, 1};
static Frac FONE = {1, 1};

static int valid_fraction(Frac r) {
    return r.den > 0 && frac_gt(r, FZERO) && frac_lt(r, FONE) &&
           r.den <= MAX_DEN && frac_set_contains(r);
}

static void print128(i128 x) {
    if (x < 0) { printf("-"); x = -x; }
    if (x > 9) print128(x / 10);
    printf("%c", (char)('0' + (int)(x % 10)));
}

int main(void) {
    nfracs = 0;
    for (int den = 1; den <= MAX_DEN; den++)
        for (int num = 1; num < den; num++) {
            i64 g = 1;
            { int a = num, b = den; while (b) { int t = b; b = a % b; a = t; } g = a; }
            if (g == 1)
                fractions[nfracs++] = frac_new(num, den);
        }

    /* Sort fractions */
    for (int i = 0; i < nfracs - 1; i++)
        for (int j = i + 1; j < nfracs; j++)
            if (frac_gt(fractions[i], fractions[j])) {
                Frac tmp = fractions[i];
                fractions[i] = fractions[j];
                fractions[j] = tmp;
            }

    memset(frac_set_table, 0, sizeof(frac_set_table));
    for (int i = 0; i < nfracs; i++)
        frac_set_insert(fractions[i]);

    for (int i = 0; i < nfracs; i++)
        sq[i] = frac_mul(fractions[i], fractions[i]);

    memset(sum_table, 0, sizeof(sum_table));
    sum_arr_count = 0;

    /* Case 1: x + y = z */
    for (int i = 0; i < nfracs; i++)
        for (int j = i; j < nfracs; j++) {
            Frac z = frac_add(fractions[i], fractions[j]);
            if (!valid_fraction(z)) continue;
            Frac s = frac_add(frac_add(fractions[i], fractions[j]), z);
            sum_insert(s);
        }

    /* Case 2: x^2 + y^2 = z^2 */
    for (int k = 0; k < nfracs; k++) {
        Frac target = sq[k];
        for (int i = 0; i < nfracs; i++) {
            Frac diff = frac_sub(target, sq[i]);
            if (diff.num <= 0) continue;
            for (int j = i; j < nfracs; j++) {
                if (frac_eq(sq[j], diff)) {
                    Frac s = frac_add(frac_add(fractions[i], fractions[j]), fractions[k]);
                    sum_insert(s);
                }
                if (frac_gt(sq[j], diff)) break;
            }
        }
    }

    /* Case 3: 1/x + 1/y = 1/z => z = xy/(x+y) */
    for (int i = 0; i < nfracs; i++)
        for (int j = i; j < nfracs; j++) {
            Frac denom = frac_add(fractions[i], fractions[j]);
            if (denom.num == 0) continue;
            Frac z = frac_div(frac_mul(fractions[i], fractions[j]), denom);
            if (!valid_fraction(z)) continue;
            Frac s = frac_add(frac_add(fractions[i], fractions[j]), z);
            sum_insert(s);
        }

    /* Case 4: 1/x^2 + 1/y^2 = 1/z^2 => z^2 = x^2*y^2/(x^2+y^2) */
    for (int i = 0; i < nfracs; i++)
        for (int j = i; j < nfracs; j++) {
            Frac denom = frac_add(sq[i], sq[j]);
            if (denom.num == 0) continue;
            Frac z_sq = frac_div(frac_mul(sq[i], sq[j]), denom);
            Frac z = frac_sqrt_val(z_sq);
            if (z.den == 0) continue;
            if (!valid_fraction(z)) continue;
            Frac s = frac_add(frac_add(fractions[i], fractions[j]), z);
            sum_insert(s);
        }

    /* Sum all distinct values using i128 fractions */
    Frac total = {0, 1};
    for (int i = 0; i < sum_arr_count; i++)
        total = frac_add(total, sum_array[i]);

    i128 result = total.num + total.den;
    print128(result);
    printf("\n");
    return 0;
}
