/*
 * Project Euler Problem 393: Ant Migration
 *
 * Count ways for n^2 ants on n x n grid to move to adjacent squares
 * such that no swaps and no collisions.
 * DP over grid cells with flow state.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define GRID 10

/* Zero-flow combinations: for each (top, left), list of (bottom, right)
 * such that sorted {top, left, bottom, right} = {-1, 0, 0, 1} */
typedef struct { int bottom, right; } FlowOption;

static FlowOption zero_flows[9][4]; /* indexed by (top+1)*3 + (left+1) */
static int nflows[9];

static void init_flows(void) {
    memset(nflows, 0, sizeof(nflows));
    for (int top = -1; top <= 1; top++) {
        for (int left = -1; left <= 1; left++) {
            int idx = (top + 1) * 3 + (left + 1);
            for (int bottom = -1; bottom <= 1; bottom++) {
                for (int right = -1; right <= 1; right++) {
                    int f[4] = {top, left, bottom, right};
                    /* Sort */
                    for (int i = 0; i < 4; i++)
                        for (int j = i + 1; j < 4; j++)
                            if (f[i] > f[j]) { int t = f[i]; f[i] = f[j]; f[j] = t; }
                    if (f[0] == -1 && f[1] == 0 && f[2] == 0 && f[3] == 1) {
                        int n = nflows[idx];
                        zero_flows[idx][n].bottom = bottom;
                        zero_flows[idx][n].right = right;
                        nflows[idx]++;
                    }
                }
            }
        }
    }
}

/* State: tuple of GRID top-flows + left_flow -> count */
/* top_flows is an array of GRID values in {-1, 0, 1} */
/* Encode: each flow value in {-1,0,1} mapped to {0,1,2}, 3^GRID * 3 states */

/* 3^10 = 59049, times 3 for left_flow = 177147 */
#define STATE_SIZE (59049 * 3)

static long long counts[STATE_SIZE];
static long long new_counts[STATE_SIZE];

static int encode_state(int *top_flows, int left_flow) {
    int key = 0;
    for (int i = 0; i < GRID; i++) {
        key = key * 3 + (top_flows[i] + 1);
    }
    key = key * 3 + (left_flow + 1);
    return key;
}

static void decode_state(int key, int *top_flows, int *left_flow) {
    *left_flow = (key % 3) - 1;
    key /= 3;
    for (int i = GRID - 1; i >= 0; i--) {
        top_flows[i] = (key % 3) - 1;
        key /= 3;
    }
}

int main(void) {
    init_flows();

    memset(counts, 0, sizeof(counts));

    /* Initial state: no flows */
    int init_top[GRID];
    memset(init_top, 0, sizeof(init_top));
    counts[encode_state(init_top, 0)] = 1;

    for (int row = 0; row < GRID; row++) {
        for (int col = 0; col < GRID; col++) {
            memset(new_counts, 0, sizeof(new_counts));

            for (int s = 0; s < STATE_SIZE; s++) {
                if (counts[s] == 0) continue;

                int top_flows[GRID];
                int left_flow;
                decode_state(s, top_flows, &left_flow);

                int top_flow = top_flows[0];
                int fidx = (top_flow + 1) * 3 + (left_flow + 1);

                for (int fi = 0; fi < nflows[fidx]; fi++) {
                    int bottom_flow = zero_flows[fidx][fi].bottom;
                    int right_flow = zero_flows[fidx][fi].right;

                    /* Check: rightmost column must have right_flow = 0 */
                    if (col == GRID - 1 && right_flow != 0) continue;

                    /* Update state: shift top_flows and add bottom */
                    int new_top[GRID];
                    for (int i = 0; i < GRID - 1; i++)
                        new_top[i] = top_flows[i + 1];
                    new_top[GRID - 1] = bottom_flow;

                    new_counts[encode_state(new_top, right_flow)] += counts[s];
                }
            }

            memcpy(counts, new_counts, sizeof(counts));
        }
    }

    /* Answer */
    int final_top[GRID];
    memset(final_top, 0, sizeof(final_top));
    printf("%lld\n", counts[encode_state(final_top, 0)]);
    return 0;
}
