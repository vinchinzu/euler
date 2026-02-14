/*
 * Project Euler 879 - Touch Screen Passwords on 4x4 Grid
 *
 * Count all passwords (sequences of 2+ distinct spots) on a 4x4 grid,
 * where intermediate spots on a line are auto-included unless already used.
 *
 * Algorithm: DFS with bitmask DP. 16 points, 2^16 = 65536 states.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NUM_POINTS 16
#define WIDTH 4
#define HEIGHT 4

static int inters[NUM_POINTS][NUM_POINTS]; /* bitmask of intermediates */

static int gcd(int a, int b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static void compute_intermediates(int p1, int p2) {
    int r1 = p1 / WIDTH, c1 = p1 % WIDTH;
    int r2 = p2 / WIDTH, c2 = p2 % WIDTH;
    int dx = r2 - r1, dy = c2 - c1;
    int g = gcd(dx, dy);
    int mask = 0;
    if (g > 1) {
        int sx = dx / g, sy = dy / g;
        for (int k = 1; k < g; k++) {
            int mr = r1 + k * sx;
            int mc = c1 + k * sy;
            int idx = mr * WIDTH + mc;
            mask |= (1 << idx);
        }
    }
    inters[p1][p2] = mask;
}

/* DP memo: memo[current][used_mask] */
/* With 16 points and 2^16 masks, we need 16 * 65536 = 1048576 entries */
static long long memo[NUM_POINTS][1 << NUM_POINTS];
static char memo_set[NUM_POINTS][1 << NUM_POINTS];

static long long dfs(int current, int used_mask) {
    if (memo_set[current][used_mask])
        return memo[current][used_mask];

    long long count = 1; /* Current path is valid */

    for (int next = 0; next < NUM_POINTS; next++) {
        if (used_mask & (1 << next))
            continue;
        int req = inters[current][next];
        if ((req & used_mask) == req) {
            count += dfs(next, used_mask | (1 << next));
        }
    }

    memo[current][used_mask] = count;
    memo_set[current][used_mask] = 1;
    return count;
}

int main(void) {
    /* Precompute intermediates */
    for (int i = 0; i < NUM_POINTS; i++)
        for (int j = 0; j < NUM_POINTS; j++)
            compute_intermediates(i, j);

    memset(memo_set, 0, sizeof(memo_set));

    long long total = 0;
    for (int start = 0; start < NUM_POINTS; start++) {
        /* Subtract 1 because we need sequences of 2+ spots */
        total += dfs(start, 1 << start) - 1;
    }

    printf("%lld\n", total);
    return 0;
}
