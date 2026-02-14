/*
 * Project Euler Problem 469: Empty chairs
 *
 * Knights sit at a round table choosing random empty non-adjacent chairs.
 * Find expected fraction of empty chairs when no more can sit.
 *
 * Recurrence: E(i) = sum_{j=1}^{i-2} (1 + E(j) + E(i-j-1)) / (i-2)
 * Answer converges to (i - E(i)) / (i + 1) as i grows.
 */
#include <stdio.h>

int main(void) {
    int LIMIT = 1000;
    double Es[1000];
    double ans = 0.0;
    double prev_ans = -1.0;

    for (int i = 0; i < LIMIT; i++) {
        double E = 0.0;
        if (i > 1) {
            for (int j = 1; j <= i - 2; j++) {
                E += (1.0 + Es[j] + Es[i - j - 1]) / (double)(i - 2);
            }
        }
        Es[i] = E;
        double cand = (i > 0) ? (i - E) / (i + 1) : 0.0;
        if (cand == prev_ans) break;
        prev_ans = cand;
        ans = cand;
    }

    printf("%.14f\n", ans);
    return 0;
}
