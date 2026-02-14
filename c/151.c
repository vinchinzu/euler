/*
 * Project Euler 151 - Paper sheets of standard sizes
 *
 * Expected number of times the envelope contains a single sheet (excluding first/last).
 * State: (count_A1, count_A2, count_A3, count_A4, count_A5).
 * Max possible counts: A1:1, A2:1, A3:2, A4:4, A5:8.
 * Memoized recursion with floating point.
 */
#include <stdio.h>
#include <string.h>

/* Dimensions: A1 in [0,1], A2 in [0,1], A3 in [0,2], A4 in [0,4], A5 in [0,8] */
#define D0 2
#define D1 2
#define D2 3
#define D3 5
#define D4 9
#define STATES (D0 * D1 * D2 * D3 * D4)

static double memo[STATES];
static char computed[STATES];

static int encode(int c0, int c1, int c2, int c3, int c4) {
    return ((((c0 * D1 + c1) * D2 + c2) * D3 + c3) * D4 + c4);
}

static double solve(int c0, int c1, int c2, int c3, int c4) {
    int num_sheets = c0 + c1 + c2 + c3 + c4;
    if (num_sheets == 0) return 0.0;
    if (num_sheets == 1 && c4 == 1) return 0.0;

    int key = encode(c0, c1, c2, c3, c4);
    if (computed[key]) return memo[key];

    double contribution = (num_sheets == 1) ? 1.0 : 0.0;
    double future = 0.0;

    int counts[5] = {c0, c1, c2, c3, c4};
    for (int idx = 0; idx < 5; idx++) {
        if (counts[idx] > 0) {
            double prob = (double)counts[idx] / num_sheets;
            int nc[5] = {c0, c1, c2, c3, c4};
            nc[idx]--;
            if (idx < 4) {
                for (int k = idx + 1; k < 5; k++) nc[k]++;
            }
            future += prob * solve(nc[0], nc[1], nc[2], nc[3], nc[4]);
        }
    }

    double result = contribution + future;
    memo[key] = result;
    computed[key] = 1;
    return result;
}

int main(void) {
    memset(computed, 0, sizeof(computed));

    double raw = solve(1, 0, 0, 0, 0);
    double result = raw - 1.0;

    printf("%.6f\n", result);
    return 0;
}
