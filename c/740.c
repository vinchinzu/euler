/* Project Euler 740: Secret Santa.
 * DP with memoization. State: (ns0, ns1, ns2) where ns[k] = number of players
 * with k slips remaining in the hat.
 * n = 100, so ns0 + ns1 + ns2 <= 100, ns2 <= 100, ns1 <= 200, ns0 <= 100.
 * We use a 3D array for memoization since states are bounded.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NMAX 101

/* State: (ns0, ns1, ns2) */
/* ns0 can be 0..100, ns1 can be 0..200, ns2 can be 0..100 */
/* Total states: 101 * 201 * 101 ~ 2M -- fits in memory */

#define NS0_MAX 101
#define NS1_MAX 201
#define NS2_MAX 101

static double memo[NS0_MAX][NS1_MAX][NS2_MAX];
static char visited[NS0_MAX][NS1_MAX][NS2_MAX];

double npr(int n, int r) {
    double result = 1.0;
    for (int i = 0; i < r; i++)
        result *= (n - i);
    return result;
}

double f(int ns0, int ns1, int ns2) {
    if (ns0 < 0 || ns1 < 0 || ns2 < 0) return 0.0;
    int sum_ns = ns0 + ns1 + ns2;
    if (sum_ns == 1) {
        return (ns0 == 0) ? 0.0 : 1.0;
    }
    if (sum_ns <= 0) return 0.0;

    if (visited[ns0][ns1][ns2])
        return memo[ns0][ns1][ns2];

    visited[ns0][ns1][ns2] = 1;

    int slips[4];
    slips[0] = 2 * ns0 + ns1;  /* slips from players with 0 remaining */
    slips[1] = ns1;             /* slips from players with 1 remaining */
    slips[2] = 2 * ns2;        /* slips from players with 2 remaining */
    slips[3] = 1;               /* special (used below) */

    double result = 0.0;

    for (int p = 0; p < 3; p++) {
        /* p is the type of the current player (0, 1, or 2 slips remaining) */
        int ns_p;
        if (p == 0) ns_p = ns0;
        else if (p == 1) ns_p = ns1;
        else ns_p = ns2;

        if (ns_p <= 0) continue;

        for (int s1 = 0; s1 < 3; s1++) {
            for (int s2 = 0; s2 < (s1 == 2 ? 4 : 3); s2++) {
                int num_s1 = slips[s1];
                int num_s2 = slips[s2];

                /* Adjust for self-exclusion */
                if (s1 == p) num_s1 -= p;
                if (s2 == p) num_s2 -= p;

                if (s1 == s2) {
                    num_s2 -= (s1 == 2) ? 2 : 1;
                }

                if (num_s1 <= 0 || num_s2 <= 0) continue;

                int new_ns0 = ns0, new_ns1 = ns1, new_ns2 = ns2;

                /* Remove current player */
                if (p == 0) new_ns0--;
                else if (p == 1) new_ns1--;
                else new_ns2--;

                /* Process first slip drawn */
                if (s1 != 0) {
                    /* Reduce count of source type */
                    if (s1 == 1) new_ns1--;
                    else if (s1 == 2) new_ns2--;

                    /* s2 == 3 means the second part of a pair from type 2 */
                    if (s2 == 3)
                        new_ns0++;
                    else
                        new_ns1 += (s1 == 1) ? 0 : 0;  /* handled below */

                    /* Actually: when drawing from type s1 (1 or 2), that player loses a slip */
                    /* If s1 == 1: player goes from ns1 to ns0 */
                    /* If s1 == 2: player goes from ns2 to ns1 */
                    if (s1 == 1) new_ns0++;
                    else if (s1 == 2) {
                        if (s2 == 3) {
                            /* second slip from same player, so skip the normal +1 */
                            /* This case: both slips from same ns2 player -> goes to ns0 */
                            /* Already decremented ns2 and added ns0 above */
                        } else {
                            new_ns1++;
                        }
                    }
                }

                /* Process second slip drawn */
                if (s2 != 0 && s2 != 3) {
                    if (s2 == 1) { new_ns1--; new_ns0++; }
                    else if (s2 == 2) { new_ns2--; new_ns1++; }
                }

                if (new_ns0 < 0 || new_ns1 < 0 || new_ns2 < 0) continue;
                if (new_ns0 >= NS0_MAX || new_ns1 >= NS1_MAX || new_ns2 >= NS2_MAX) continue;

                double sub = f(new_ns0, new_ns1, new_ns2);
                result += sub * num_s1 * num_s2 / npr(2 * sum_ns - p, 2) * ns_p;
            }
        }
    }

    result /= sum_ns;
    memo[ns0][ns1][ns2] = result;
    return result;
}

int main() {
    memset(visited, 0, sizeof(visited));

    double result = f(0, 0, 100);
    printf("%.10f\n", result);
    return 0;
}
