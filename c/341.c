/* Project Euler 341 - Golomb's self-describing sequence
 *
 * Find sum_{n=1}^{10^6} G(n^3).
 * Build Golomb's sequence iteratively, then use prefix sums.
 */

#include <stdio.h>
#include <stdlib.h>

int main(void) {
    long long N = 1000000;
    /* L = N^1.2 ~ 10^7.2 ~ 15848932 */
    long long L = 15848932;

    int *G = (int*)calloc(L + 10, sizeof(int));
    if (!G) { fprintf(stderr, "malloc failed\n"); return 1; }

    long long size_G = 1;
    G[1] = 1;
    int k = 1;
    while (size_G < L) {
        G[size_G] = k;
        size_G++;
        for (int t = 1; t < G[k]; t++) {
            if (size_G >= L) break;
            G[size_G] = k;
            size_G++;
        }
        k++;
    }

    /* Compute answer using prefix sums */
    long long sum_G = 0;
    long long sum_KG = 0;
    long long ans = 0;
    long long n = 1;

    for (long long ki = 1; ki < size_G; ki++) {
        sum_G += G[ki];
        sum_KG += ki * G[ki];
        while (n < N && n * n * n <= sum_KG) {
            ans += sum_G - (sum_KG - n * n * n) / ki;
            n++;
        }
        if (n >= N) break;
    }

    printf("%lld\n", ans);
    free(G);
    return 0;
}
