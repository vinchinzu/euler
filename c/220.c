/*
 * Project Euler 220: Heighway Dragon
 *
 * D_0 = "Fa", rules: a -> aRbFR, b -> LFaLb
 * F = forward, L = left 90, R = right 90
 * Find position after 10^12 steps in D_50.
 *
 * Precompute full-traversal summaries at each level, then recursively
 * walk with skipping of complete sub-trees.
 */
#include <stdio.h>

#define MAX_LEVEL 50

typedef long long ll;

/* Summary: (steps, dx, dy, ddir) for 'a' and 'b' at each level */
typedef struct {
    ll steps;
    ll dx, dy;
    int ddir;
} Summary;

static Summary sum_a[MAX_LEVEL + 1], sum_b[MAX_LEVEL + 1];

/* Direction: 0=up(+y), 1=left(-x), 2=down(-y), 3=right(+x) */
static int DX[] = {0, -1, 0, 1};
static int DY[] = {1, 0, -1, 0};

static void rotate(ll sx, ll sy, int d, ll *rx, ll *ry) {
    switch (d % 4) {
        case 0: *rx = sx; *ry = sy; break;
        case 1: *rx = -sy; *ry = sx; break;
        case 2: *rx = -sx; *ry = -sy; break;
        case 3: *rx = sy; *ry = -sx; break;
    }
}

static ll g_x, g_y;
static int g_d;

/* Walk expansion string at given level, taking at most 'remaining' steps.
 * string: 0 = "Fa" (top level), 1 = "aRbFR" (expansion of a), 2 = "LFaLb" (expansion of b)
 * Returns steps taken. */
static ll walk(int string_type, int level, ll remaining);

/* Process a single character */
static ll process_char(char c, int level, ll remaining) {
    if (remaining <= 0) return 0;

    if (c == 'F') {
        g_x += DX[g_d];
        g_y += DY[g_d];
        return 1;
    } else if (c == 'L') {
        g_d = (g_d + 1) & 3;
        return 0;
    } else if (c == 'R') {
        g_d = (g_d + 3) & 3;
        return 0;
    } else {
        /* 'a' or 'b' */
        if (level <= 0) return 0;

        Summary *s = (c == 'a') ? &sum_a[level - 1] : &sum_b[level - 1];
        if (s->steps <= remaining) {
            /* Take full sub-path */
            ll rx, ry;
            rotate(s->dx, s->dy, g_d, &rx, &ry);
            g_x += rx;
            g_y += ry;
            g_d = (g_d + s->ddir) & 3;
            return s->steps;
        } else {
            /* Partial: expand one level deeper */
            int expansion = (c == 'a') ? 1 : 2;
            return walk(expansion, level - 1, remaining);
        }
    }
}

static ll walk(int string_type, int level, ll remaining) {
    ll taken = 0;

    /* String sequences */
    /* 0: "Fa" */
    /* 1: "aRbFR" */
    /* 2: "LFaLb" */

    static const char *strings[] = {"Fa", "aRbFR", "LFaLb"};
    const char *str = strings[string_type];

    for (int i = 0; str[i] && taken < remaining; i++) {
        taken += process_char(str[i], level, remaining - taken);
    }
    return taken;
}

int main(void) {
    ll N = 1000000000000LL;

    /* Level 0: a and b are no-ops */
    sum_a[0] = (Summary){0, 0, 0, 0};
    sum_b[0] = (Summary){0, 0, 0, 0};

    for (int lev = 1; lev <= MAX_LEVEL; lev++) {
        /* a -> aRbFR */
        {
            ll steps = 0, dx = 0, dy = 0;
            int d = 0;
            ll rx, ry;

            /* 'a' at level-1 */
            Summary *sa = &sum_a[lev - 1];
            rotate(sa->dx, sa->dy, d, &rx, &ry);
            dx += rx; dy += ry; d = (d + sa->ddir) & 3; steps += sa->steps;

            /* R */
            d = (d + 3) & 3;

            /* 'b' at level-1 */
            Summary *sb = &sum_b[lev - 1];
            rotate(sb->dx, sb->dy, d, &rx, &ry);
            dx += rx; dy += ry; d = (d + sb->ddir) & 3; steps += sb->steps;

            /* F */
            dx += DX[d]; dy += DY[d]; steps++;

            /* R */
            d = (d + 3) & 3;

            sum_a[lev] = (Summary){steps, dx, dy, d};
        }

        /* b -> LFaLb */
        {
            ll steps = 0, dx = 0, dy = 0;
            int d = 0;
            ll rx, ry;

            /* L */
            d = (d + 1) & 3;

            /* F */
            dx += DX[d]; dy += DY[d]; steps++;

            /* 'a' at level-1 */
            Summary *sa = &sum_a[lev - 1];
            rotate(sa->dx, sa->dy, d, &rx, &ry);
            dx += rx; dy += ry; d = (d + sa->ddir) & 3; steps += sa->steps;

            /* L */
            d = (d + 1) & 3;

            /* 'b' at level-1 */
            Summary *sb = &sum_b[lev - 1];
            rotate(sb->dx, sb->dy, d, &rx, &ry);
            dx += rx; dy += ry; d = (d + sb->ddir) & 3; steps += sb->steps;

            sum_b[lev] = (Summary){steps, dx, dy, d};
        }
    }

    g_x = 0; g_y = 0; g_d = 0;
    walk(0, MAX_LEVEL, N);

    printf("%lld,%lld\n", g_x, g_y);
    return 0;
}
