/*
 * Project Euler Problem 572: Idempotent Matrices
 *
 * Count 3x3 idempotent matrices (A^2=A) with integer elements |x| <= N=200.
 * Uses rank decomposition: rank 0 (1), rank 1 + rank 2 (paired), rank 3 (1).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 200
#define MAXPROD (N * N)
#define PROD_OFFSET MAXPROD
#define PROD_SIZE (2 * MAXPROD + 1)

static int gcd_table[N + 1][N + 1];

#define MAX_TOTAL_PAIRS 170000

static int pair_r[MAX_TOTAL_PAIRS];
static int pair_c[MAX_TOTAL_PAIRS];
static int prod_start[PROD_SIZE];
static int prod_count[PROD_SIZE];

static int my_gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static inline int my_abs(int x) { return x < 0 ? -x : x; }

int main(void) {
    for (int i = 0; i <= N; i++)
        for (int j = 0; j <= N; j++)
            gcd_table[i][j] = my_gcd(i, j);

    memset(prod_count, 0, sizeof(prod_count));
    for (int a = -N; a <= N; a++)
        for (int b = -N; b <= N; b++)
            prod_count[a * b + PROD_OFFSET]++;

    prod_start[0] = 0;
    for (int i = 1; i < PROD_SIZE; i++)
        prod_start[i] = prod_start[i - 1] + prod_count[i - 1];

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
