#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int main(void) {
    int N = 100000;
    double *G = (double *)calloc(N + 1, sizeof(double));
    double total = 0.0;

    for (int i = 1; i <= N; i++) {
        double low = 0.0;
        double high = 1e300;  /* infinity substitute */
        int j = 0;
        while ((1 << j) <= i) {
            int remaining = (i >> (j + 1) << j) + i % (1 << j);
            if ((i & (1 << j)) > 0) {
                if (G[remaining] > low) low = G[remaining];
            } else {
                if (G[remaining] < high) high = G[remaining];
            }
            j++;
        }
        double d = 1.0;
        G[i] = 0.0;
        while (G[i] <= low || G[i] >= high) {
            G[i] = floor(low / d + 1) * d;
            d /= 2;
        }
        total += (double)i * G[i];
    }
    printf("%lld\n", (long long)ceil(total));
    free(G);
    return 0;
}
