/*
 * Project Euler Problem 254: Sums of Digit Factorials
 *
 * For i in 1..150, g(i) is the smallest positive integer n such that
 * the sum of digits of the sum of factorials of digits of n equals i.
 * Find sum of sg(g(i)) where sg = digit sum.
 *
 * The greedy factoriadic decomposition of f gives g: the sorted-ascending
 * number whose digit-factorial-sum equals f. We maintain g incrementally
 * using a stack, tracking digit counts for is_better comparisons.
 *
 * The g string stores digits in descending order (highest digit first).
 * To compare two g strings of same length, we compare from the END
 * (least significant digit), matching Python's comparison logic.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define B 10
#define LVAL 10000000  /* 10^7 */
#define NVAL 150
#define C_MAX 63       /* max digit sum of 9999999 */

/* g stored as a stack of digits (descending order: highest digit at index 0) */
#define MAX_G_LEN 10000100
static unsigned char *g_stack;
static int g_len;

/* sg = digit sum of g */
static int sg;

/* Best for each sf value */
#define MAX_BEST_LEN 200  /* g strings are at most ~40 digits */
static unsigned char best_g[C_MAX + 1][MAX_BEST_LEN];
static int best_len[C_MAX + 1];
static int best_sg[C_MAX + 1];
static int best_valid[C_MAX + 1];

static int is_better(int sf) {
    if (!best_valid[sf]) return 1;
    if (g_len < best_len[sf]) return 1;
    if (g_len > best_len[sf]) return 0;
    /* Same length: compare from the end (least significant digit first).
     * The string is in descending order, so the last char is the smallest digit.
     * Smaller at the end => number is smaller => better. */
    for (int i = g_len - 1; i >= 0; i--) {
        if (g_stack[i] < best_g[sf][i]) return 1;
        if (g_stack[i] > best_g[sf][i]) return 0;
    }
    return 0; /* equal */
}

static void save_best(int sf) {
    best_valid[sf] = 1;
    best_sg[sf] = sg;
    best_len[sf] = g_len;
    if (g_len <= MAX_BEST_LEN)
        memcpy(best_g[sf], g_stack, g_len);
}

static ll factorial(int n) {
    ll r = 1;
    for (int i = 2; i <= n; i++) r *= i;
    return r;
}

int main(void) {
    g_stack = (unsigned char*)malloc(MAX_G_LEN);
    g_len = 0;
    sg = 0;
    memset(best_valid, 0, sizeof(best_valid));

    int sf = 0;

    for (ll f = 1; f < LVAL; f++) {
        /* Update sf: digit sum of f */
        sf += 1;
        {
            ll n = f;
            while (n % B == 0) {
                sf -= B - 1;
                n /= B;
            }
        }

        /* Append "1" to g */
        g_stack[g_len++] = 1;
        sg += 1;

        /* Consolidate: for d=2,3,...,9 while f%d==0 */
        {
            ll n = f;
            int d = 2;
            while (d < B && n % d == 0) {
                /* Remove last d characters from g, add digit d */
                for (int i = 0; i < d; i++) {
                    sg -= g_stack[g_len - 1 - i];
                }
                g_len -= d;
                g_stack[g_len++] = (unsigned char)d;
                sg += d;

                n /= d;
                d++;
            }
        }

        if (sf >= 1 && sf <= C_MAX) {
            if (is_better(sf)) {
                save_best(sf);
            }
        }
    }

    ll ans = 0;
    for (int i = 1; i <= C_MAX && i <= NVAL; i++) {
        if (best_valid[i])
            ans += best_sg[i];
    }

    /* For sf > C_MAX (64..150) */
    ll facts[10];
    for (int i = 0; i <= 9; i++) facts[i] = factorial(i);

    for (int sf2 = C_MAX + 1; sf2 <= NVAL; sf2++) {
        ll f_val = ((ll)(sf2 % (B-1)) + 1);
        ll power = 1;
        for (int i = 0; i < sf2 / (B-1); i++) power *= B;
        f_val = f_val * power - 1;

        ll sg_val = 0;
        for (int d = 9; d >= 1; d--) {
            ll c = f_val / facts[d];
            f_val -= c * facts[d];
            sg_val += (ll)d * c;
        }
        ans += sg_val;
    }

    printf("%lld\n", ans);
    free(g_stack);
    return 0;
}
