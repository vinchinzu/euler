/*
 * Project Euler 662 - Fibonacci Paths
 *
 * Count paths from (0,0) to (N,N) on a lattice where each step (dx,dy)
 * has dx >= 0, dy >= 0, (dx,dy) != (0,0), and sqrt(dx^2+dy^2) is a
 * positive Fibonacci number. Answer mod 10^9+7 for N=10000.
 *
 * 2D DP with all valid jump vectors. Process row by row.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef unsigned int u32;
typedef long long ll;

int main() {
    int N = 10000;
    u32 MOD = 1000000007u;

    /* Generate Fibonacci numbers */
    int max_fib_limit = (int)sqrt(2.0 * N * N) + 2;
    int fib_set[100];
    int nfibs = 0;
    int fa = 1, fb = 1;
    while (fa <= max_fib_limit) {
        fib_set[nfibs++] = fa;
        int tmp = fa + fb;
        fa = fb;
        fb = tmp;
    }

    /* Generate all valid jump vectors */
    int h_jumps[200], nh = 0;
    int v_jumps[200], nv = 0;
    int ddx[2000], ddy[2000];
    int nd = 0;

    /* Use a set to avoid duplicates */
    char h_seen[200001] = {0};
    char v_seen[200001] = {0};
    /* For diagonal, store (dx, dy) pairs */

    for (int fi = 0; fi < nfibs; fi++) {
        int f = fib_set[fi];
        for (int dx = 0; dx <= f && dx <= N; dx++) {
            ll dy2 = (ll)f * f - (ll)dx * dx;
            int dy = (int)sqrt((double)dy2);
            /* Check both dy and dy+1 in case of floating point */
            for (int ddy_try = (dy > 0 ? dy - 1 : 0); ddy_try <= dy + 1; ddy_try++) {
                if ((ll)ddy_try * ddy_try == dy2 && ddy_try >= 0 && ddy_try <= N) {
                    if (dx > 0 && ddy_try == 0) {
                        if (!h_seen[dx]) {
                            h_seen[dx] = 1;
                            h_jumps[nh++] = dx;
                        }
                    } else if (dx == 0 && ddy_try > 0) {
                        if (!v_seen[ddy_try]) {
                            v_seen[ddy_try] = 1;
                            v_jumps[nv++] = ddy_try;
                        }
                    } else if (dx > 0 && ddy_try > 0) {
                        ddx[nd] = dx;
                        ddy[nd] = ddy_try;
                        nd++;
                    }
                }
            }
        }
    }

    int W = N + 1;

    /* Allocate dp grid: (N+1) rows of (N+1) columns */
    u32 **dp = (u32 **)malloc(W * sizeof(u32 *));
    for (int y = 0; y < W; y++) {
        dp[y] = (u32 *)calloc(W, sizeof(u32));
    }
    dp[0][0] = 1;

    for (int y = 0; y < W; y++) {
        /* Step 1: Add vert and diagonal contributions to dp[y] */
        for (int vi = 0; vi < nv; vi++) {
            int dyv = v_jumps[vi];
            if (y < dyv) continue;
            u32 *src = dp[y - dyv];
            u32 *dst = dp[y];
            for (int x = 0; x < W; x++) {
                u32 v = dst[x] + src[x];
                dst[x] = v >= MOD ? v - MOD : v;
            }
        }

        for (int di = 0; di < nd; di++) {
            int dx = ddx[di], dyv = ddy[di];
            if (y < dyv) continue;
            u32 *src = dp[y - dyv];
            u32 *dst = dp[y];
            for (int x = dx; x < W; x++) {
                u32 v = dst[x] + src[x - dx];
                dst[x] = v >= MOD ? v - MOD : v;
            }
        }

        /* Step 2: Process horizontal jumps within the same row. */
        {
            u32 *row = dp[y];
            for (int x = 1; x < W; x++) {
                u32 sum = 0;
                for (int hi = 0; hi < nh; hi++) {
                    int dx = h_jumps[hi];
                    if (dx > x) continue;
                    sum += row[x - dx];
                    if (sum >= MOD) sum -= MOD;
                }
                row[x] += sum;
                if (row[x] >= MOD) row[x] -= MOD;
            }
        }
    }

    printf("%u\n", dp[N][N]);

    for (int y = 0; y < W; y++) free(dp[y]);
    free(dp);
    return 0;
}
