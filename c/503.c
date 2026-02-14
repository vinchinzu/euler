/*
 * Project Euler Problem 503: Alice's Game.
 * Optimal stopping problem. Expected score with N=10^6.
 */
#include <stdio.h>

int main() {
    int N = 1000000;
    double ans = (double)N;

    for (int n = N; n >= 1; n--) {
        double d = (double)(N + 1) / (n + 1);
        int k = (int)(ans / d);
        /* tr(k) = k*(k+1)/2 */
        double tr_k = (double)k * (k + 1) / 2.0;
        ans = (tr_k * d + (double)(n - k) * ans) / (double)n;
    }

    printf("%.10f\n", ans);
    return 0;
}
