"""Project Euler Problem 662: Fibonacci Paths.

Count paths from (0,0) to (N,N) on a lattice where each step (dx,dy)
has dx >= 0, dy >= 0, (dx,dy) != (0,0), and sqrt(dx^2+dy^2) is a
positive Fibonacci number. Answer mod 10^9+7 for N=10000.

Approach: 2D DP with all valid jump vectors. Process row by row.
Uses C for speed.
"""

import subprocess, tempfile, os
from math import isqrt

def solve():
    N = 10000
    MOD = 10**9 + 7

    # Generate Fibonacci numbers
    max_fib = isqrt(2 * N * N) + 1
    fibs = set()
    a, b = 1, 1
    while a <= max_fib:
        fibs.add(a)
        a, b = b, a + b

    # Generate all valid jump vectors
    jumps = []
    for f in sorted(fibs):
        if f > max_fib:
            break
        for dx in range(min(f, N) + 1):
            dy2 = f * f - dx * dx
            dy = isqrt(dy2)
            if dy * dy == dy2 and 0 <= dy <= N:
                if dx > 0 or dy > 0:
                    jumps.append((dx, dy))

    # Separate into horizontal, vertical, and diagonal jumps
    h_jumps = sorted(set(dx for dx, dy in jumps if dy == 0 and dx > 0))
    v_jumps = sorted(set(dy for dx, dy in jumps if dx == 0 and dy > 0))
    d_jumps = sorted([(dx, dy) for dx, dy in jumps if dx > 0 and dy > 0])

    # Build input for C
    lines = []
    lines.append(f"{N} {MOD}")
    lines.append(f"{len(h_jumps)} " + " ".join(str(j) for j in h_jumps))
    lines.append(f"{len(v_jumps)} " + " ".join(str(j) for j in v_jumps))
    lines.append(f"{len(d_jumps)}")
    for dx, dy in d_jumps:
        lines.append(f"{dx} {dy}")
    input_data = "\n".join(lines) + "\n"

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned int u32;
typedef long long ll;

int main() {
    int N;
    u32 MOD;
    scanf("%d %u", &N, &MOD);

    int nh, nv, nd;
    scanf("%d", &nh);
    int *hj = (int *)malloc(nh * sizeof(int));
    for (int i = 0; i < nh; i++) scanf("%d", &hj[i]);

    scanf("%d", &nv);
    int *vj = (int *)malloc(nv * sizeof(int));
    for (int i = 0; i < nv; i++) scanf("%d", &vj[i]);

    scanf("%d", &nd);
    int *ddx = (int *)malloc(nd * sizeof(int));
    int *ddy = (int *)malloc(nd * sizeof(int));
    for (int i = 0; i < nd; i++) scanf("%d %d", &ddx[i], &ddy[i]);

    int W = N + 1;

    /* Allocate dp grid: (N+1) rows of (N+1) columns */
    u32 **dp = (u32 **)malloc(W * sizeof(u32 *));
    for (int y = 0; y < W; y++) {
        dp[y] = (u32 *)calloc(W, sizeof(u32));
    }
    dp[0][0] = 1;

    /* For horizontal jumps within the same row, we need a different approach.
       We use a temporary row to accumulate vert+diag contributions, then
       process horizontal contributions iteratively. */

    for (int y = 0; y < W; y++) {
        /* Step 1: Add vert and diagonal contributions to dp[y] */
        /* These all come from completed earlier rows, so order doesn't matter */
        for (int vi = 0; vi < nv; vi++) {
            int dy = vj[vi];
            if (y < dy) continue;
            u32 *src = dp[y - dy];
            u32 *dst = dp[y];
            for (int x = 0; x < W; x++) {
                u32 v = dst[x] + src[x];
                dst[x] = v >= MOD ? v - MOD : v;
            }
        }

        for (int di = 0; di < nd; di++) {
            int dx = ddx[di], dy = ddy[di];
            if (y < dy) continue;
            u32 *src = dp[y - dy];
            u32 *dst = dp[y];
            for (int x = dx; x < W; x++) {
                u32 v = dst[x] + src[x - dx];
                dst[x] = v >= MOD ? v - MOD : v;
            }
        }

        /* Step 2: Process horizontal jumps within the same row.
           For cell x, dp[x][y] += sum over horiz dx: dp[x-dx][y].
           Process x from left to right. dp[x-dx][y] includes all contributions
           (vert, diag, and horizontal chains from earlier cells).
           Process all horizontal jumps per cell to avoid the sequential-per-jump bug. */
        {
            u32 *row = dp[y];
            for (int x = 1; x < W; x++) {
                u32 sum = 0;
                for (int hi = 0; hi < nh; hi++) {
                    int dx = hj[hi];
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
    free(hj); free(vj); free(ddx); free(ddy);
    return 0;
}
"""

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        r = subprocess.run(["gcc", "-O2", "-o", exe, src], capture_output=True, text=True)
        if r.returncode != 0:
            import sys; sys.stderr.write("Compile: " + r.stderr + "\n"); return -1
        result = subprocess.run([exe], input=input_data, capture_output=True, text=True, timeout=28)
        if result.returncode != 0:
            import sys; sys.stderr.write("Runtime: " + result.stderr + "\n"); return -1
        return int(result.stdout.strip())


if __name__ == "__main__":
    print(solve())
