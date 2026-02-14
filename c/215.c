/*
 * Project Euler 215: Crack-free Walls
 *
 * Build a wall 32 wide, 10 high using 2x1 and 3x1 bricks.
 * No vertical cracks may align between adjacent rows.
 * Count the number of valid wall configurations.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define W 32
#define H 10
#define MAX_ROWS 5000

static int row_bitsets[MAX_ROWS];
static int n_rows;

static void gen_rows(int x, int bits) {
    if (x == W) {
        row_bitsets[n_rows++] = bits;
        return;
    }
    if (x + 2 <= W) {
        if (x + 2 == W)
            gen_rows(W, bits);
        else
            gen_rows(x + 2, bits | (1 << (x + 2)));
    }
    if (x + 3 <= W) {
        if (x + 3 == W)
            gen_rows(W, bits);
        else
            gen_rows(x + 3, bits | (1 << (x + 3)));
    }
}

int main(void) {
    n_rows = 0;
    gen_rows(0, 0);

    /* Build adjacency list on the heap */
    int *adj_count = calloc(n_rows, sizeof(int));
    int **adj = malloc(n_rows * sizeof(int *));
    for (int i = 0; i < n_rows; i++)
        adj[i] = malloc(n_rows * sizeof(int));

    for (int i = 0; i < n_rows; i++) {
        adj_count[i] = 0;
        for (int j = 0; j < n_rows; j++) {
            if ((row_bitsets[i] & row_bitsets[j]) == 0) {
                adj[i][adj_count[i]++] = j;
            }
        }
    }

    long long *ways = malloc(n_rows * sizeof(long long));
    long long *new_ways = malloc(n_rows * sizeof(long long));
    for (int i = 0; i < n_rows; i++) ways[i] = 1;

    for (int y = 2; y <= H; y++) {
        memset(new_ways, 0, n_rows * sizeof(long long));
        for (int i = 0; i < n_rows; i++) {
            if (ways[i] == 0) continue;
            for (int k = 0; k < adj_count[i]; k++) {
                new_ways[adj[i][k]] += ways[i];
            }
        }
        memcpy(ways, new_ways, n_rows * sizeof(long long));
    }

    long long ans = 0;
    for (int i = 0; i < n_rows; i++) ans += ways[i];

    printf("%lld\n", ans);

    for (int i = 0; i < n_rows; i++) free(adj[i]);
    free(adj); free(adj_count); free(ways); free(new_ways);
    return 0;
}
