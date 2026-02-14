/*
 * Project Euler 863 - Dice Emulation
 *
 * Using a 5-sided and 6-sided die, emulate an n-sided die with minimum expected rolls.
 * Value iteration to find R(n) = min expected rolls for each n.
 * S(1000) = sum R(k) for k=2..1000.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

static double v[1001]; /* V[r] values for current n */

static double solve_r(int n) {
    /* Initialize */
    for (int i = 0; i < n; i++)
        v[i] = 0.0;

    int max_iter = 200000;
    double tol = 1e-11;

    for (int iter = 0; iter < max_iter; iter++) {
        double max_diff = 0.0;

        for (int r = 1; r < n; r++) {
            /* Try D5 */
            int r5 = r * 5;
            int rem5 = r5 % n;
            double term5 = (rem5 != 0) ? ((double)rem5 / r5) * v[rem5] : 0.0;
            double val5 = 1.0 + term5;

            /* Try D6 */
            int r6 = r * 6;
            int rem6 = r6 % n;
            double term6 = (rem6 != 0) ? ((double)rem6 / r6) * v[rem6] : 0.0;
            double val6 = 1.0 + term6;

            double best = val5 < val6 ? val5 : val6;

            double diff = best - v[r];
            if (diff < 0) diff = -diff;
            if (diff > max_diff) max_diff = diff;
            v[r] = best;
        }

        if (max_diff < tol)
            break;
    }

    return v[1];
}

int main(void) {
    double total_s = 0.0;
    for (int k = 2; k <= 1000; k++)
        total_s += solve_r(k);

    printf("%.6f\n", total_s);
    return 0;
}
