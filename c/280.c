/*
 * Project Euler 280: Ant and Seeds
 *
 * 5x5 grid, 5 seeds initially in bottom row (row 0). Ant starts at (2,2).
 * At each step, ant moves to random adjacent cell (up/down/left/right).
 * Picks up seed if on bottom row with seed and not carrying.
 * Drops seed if on top row (row 4) with empty slot and carrying.
 * Find expected number of steps to move all seeds to top row.
 *
 * State: (ant position, carrying?, bottom row bitmask, top row bitmask)
 * Solve E[steps] via linear system or value iteration.
 *
 * Bottom bitmask: which columns still have seed in row 0 (starts 0x1F = 31)
 * Top bitmask: which columns have seed in row 4 (starts 0)
 * Total seeds moved = popcount(top). bottom + top + carrying = 5.
 *
 * Since bottom and top bitmasks plus carrying give full state, and
 * ant position is (x, y) with 0<=x<5, 0<=y<5:
 *
 * States: 25 positions * 2 carry * 32 bottom * 32 top = 51200
 * But many are unreachable. We'll enumerate all reachable states.
 *
 * Use Gauss-Seidel iteration for the linear system E[s] = 1 + avg(E[neighbors]).
 */
#include <stdio.h>
#include <string.h>
#include <math.h>

/* State encoding:
 * pos = y*5+x (0..24)
 * carry = 0 or 1
 * bottom = bitmask of seeds in row 0 (0..31)
 * top = bitmask of seeds in row 4 (0..31)
 *
 * But bottom + top + carry = 5, so top is determined by bottom and carry:
 *   top = (5 - popcount(bottom) - carry) bits set
 * Actually no -- top tells WHICH columns have seeds, not just count.
 * We need both.
 *
 * State index: pos * 2 * 32 * 32 + carry * 32 * 32 + bottom * 32 + top
 * Max: 25 * 2 * 32 * 32 = 51200
 */

#define NPOS 25
#define MAX_STATES 51200

static double E[MAX_STATES];

static int popcount(int x) {
    int c = 0;
    while (x) { c += x & 1; x >>= 1; }
    return c;
}

static int encode(int pos, int carry, int bot, int top) {
    return pos * 2048 + carry * 1024 + bot * 32 + top;
}

/* Check if state is valid: popcount(bot) + popcount(top) + carry = 5 */
static int valid_state(int pos, int carry, int bot, int top) {
    if (popcount(bot) + popcount(top) + carry != 5) return 0;
    /* bot and top shouldn't share columns (but they're different rows, so they can) */
    /* Actually seeds move from bottom to top, so any column config is possible */
    return 1;
}

/* Directions: up, down, left, right */
static int dx[] = {0, 0, -1, 1};
static int dy[] = {0, 0, 0, 0};
/* Actually: up means y+1, down means y-1 */
static int dxx[] = {0, 0, -1, 1};
static int dyy[] = {1, -1, 0, 0};

int main(void) {
    /* Terminal state: top = 31 (all 5 seeds in top row) */
    /* For terminal states, E = 0 */

    /* Initialize E to 0 for terminal, large for others */
    memset(E, 0, sizeof(E));

    /* Mark terminal and non-existent states */
    /* We'll iterate: for each non-terminal valid state, compute E */

    /* Precompute validity */
    static char is_valid[MAX_STATES];
    static char is_terminal[MAX_STATES];
    memset(is_valid, 0, sizeof(is_valid));
    memset(is_terminal, 0, sizeof(is_terminal));

    for (int pos = 0; pos < NPOS; pos++) {
        for (int carry = 0; carry <= 1; carry++) {
            for (int bot = 0; bot < 32; bot++) {
                for (int top = 0; top < 32; top++) {
                    if (!valid_state(pos, carry, bot, top)) continue;
                    int idx = encode(pos, carry, bot, top);
                    is_valid[idx] = 1;
                    if (top == 31) is_terminal[idx] = 1;
                }
            }
        }
    }

    /* Initialize non-terminal E to some positive value */
    for (int i = 0; i < MAX_STATES; i++) {
        if (is_valid[i] && !is_terminal[i]) E[i] = 1000.0;
    }

    /* Value iteration: E[s] = 1 + (1/num_neighbors) * sum(E[next_state(s, dir)]) */
    /* Repeat until convergence */

    for (int iter = 0; iter < 10000; iter++) {
        double max_change = 0.0;

        for (int pos = 0; pos < NPOS; pos++) {
            int x = pos % 5;
            int y = pos / 5;

            for (int carry = 0; carry <= 1; carry++) {
                for (int bot = 0; bot < 32; bot++) {
                    for (int top = 0; top < 32; top++) {
                        int idx = encode(pos, carry, bot, top);
                        if (!is_valid[idx] || is_terminal[idx]) continue;

                        double sum_next = 0.0;
                        int count = 0;

                        for (int d = 0; d < 4; d++) {
                            int nx = x + dxx[d];
                            int ny = y + dyy[d];
                            if (nx < 0 || nx >= 5 || ny < 0 || ny >= 5) continue;

                            int npos = ny * 5 + nx;
                            int ncarry = carry;
                            int nbot = bot;
                            int ntop = top;

                            /* Pick up seed: on bottom row (ny=0), not carrying, seed present */
                            if (ny == 0 && ncarry == 0 && (nbot & (1 << nx))) {
                                ncarry = 1;
                                nbot &= ~(1 << nx);
                            }
                            /* Drop seed: on top row (ny=4), carrying, no seed there */
                            else if (ny == 4 && ncarry == 1 && !(ntop & (1 << nx))) {
                                ncarry = 0;
                                ntop |= (1 << nx);
                            }

                            int nidx = encode(npos, ncarry, nbot, ntop);
                            sum_next += E[nidx];
                            count++;
                        }

                        double new_E = 1.0 + sum_next / count;
                        double change = fabs(new_E - E[idx]);
                        if (change > max_change) max_change = change;
                        E[idx] = new_E;
                    }
                }
            }
        }

        if (max_change < 1e-12) break;
    }

    /* Starting state: ant at (2,2), not carrying, bottom = 31, top = 0 */
    int start = encode(2 * 5 + 2, 0, 31, 0);
    printf("%.6f\n", E[start]);
    return 0;
}
