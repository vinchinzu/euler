#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define N 10000

typedef struct {
    double sum;
    int a;
    int b;
} Pair;

static int cmp_pair(const void *x, const void *y) {
    double da = ((const Pair *)x)->sum;
    double db = ((const Pair *)y)->sum;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

int main(void) {
    // Compute f[i] = exp(i/N) - 1
    double f[2 * N + 1];
    for (int i = 0; i <= 2 * N; i++) {
        f[i] = exp((double)i / N) - 1.0;
    }

    // Count pairs where f[k1] + f[k2] < PI, with 1 <= k1 <= k2
    int count = 0;
    for (int k1 = 1; f[k1] < M_PI; k1++) {
        for (int k2 = k1; f[k1] + f[k2] < M_PI; k2++) {
            count++;
        }
    }

    // Allocate and fill pairs
    Pair *pairs = malloc(count * sizeof(Pair));
    if (!pairs) {
        fprintf(stderr, "Allocation failed\n");
        return 1;
    }

    int idx = 0;
    for (int k1 = 1; f[k1] < M_PI; k1++) {
        for (int k2 = k1; f[k1] + f[k2] < M_PI; k2++) {
            pairs[idx].sum = f[k1] + f[k2];
            pairs[idx].a = k1;
            pairs[idx].b = k2;
            idx++;
        }
    }

    // Sort by sum
    qsort(pairs, count, sizeof(Pair), cmp_pair);

    // Two-pointer search for sum closest to PI
    int left = 0;
    int right = count - 1;
    double minError = 1e100;
    int minLeft = -1, minRight = -1;

    while (left <= right) {
        double error = pairs[left].sum + pairs[right].sum - M_PI;
        if (fabs(error) < minError) {
            minError = fabs(error);
            minLeft = left;
            minRight = right;
        }
        if (error < 0) {
            left++;
        } else {
            if (right == 0) break;
            right--;
        }
    }

    // Now find all (k1, k2) pairs that match the target sums
    double targetLeft = pairs[minLeft].sum;
    double targetRight = pairs[minRight].sum;

    uint64_t ans = 0;
    for (int k1 = 1; f[k1] < M_PI; k1++) {
        for (int k2 = k1; f[k1] + f[k2] < M_PI; k2++) {
            double s = f[k1] + f[k2];
            if (s == targetLeft || s == targetRight) {
                ans += (uint64_t)k1 * k1 + (uint64_t)k2 * k2;
            }
        }
    }

    printf("%llu\n", (unsigned long long)ans);

    free(pairs);
    return 0;
}
