/* Project Euler 189: Tri-colouring a triangular grid.
   DP over rows, state = base-3 encoding of upward-triangle colours. */
#include <stdio.h>
#include <string.h>

#define N_ROWS 8
/* Max upward triangles in a row = N_ROWS = 8, so state space up to 3^8 = 6561 */
#define MAX_STATES 6561
/* Row state: (last_colour, up_encoding) -> count.  3 * 6561 = 19683 */
#define RS_SIZE (3 * MAX_STATES)

int main(void) {
    int pow3[N_ROWS + 2];
    pow3[0] = 1;
    for (int i = 1; i <= N_ROWS + 1; i++) pow3[i] = pow3[i - 1] * 3;

    /* dp[state] = count, where state is base-3 encoding of up-triangle colours */
    static long long dp[MAX_STATES];
    static long long new_dp[MAX_STATES];
    memset(dp, 0, sizeof(dp));

    /* Row 0: 1 upward triangle, 3 colour choices */
    dp[0] = 1; dp[1] = 1; dp[2] = 1;

    /* Row-transition work arrays: index = last_colour * MAX_STATES + up_encoding */
    static long long rs[RS_SIZE], rs2[RS_SIZE];

    /* Track active entries for efficiency */
    static int active[RS_SIZE];
    static int active2[RS_SIZE];
    int n_active, n_active2;

    for (int row = 1; row < N_ROWS; row++) {
        int prev_ups = row;
        int n_positions = 2 * row + 1;

        memset(new_dp, 0, sizeof(new_dp));

        for (int prev_state = 0; prev_state < pow3[prev_ups]; prev_state++) {
            if (dp[prev_state] == 0) continue;
            long long prev_count = dp[prev_state];

            /* Decode previous row's up-triangle colours */
            int prev_up_colours[N_ROWS];
            int tmp = prev_state;
            for (int k = 0; k < prev_ups; k++) {
                prev_up_colours[k] = tmp % 3;
                tmp /= 3;
            }

            /* Process current row left to right */
            n_active = 0;

            /* Position 0: upward triangle, no left neighbour */
            for (int c = 0; c < 3; c++) {
                int key = c * MAX_STATES + c;
                rs[key] = 1;
                active[n_active++] = key;
            }

            int up_count = 1;

            for (int pos = 1; pos < n_positions; pos++) {
                n_active2 = 0;

                if (pos % 2 == 1) {
                    /* Downward triangle */
                    int k = pos / 2;
                    int above_c = prev_up_colours[k];
                    for (int ai = 0; ai < n_active; ai++) {
                        int key = active[ai];
                        long long cnt = rs[key];
                        rs[key] = 0;
                        int last_c = key / MAX_STATES;
                        int up_enc = key % MAX_STATES;
                        for (int c = 0; c < 3; c++) {
                            if (c == last_c || c == above_c) continue;
                            int nk = c * MAX_STATES + up_enc;
                            if (rs2[nk] == 0) active2[n_active2++] = nk;
                            rs2[nk] += cnt;
                        }
                    }
                } else {
                    /* Upward triangle */
                    int p3 = pow3[up_count];
                    for (int ai = 0; ai < n_active; ai++) {
                        int key = active[ai];
                        long long cnt = rs[key];
                        rs[key] = 0;
                        int last_c = key / MAX_STATES;
                        int up_enc = key % MAX_STATES;
                        for (int c = 0; c < 3; c++) {
                            if (c == last_c) continue;
                            int nk = c * MAX_STATES + up_enc + c * p3;
                            if (rs2[nk] == 0) active2[n_active2++] = nk;
                            rs2[nk] += cnt;
                        }
                    }
                    up_count++;
                }

                /* Swap rs and rs2 by copying active entries */
                for (int ai = 0; ai < n_active2; ai++) {
                    int key = active2[ai];
                    rs[key] = rs2[key];
                    rs2[key] = 0;
                }
                /* Copy active list */
                n_active = n_active2;
                int *tmp_ptr = active;
                for (int ai = 0; ai < n_active; ai++) active[ai] = active2[ai];
            }

            /* Collect: sum over last_colour, group by up_encoding */
            for (int ai = 0; ai < n_active; ai++) {
                int key = active[ai];
                long long cnt = rs[key];
                rs[key] = 0;
                int up_enc = key % MAX_STATES;
                new_dp[up_enc] += cnt * prev_count;
            }
        }

        memcpy(dp, new_dp, sizeof(dp));
    }

    long long total = 0;
    for (int i = 0; i < MAX_STATES; i++) {
        total += dp[i];
    }

    printf("%lld\n", total);
    return 0;
}
