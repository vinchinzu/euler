/*
 * Project Euler 213: Flea Circus
 *
 * 30x30 grid with 1 flea per square. Each step, each flea jumps to a
 * random orthogonal neighbor. Find expected empty squares after 50 steps.
 *
 * For each flea starting at (fi,fj), compute the probability distribution
 * after 50 steps, then for each target cell (i,j), prob(empty) =
 * product over all fleas of (1 - prob(flea at (i,j))).
 *
 * Exploit 4-fold symmetry: only compute for fleas in top-left quadrant.
 */
#include <stdio.h>
#include <string.h>

#define SZ 30
#define STEPS 50

static double grid[SZ][SZ], new_grid[SZ][SZ];
/* table[fi][fj][i][j] = prob flea starting at (fi,fj) is at (i,j) after STEPS */
/* Only store for fi < SZ/2, fj < SZ/2 */
static double table[SZ/2][SZ/2][SZ][SZ];

static int neighbors(int i, int j) {
    int n = 0;
    if (i > 0) n++;
    if (i < SZ-1) n++;
    if (j > 0) n++;
    if (j < SZ-1) n++;
    return n;
}

int main(void) {
    int half = SZ / 2;

    for (int fi = 0; fi < half; fi++) {
        for (int fj = 0; fj < half; fj++) {
            memset(grid, 0, sizeof(grid));
            grid[fi][fj] = 1.0;

            for (int step = 0; step < STEPS; step++) {
                memset(new_grid, 0, sizeof(new_grid));
                for (int i = 0; i < SZ; i++) {
                    for (int j = 0; j < SZ; j++) {
                        if (grid[i][j] == 0.0) continue;
                        double p = grid[i][j] / neighbors(i, j);
                        if (i > 0) new_grid[i-1][j] += p;
                        if (i < SZ-1) new_grid[i+1][j] += p;
                        if (j > 0) new_grid[i][j-1] += p;
                        if (j < SZ-1) new_grid[i][j+1] += p;
                    }
                }
                memcpy(grid, new_grid, sizeof(grid));
            }

            memcpy(table[fi][fj], grid, sizeof(grid));
        }
    }

    double ans = 0.0;
    for (int i = 0; i < SZ; i++) {
        for (int j = 0; j < SZ; j++) {
            double prob = 1.0;
            for (int fi = 0; fi < half; fi++) {
                for (int fj = 0; fj < half; fj++) {
                    prob *= (1.0 - table[fi][fj][i][j])
                          * (1.0 - table[fi][fj][i][SZ-1-j])
                          * (1.0 - table[fi][fj][SZ-1-i][j])
                          * (1.0 - table[fi][fj][SZ-1-i][SZ-1-j]);
                }
            }
            ans += prob;
        }
    }

    printf("%.6f\n", ans);
    return 0;
}
