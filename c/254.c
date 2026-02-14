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
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define B 10
#define LVAL 10000000  /* 10^7 */
#define NVAL 150
#define C_MAX 63       /* max digit sum of 9999999 */

/* g stored as a stack of digits */
#define MAX_G_LEN 10000100
static unsigned char *g_stack;
static int g_len;

/* Digit counts of g, and sg = digit sum of g */
static int dcnt[B];
static int sg;

/* Best for each sf value */
static int best_dcnt[C_MAX + 1][B];
static int best_len[C_MAX + 1];
static int best_sg[C_MAX + 1];
static int best_valid[C_MAX + 1];

static int is_better(int sf) {
    if (!best_valid[sf]) return 1;
    if (g_len < best_len[sf]) return 1;
    if (g_len > best_len[sf]) return 0;
    /* Same length: compare digit counts from highest digit down.
     * Fewer high digits means smaller number (since digits sorted ascending). */
    for (int d = B - 1; d >= 1; d--) {
        if (dcnt[d] < best_dcnt[sf][d]) return 1;
        if (dcnt[d] > best_dcnt[sf][d]) return 0;
    }
    return 0; /* equal */
}

static void save_best(int sf) {
    best_valid[sf] = 1;
    best_sg[sf] = sg;
    best_len[sf] = g_len;
    memcpy(best_dcnt[sf], dcnt, sizeof(dcnt));
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
    memset(dcnt, 0, sizeof(dcnt));
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
        dcnt[1]++;
        sg += 1;

        /* Consolidate: for d=2,3,...,9 while f%d==0 */
        {
            ll n = f;
            int d = 2;
            while (d < B && n % d == 0) {
                /* Remove last d characters from g */
                for (int i = 0; i < d; i++) {
                    int removed = g_stack[g_len - 1 - i];
                    dcnt[removed]--;
                    sg -= removed;
                }
                g_len -= d;
                /* Append digit d */
                g_stack[g_len++] = (unsigned char)d;
                dcnt[d]++;
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

        int sg_val = 0;
        for (int d = 9; d >= 1; d--) {
            int c = (int)(f_val / facts[d]);
            f_val -= (ll)c * facts[d];
            sg_val += d * c;
        }
        ans += sg_val;
    }

    printf("%lld\n", ans);
    free(g_stack);
    return 0;
}
