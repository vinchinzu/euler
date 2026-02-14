/* Project Euler 183: Maximum product of parts. */
#include <stdio.h>
#include <math.h>

static long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    int START_N = 5, END_N = 10000;
    long long sum_val = 0;

    for (int n = START_N; n <= END_N; n++) {
        int k0 = (int)(n / M_E);
        /* candidates: 1, n, k0-1..k0+2 */
        int candidates[6];
        int nc = 0;
        candidates[nc++] = 1;
        candidates[nc++] = n;
        for (int delta = -1; delta <= 2; delta++) {
            int k = k0 + delta;
            if (k >= 1 && k <= n) {
                candidates[nc++] = k;
            }
        }

        int best_k = 1;
        double best_val = -1e30;
        for (int i = 0; i < nc; i++) {
            int k = candidates[i];
            double val = k * log((double)n / k);
            if (val > best_val) {
                best_val = val;
                best_k = k;
            }
        }

        long long gcd_val = gcd(n, best_k);
        long long reduced_k = best_k / gcd_val;
        while (reduced_k % 2 == 0) reduced_k /= 2;
        while (reduced_k % 5 == 0) reduced_k /= 5;

        sum_val += (reduced_k == 1) ? -n : n;
    }

    printf("%lld\n", sum_val);
    return 0;
}
