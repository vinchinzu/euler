/*
 * Project Euler Problem 948
 * Left-Right game on words of L's and R's.
 * F(n) = number of words of length n where first mover wins.
 * Find F(60).
 */
#include <stdio.h>
#include <string.h>

/*
 * State: (k, gp, gs) where k = number of proper suffixes of Type L,
 * gp = Good Prefix, gs = Good Suffix (booleans).
 * k can go up to n, gp and gs are 0/1.
 * Index: k * 4 + gp * 2 + gs
 */

#define MAXN 61
#define MAXK 62
#define STATE_SIZE (MAXK * 4)

static long long counts[STATE_SIZE];
static long long new_counts[STATE_SIZE];

static inline int idx(int k, int gp, int gs) {
    return k * 4 + gp * 2 + gs;
}

int main(void) {
    int n = 60;
    if (n <= 0) { printf("0\n"); return 0; }

    memset(counts, 0, sizeof(counts));
    /* Base cases for length 1: "L" -> (0, 0, 1), "R" -> (0, 0, 0) */
    counts[idx(0, 0, 1)] = 1;  /* "L" */
    counts[idx(0, 0, 0)] = 1;  /* "R" */

    for (int length = 1; length < n; length++) {
        memset(new_counts, 0, sizeof(new_counts));
        for (int k = 0; k <= length; k++) {
            for (int gp = 0; gp <= 1; gp++) {
                for (int gs = 0; gs <= 1; gs++) {
                    long long count = counts[idx(k, gp, gs)];
                    if (count == 0) continue;

                    /* Add "L" */
                    int nk_l = k + 1;
                    int ngp_l = gp || (!gs);
                    int ngs_l = 1;
                    new_counts[idx(nk_l, ngp_l, ngs_l)] += count;

                    /* Add "R" */
                    int nk_r = k > 0 ? k - 1 : 0;
                    int ngp_r = gp || (!gs);
                    int ngs_r = (k > 0) ? 1 : 0;
                    new_counts[idx(nk_r, ngp_r, ngs_r)] += count;
                }
            }
        }
        memcpy(counts, new_counts, sizeof(counts));
    }

    /* Count Type N strings: gp=1 and gs=1 */
    long long ans = 0;
    for (int k = 0; k <= n; k++) {
        ans += counts[idx(k, 1, 1)];
    }

    printf("%lld\n", ans);
    return 0;
}
