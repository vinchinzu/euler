/*
 * Project Euler Problem 597: Torpedo.
 *
 * N boats starting at points 0, D, 2D, ... on the number line row in the
 * positive direction. Find the probability that the final order is an
 * even permutation.
 *
 * Recursive approach with memoized combinatorial formula.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAXN 15

int N_val;
double L_val;
double D_val;
double spots[MAXN + 2];
double answer;

typedef struct {
    int prev;
    int bumper;
    int bumped;
    int after;
} Bump;

void helper(int *boats, int nboats, Bump *bumps, int nbumps, int last_bumped) {
    if (nbumps == N_val) {
        int parity = 0;
        double constant = 1.0;
        double exponents[MAXN + 2];
        memset(exponents, 0, sizeof(exponents));

        for (int i = 0; i < nbumps; i++) {
            Bump *bump = &bumps[i];
            int bumper = bump->bumper;
            int bumped = bump->bumped;
            int next_bumper, next_bumped;

            if (i + 1 < nbumps && bumps[i + 1].bumper == bumped) {
                next_bumper = bump->bumped;
                next_bumped = bump->after;
            } else {
                next_bumper = bump->prev;
                next_bumped = bump->bumped;
            }

            parity += bumper - bump->prev;
            double exponent = exponents[bumper] + 1;
            constant /= exponent;
            double pow_val = (spots[bumped] - spots[bumper]) /
                           (spots[next_bumped] - spots[next_bumper]);
            exponents[next_bumper] += exponent * pow_val;
            exponents[next_bumped] -= exponent * pow_val;
            exponents[bumped] += exponent;
        }

        if (parity % 2 == N_val % 2) {
            answer += constant;
        }
        return;
    }

    for (int i = 1; i < nboats; i++) {
        if (boats[i] > N_val || boats[i] > last_bumped) break;

        int new_last_bumper = boats[i];

        /* Remove boats[i] */
        int new_boats[MAXN + 2];
        int new_nboats = 0;
        for (int j = 0; j < nboats; j++) {
            if (j != i) new_boats[new_nboats++] = boats[j];
        }

        /* boats[i] in new_boats is now the element that was at boats[i+1] originally */
        /* In the new array, index i corresponds to the old boats[i+1] */
        int new_last_bumped = new_boats[i];  /* was boats[i+1] */
        int after = (i + 1 < new_nboats) ? new_boats[i + 1] : N_val + 1;

        /* The prev is new_boats[i-1] = boats[i-1] (unchanged since we removed boats[i]) */
        Bump b;
        b.prev = new_boats[i - 1];
        b.bumper = new_last_bumper;
        b.bumped = new_last_bumped;
        b.after = after;

        bumps[nbumps] = b;
        helper(new_boats, new_nboats, bumps, nbumps + 1, new_last_bumped);
    }
}

int main() {
    N_val = 13;
    L_val = 1800.0;
    D_val = 40.0;

    memset(spots, 0, sizeof(spots));
    for (int i = 0; i < N_val; i++) {
        spots[i + 1] = D_val * i;
    }
    spots[N_val + 1] = L_val;

    answer = 0.0;

    int boats[MAXN + 2];
    for (int i = 0; i <= N_val + 1; i++) boats[i] = i;

    Bump bumps[MAXN];
    helper(boats, N_val + 2, bumps, 0, N_val);

    printf("%.10f\n", answer);
    return 0;
}
