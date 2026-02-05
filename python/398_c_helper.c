#include <stdio.h>
#include <math.h>
#include <stdlib.h>

#define N 10000000L
#define K 100

int main() {
    long array_size = N + 10000;
    double *F = malloc(array_size * sizeof(double));

    if (F == NULL) return 1;

    for (long i = 0; i <= N; i++) {
        if (i % 64 == 0) {
            F[i] = 1.0;
            for (int j = 0; j < K - 1; j++) {
                F[i] *= (double)(N - i + K - 1 - j) / (N - 1 - j);
            }
        } else {
            F[i] = F[i - 1] * (double)(N - i) / (N - i + K - 1);
        }
    }

    double ans = 0.0;

    for (long b = 1; b < N; b++) {
        if (b * (K - 1) >= N) break;

        long idx1 = b * K;
        long idx2 = b + (b + 1) * (K - 1);
        long idx3 = (b + 1) * K;

        if (idx1 >= array_size || idx2 >= array_size || idx3 >= array_size) break;

        double prob = F[idx1] - K * F[idx2] + (K - 1) * F[idx3];

        for (long a = 1; a < b; a++) {
            long i1 = a + b * (K - 1);
            long i2 = a + (b + 1) * (K - 1);
            long i3 = (a + 1) + b * (K - 1);
            long i4 = (a + 1) + (b + 1) * (K - 1);

            if (i1 >= array_size || i2 >= array_size ||
                i3 >= array_size || i4 >= array_size) break;

            double subProb = K * (F[i1] - F[i2] - F[i3] + F[i4]);
            if (fabs(b * subProb) < 1e-15) break;
            prob += subProb;
        }

        ans += b * prob;
    }

    free(F);
    printf("%.5f\n", ans);
    return 0;
}
