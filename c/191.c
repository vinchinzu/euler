/* Project Euler 191: Prize Strings. */
#include <stdio.h>
#include <string.h>

#define CONSEC_A_MAX 2
#define L_COUNT_MAX 1

int main(void) {
    int n = 30;
    /* prev[consec_a][l_count] */
    long long prev[CONSEC_A_MAX + 1][L_COUNT_MAX + 1];
    long long curr[CONSEC_A_MAX + 1][L_COUNT_MAX + 1];
    memset(prev, 0, sizeof(prev));
    prev[0][0] = 1;

    for (int day = 0; day < n; day++) {
        memset(curr, 0, sizeof(curr));
        for (int ca = 0; ca <= CONSEC_A_MAX; ca++) {
            for (int lc = 0; lc <= L_COUNT_MAX; lc++) {
                long long ways = prev[ca][lc];
                if (ways == 0) continue;
                /* On time (O) */
                curr[0][lc] += ways;
                /* Absent (A) */
                if (ca < CONSEC_A_MAX) {
                    curr[ca + 1][lc] += ways;
                }
                /* Late (L) */
                if (lc == 0) {
                    curr[0][1] += ways;
                }
            }
        }
        memcpy(prev, curr, sizeof(prev));
    }

    long long total = 0;
    for (int ca = 0; ca <= CONSEC_A_MAX; ca++)
        for (int lc = 0; lc <= L_COUNT_MAX; lc++)
            total += prev[ca][lc];

    printf("%lld\n", total);
    return 0;
}
