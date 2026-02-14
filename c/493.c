/* Project Euler 493 - Under the rainbow
 * Translated from python/493.py
 *
 * Expected distinct colors when selecting N=20 balls from urn
 * with C=7 colors of K=10 balls each.
 * E = C * (1 - C(60,20)/C(70,20))
 */
#include <stdio.h>

int main() {
    int N = 20;
    int K = 10;
    int C = 7;

    /* Compute p = C((C-1)*K, N) / C(C*K, N) = C(60,20) / C(70,20) */
    /* p = product_{i=0}^{N-1} ((C-1)*K - i) / (C*K - i) */
    double p = 1.0;
    for (int i = 0; i < N; i++) {
        p *= (double)((C - 1) * K - i) / (double)(C * K - i);
    }

    double ans = C * (1.0 - p);
    printf("%.9f\n", ans);
    return 0;
}
