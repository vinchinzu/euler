"""Project Euler Problem 572: Idempotent Matrices.

Find the number of 3x3 idempotent matrices (A²=A) with integer elements with
absolute value at most N.

First we consider the rank of the matrix A:
- There is only one rank zero matrix: the matrix of all zeros. It is idempotent.
- If the rank is 1, then let A = [r s t]^T [x y z]. Note that the equations
  equating A²=A element-wise all reduce to the requirement that the trace
  r*x + s*y + t*z = 1. So we iterate over all possible triplets (r*x, s*y, t*z)
  summing to 1, and iterate over all possible ways to split each product into
  two factors. Note that at the end we need to divide by 2, because changing
  the sign of (r,s,t) is the same as changing the sign of (x,y,z).
- If the rank is 2, note that I-A is idempotent because (I-A)² = I²-2A+A² =
  I-A, and I-A has rank 1. So for each rank 1 idempotent matrix we find above,
  we also get a rank 2 idempotent matrix. The only issue is that the bounds are
  slightly changed; the rank 1 matrix cannot have -N in the diagonal, but can
  have N+1 in the diagonal.
- If the rank is 3, then A is invertible, which means A²=A => A=I, the identity
  matrix.

So we count the valid rank 1 and rank 2 matrices, divide by 2, and add 2 for
the all-zero matrix and identity matrix.

Uses embedded C for performance (6-deep nested loops).
"""

from __future__ import annotations

import os
import subprocess
import tempfile


C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 200
#define MAXPROD (N * N)
/* Products range from -MAXPROD to +MAXPROD. We index by product + MAXPROD. */
#define PROD_OFFSET MAXPROD
#define PROD_SIZE (2 * MAXPROD + 1)

/* GCD table for 0..N */
static int gcd_table[N + 1][N + 1];

/*
 * Divisor pairs stored in flat arrays with index table.
 * Total pairs: 401 * 401 = 160801 (each (a,b) in [-N,N]^2 produces one product).
 * We store all pairs contiguously and use start/count arrays indexed by product.
 */
#define MAX_TOTAL_PAIRS 170000

static int pair_r[MAX_TOTAL_PAIRS];  /* first factor */
static int pair_c[MAX_TOTAL_PAIRS];  /* second factor */
static int prod_start[PROD_SIZE];    /* start index in pair arrays */
static int prod_count[PROD_SIZE];    /* number of pairs for this product */

static int my_gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static inline int my_abs(int x) { return x < 0 ? -x : x; }

int main(void) {
    /* Precompute GCD table */
    for (int i = 0; i <= N; i++)
        for (int j = 0; j <= N; j++)
            gcd_table[i][j] = my_gcd(i, j);

    /* Count pairs per product first */
    memset(prod_count, 0, sizeof(prod_count));
    for (int a = -N; a <= N; a++)
        for (int b = -N; b <= N; b++)
            prod_count[a * b + PROD_OFFSET]++;

    /* Compute start indices (prefix sum) */
    prod_start[0] = 0;
    for (int i = 1; i < PROD_SIZE; i++)
        prod_start[i] = prod_start[i - 1] + prod_count[i - 1];

    /* Fill pairs using a temporary write-position array */
    int *wpos = (int *)malloc(PROD_SIZE * sizeof(int));
    memcpy(wpos, prod_start, PROD_SIZE * sizeof(int));
    for (int a = -N; a <= N; a++) {
        for (int b = -N; b <= N; b++) {
            int idx = a * b + PROD_OFFSET;
            int w = wpos[idx];
            pair_r[w] = a;
            pair_c[w] = b;
            wpos[idx] = w + 1;
        }
    }
    free(wpos);

    long long ans = 0;

    for (int a_val = -N; a_val <= N + 1; a_val++) {
        for (int e_val = -N; e_val <= N + 1; e_val++) {
            int i_val = 1 - a_val - e_val;
            if (i_val < -N || i_val > N + 1)
                continue;

            /* Check if this (a_val, e_val, i_val) can contribute to rank1 or rank2 */
            int can_rank1 = (a_val <= N && e_val <= N && i_val <= N);
            int can_rank2 = (a_val > -N && e_val > -N && i_val > -N);
            if (!can_rank1 && !can_rank2)
                continue;

            int inc = can_rank1 + can_rank2;

            int pa = a_val + PROD_OFFSET;
            int pe = e_val + PROD_OFFSET;
            int pi_idx = i_val + PROD_OFFSET;

            int na = prod_count[pa];
            int ne = prod_count[pe];
            int ni = prod_count[pi_idx];

            if (na == 0 || ne == 0 || ni == 0)
                continue;

            int sa = prod_start[pa];
            int se = prod_start[pe];
            int si = prod_start[pi_idx];

            for (int j1 = sa; j1 < sa + na; j1++) {
                int r = pair_r[j1];
                int x = pair_c[j1];

                for (int j2 = se; j2 < se + ne; j2++) {
                    int s = pair_r[j2];
                    int y = pair_c[j2];

                    /* Cross product bounds check */
                    if (my_abs(r * y) > N || my_abs(s * x) > N)
                        continue;

                    for (int j3 = si; j3 < si + ni; j3++) {
                        int t = pair_r[j3];
                        int z = pair_c[j3];

                        if (my_abs(r * z) > N ||
                            my_abs(s * z) > N ||
                            my_abs(t * x) > N ||
                            my_abs(t * y) > N)
                            continue;

                        /* GCD check: gcd(gcd(|r|,|s|),|t|) must be 1 */
                        if (gcd_table[gcd_table[my_abs(r)][my_abs(s)]][my_abs(t)] != 1)
                            continue;

                        ans += inc;
                    }
                }
            }
        }
    }

    ans = ans / 2 + 2;
    printf("%lld\n", ans);
    return 0;
}
"""


def solve() -> int:
    """Solve Problem 572 using embedded C."""
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p572.c")
        exe = os.path.join(tmpdir, "p572")

        with open(src, "w") as f:
            f.write(C_CODE)

        # Compile with optimizations
        subprocess.run(
            ["gcc", "-O2", "-o", exe, src, "-lm"],
            check=True,
            capture_output=True,
        )

        # Run with timeout
        result = subprocess.run(
            [exe],
            capture_output=True,
            text=True,
            timeout=280,
        )

        return int(result.stdout.strip())


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
