/*
 * Project Euler Problem 315: Digital root clocks
 *
 * Sam's clock clears display between terms; Max's keeps common segments lit.
 * Feed all primes p in [10^7, 2*10^7] and return Sam_transitions - Max_transitions.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* 7-segment digit masks */
#define TOP       (1<<0)
#define TOP_LEFT  (1<<1)
#define TOP_RIGHT (1<<2)
#define MIDDLE    (1<<3)
#define BOT_LEFT  (1<<4)
#define BOT_RIGHT (1<<5)
#define BOTTOM    (1<<6)

static const int DIGIT_MASK[10] = {
    TOP|TOP_LEFT|TOP_RIGHT|BOT_LEFT|BOT_RIGHT|BOTTOM,        /* 0 */
    TOP_RIGHT|BOT_RIGHT,                                       /* 1 */
    TOP|TOP_RIGHT|MIDDLE|BOT_LEFT|BOTTOM,                      /* 2 */
    TOP|TOP_RIGHT|MIDDLE|BOT_RIGHT|BOTTOM,                     /* 3 */
    TOP_LEFT|TOP_RIGHT|MIDDLE|BOT_RIGHT,                       /* 4 */
    TOP|TOP_LEFT|MIDDLE|BOT_RIGHT|BOTTOM,                      /* 5 */
    TOP|TOP_LEFT|MIDDLE|BOT_LEFT|BOT_RIGHT|BOTTOM,             /* 6 */
    TOP|TOP_LEFT|TOP_RIGHT|BOT_RIGHT,                           /* 7 */
    TOP|TOP_LEFT|TOP_RIGHT|MIDDLE|BOT_LEFT|BOT_RIGHT|BOTTOM,   /* 8 */
    TOP|TOP_LEFT|TOP_RIGHT|MIDDLE|BOT_RIGHT|BOTTOM             /* 9 */
};

static int seg_count[10];
static int diff_count[10][10];

static int popcount(int x) {
    int c = 0;
    while (x) { c += x & 1; x >>= 1; }
    return c;
}

static void init_tables(void) {
    for (int i = 0; i < 10; i++)
        seg_count[i] = popcount(DIGIT_MASK[i]);
    for (int i = 0; i < 10; i++)
        for (int j = 0; j < 10; j++)
            diff_count[i][j] = popcount(DIGIT_MASK[i] ^ DIGIT_MASK[j]);
}

/* Get digits of a number (LSD first), return segment count and digit sum */
static int get_info(int val, int *digits, int *ndigits, int *dsum) {
    int segs = 0;
    *ndigits = 0;
    *dsum = 0;
    if (val == 0) {
        digits[0] = 0;
        *ndigits = 1;
        return seg_count[0];
    }
    while (val > 0) {
        int d = val % 10;
        digits[*ndigits] = d;
        (*ndigits)++;
        segs += seg_count[d];
        *dsum += d;
        val /= 10;
    }
    return segs;
}

static int transitions_between(int *prev, int nprev, int *curr, int ncurr) {
    int total = 0;
    int shared = nprev < ncurr ? nprev : ncurr;
    for (int i = 0; i < shared; i++)
        total += diff_count[prev[i]][curr[i]];
    for (int i = shared; i < nprev; i++)
        total += seg_count[prev[i]];
    for (int i = shared; i < ncurr; i++)
        total += seg_count[curr[i]];
    return total;
}

static int sam_minus_max(int value) {
    int digits[10], ndigits, dsum;
    int segs = get_info(value, digits, &ndigits, &dsum);

    int sam_total = 2 * segs;
    int max_total = segs;

    int prev_digits[10], nprev;
    for (int i = 0; i < ndigits; i++) prev_digits[i] = digits[i];
    nprev = ndigits;
    int prev_segs = segs;

    int current = value;
    int next_val = dsum;

    while (current >= 10) {
        current = next_val;
        segs = get_info(current, digits, &ndigits, &dsum);
        sam_total += 2 * segs;
        max_total += transitions_between(prev_digits, nprev, digits, ndigits);
        for (int i = 0; i < ndigits; i++) prev_digits[i] = digits[i];
        nprev = ndigits;
        prev_segs = segs;
        next_val = dsum;
    }

    max_total += prev_segs;
    return sam_total - max_total;
}

#define LOWER 10000000
#define UPPER 20000000

int main(void) {
    init_tables();

    /* Segmented sieve for primes in [LOWER, UPPER] */
    int root = (int)sqrt((double)UPPER) + 1;

    /* Small primes sieve */
    char *small_sieve = malloc(root + 1);
    memset(small_sieve, 1, root + 1);
    small_sieve[0] = small_sieve[1] = 0;
    for (int i = 2; i * i <= root; i++) {
        if (small_sieve[i]) {
            for (int j = i * i; j <= root; j += i)
                small_sieve[j] = 0;
        }
    }

    int *small_primes = malloc(root * sizeof(int));
    int nsp = 0;
    for (int i = 2; i <= root; i++)
        if (small_sieve[i]) small_primes[nsp++] = i;
    free(small_sieve);

    /* Process in segments */
    int seg_size = 1000000;
    long long total = 0;

    for (int low = LOWER; low <= UPPER; low += seg_size) {
        int high = low + seg_size - 1;
        if (high > UPPER) high = UPPER;
        int size = high - low + 1;

        char *seg = malloc(size);
        memset(seg, 1, size);

        for (int i = 0; i < nsp; i++) {
            int p = small_primes[i];
            int start = ((low + p - 1) / p) * p;
            if (start < p * p) start = p * p;
            if (start > high) continue;
            for (int j = start - low; j < size; j += p)
                seg[j] = 0;
        }

        for (int i = 0; i < size; i++) {
            if (seg[i]) {
                total += sam_minus_max(low + i);
            }
        }

        free(seg);
    }

    printf("%lld\n", total);
    free(small_primes);
    return 0;
}
