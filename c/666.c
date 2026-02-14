/*
 * Project Euler 666 - Polymorphic Bacteria
 *
 * Fixed point iteration for extinction probability of bacteria.
 */
#include <stdio.h>
#include <math.h>

int main() {
    int N = 500;
    int K = 10;

    /* Generate random sequence R */
    int R[5000]; /* N*K = 5000 */
    int r = 306;
    for (int i = 0; i < N * K; i++) {
        R[i] = r;
        r = (r * r) % 10007;
    }

    double probs[500], new_probs[500];
    for (int i = 0; i < N; i++) probs[i] = 0.5;

    while (1) {
        for (int i = 0; i < N; i++) new_probs[i] = 0.0;

        for (int i = 0; i < N; i++) {
            for (int j = 0; j < K; j++) {
                int q = R[i * K + j] % 5;
                if (q == 0) {
                    new_probs[i] += 1.0 / K;
                } else if (q == 1) {
                    new_probs[i] += (probs[i] * probs[i]) / K;
                } else if (q == 2) {
                    new_probs[i] += probs[(2 * i) % N] / K;
                } else if (q == 3) {
                    int idx = (i * i + 1) % N;
                    new_probs[i] += (probs[idx] * probs[idx] * probs[idx]) / K;
                } else if (q == 4) {
                    new_probs[i] += (probs[i] * probs[(i + 1) % N]) / K;
                }
            }
        }

        if (fabs(probs[0] - new_probs[0]) < 1e-10) break;

        for (int i = 0; i < N; i++) probs[i] = new_probs[i];
    }

    printf("%.8f\n", new_probs[0]);
    return 0;
}
