#include <stdio.h>
#include <string.h>

/*
 * Project Euler 856 - Expected number of cards drawn until consecutive pair
 *
 * 52-card deck, 13 ranks, 4 suits. Draw until consecutive same rank.
 * State: counts[k] = number of ranks with k cards remaining,
 *        curr_count = cards remaining of current rank.
 *
 * State space: counts is (c0,c1,c2,c3,c4) with c0+c1+c2+c3+c4=13.
 * curr_count in 0..4.
 *
 * Encode state as index for memoization.
 */

#define N_RANKS 13
#define N_SUITS 4

/* Number of compositions of 13 into 5 non-negative parts: C(17,4) = 2380 */
/* Total states: 2380 * 5 = 11900 */

double memo[2400][5];
int memo_valid[2400][5];

/* Encode (c0,c1,c2,c3,c4) into a unique index */
/* c0+c1+c2+c3+c4 = 13 */
/* Use combinatorial number system */
int state_idx[14][14][14][14]; /* [c1][c2][c3][c4] -> index, c0 = 13-c1-c2-c3-c4 */
int num_states = 0;

void init_states(void) {
    memset(state_idx, -1, sizeof(state_idx));
    for (int c4 = 0; c4 <= N_RANKS; c4++)
        for (int c3 = 0; c3 <= N_RANKS - c4; c3++)
            for (int c2 = 0; c2 <= N_RANKS - c4 - c3; c2++)
                for (int c1 = 0; c1 <= N_RANKS - c4 - c3 - c2; c1++) {
                    state_idx[c1][c2][c3][c4] = num_states++;
                }
}

double E(int c0, int c1, int c2, int c3, int c4, int curr_count) {
    int total = 1 * c1 + 2 * c2 + 3 * c3 + 4 * c4;
    if (total == 0) return 0.0;

    int si = state_idx[c1][c2][c3][c4];
    if (memo_valid[si][curr_count]) return memo[si][curr_count];

    int counts[5] = {c0, c1, c2, c3, c4};

    double result = 1.0;

    for (int k = 1; k <= N_SUITS; k++) {
        int available = counts[k];
        if (curr_count == k) available--;
        if (available <= 0) continue;

        /* Transition: one rank with k cards goes to k-1 */
        int nc[5];
        for (int i = 0; i < 5; i++) nc[i] = counts[i];
        nc[k - 1]++;
        nc[k]--;

        double future = E(nc[0], nc[1], nc[2], nc[3], nc[4], k - 1);
        result += future * k * available / total;
    }

    memo[si][curr_count] = result;
    memo_valid[si][curr_count] = 1;
    return result;
}

int main(void) {
    init_states();
    memset(memo_valid, 0, sizeof(memo_valid));

    /* Initial: 13 ranks with 4 cards each, no previous card */
    double answer = E(0, 0, 0, 0, 13, 0);
    printf("%.8f\n", answer);
    return 0;
}
