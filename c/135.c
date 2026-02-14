/* Project Euler Problem 135: Same differences.
 *
 * Count n < 1,000,000 with exactly 10 solutions to x^2 - y^2 - z^2 = n
 * where x, y, z are positive integers in arithmetic progression.
 *
 * If x = a+d, y = a, z = a-d (a > d > 0), then n = a(4d - a).
 * Equivalently, for each divisor pair, n = k(4d - k) where k in [1, 3d-1].
 */
#include <stdio.h>
#include <string.h>
#include <math.h>

#define LIMIT 1000000
#define TARGET 10

int counts[LIMIT];

int main(void) {
    memset(counts, 0, sizeof(counts));

    int max_d = LIMIT / 4;

    for (int d = 1; d <= max_d; d++) {
        int max_k = 3 * d - 1;
        long long threshold = 4LL * d * d;

        if (threshold <= LIMIT) {
            for (int k = 1; k <= max_k; k++) {
                int n = k * (4 * d - k);
                if (n > 0 && n < LIMIT)
                    counts[n]++;
            }
            continue;
        }

        double s = sqrt((double)(threshold - LIMIT));
        int k1 = (int)ceil(2.0 * d - s);
        int k2 = (int)floor(2.0 * d + s);

        if (k1 < 1) k1 = 1;

        if (k1 > 1) {
            for (int k = 1; k < k1; k++) {
                int n = k * (4 * d - k);
                if (n > 0 && n < LIMIT)
                    counts[n]++;
            }
        }

        if (k2 < max_k) {
            for (int k = k2 + 1; k <= max_k; k++) {
                int n = k * (4 * d - k);
                if (n > 0 && n < LIMIT)
                    counts[n]++;
            }
        }
    }

    int result = 0;
    for (int i = 1; i < LIMIT; i++) {
        if (counts[i] == TARGET) result++;
    }

    printf("%d\n", result);
    return 0;
}
