/* Project Euler Problem 938 - Card Game Probability
 * DP: P(r,b) = (prob_rr * P(r-2,b) + prob_mixed * P(r,b-1)) / (1 - prob_bb)
 * Using O(B) space with 3 rows.
 */
#include <stdio.h>
#include <stdlib.h>

#define TARGET_R 24690
#define TARGET_B 12345

int main(void) {
    int R = TARGET_R, B = TARGET_B;

    double *dp_prev2 = (double *)calloc(B + 1, sizeof(double)); /* P(r-2, b) initially P(0,b) */
    double *dp_prev  = (double *)calloc(B + 1, sizeof(double)); /* P(r-1, b) initially P(1,b) */
    double *dp_curr  = (double *)calloc(B + 1, sizeof(double));

    /* Base: P(0, b) = 1 for b >= 1, P(0,0) = 0 */
    for (int i = 1; i <= B; i++) dp_prev2[i] = 1.0;

    /* P(1, b): prob_rr=0, just mixed */
    for (int b = 1; b <= B; b++) {
        double total = 1.0 + b;
        double prob_bb = (b * (b - 1.0)) / (total * (total - 1.0));
        double prob_mixed = (2.0 * 1.0 * b) / (total * (total - 1.0));
        double p_mixed = dp_prev2[b - 1];
        double temp = prob_mixed * p_mixed;
        dp_prev[b] = temp / (1.0 - prob_bb);
    }

    /* Compute for r from 2 to R */
    for (int r = 2; r <= R; r++) {
        for (int b = 0; b <= B; b++) dp_curr[b] = 0.0;
        for (int b = 1; b <= B; b++) {
            double total = r + b;
            double prob_rr = (r * (r - 1.0)) / (total * (total - 1.0));
            double prob_bb = (b * (b - 1.0)) / (total * (total - 1.0));
            double prob_mixed = (2.0 * r * b) / (total * (total - 1.0));

            double p_rr = dp_prev2[b];
            double p_mixed = dp_curr[b - 1];
            double temp = prob_rr * p_rr + prob_mixed * p_mixed;
            dp_curr[b] = temp / (1.0 - prob_bb);
        }
        /* Rotate arrays */
        double *tmp = dp_prev2;
        dp_prev2 = dp_prev;
        dp_prev = dp_curr;
        dp_curr = tmp;
    }

    printf("%.10f\n", dp_prev[B]);

    free(dp_prev2);
    free(dp_prev);
    free(dp_curr);
    return 0;
}
