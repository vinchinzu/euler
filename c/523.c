/*
 * Project Euler 523 - First Sort I
 *
 * Expected number of moves for the sorting algorithm over all
 * permutations of N elements. Formula: sum over n=2..N of
 * sum over i=0..n-2 of 2^i / n.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 30;
    double ans = 0.0;

    for (int n = 2; n <= N; n++) {
        double pow2 = 1.0;
        for (int i = 0; i < n - 1; i++) {
            ans += pow2 / n;
            pow2 *= 2.0;
        }
    }

    printf("%.2f\n", ans);
    return 0;
}
