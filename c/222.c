/*
 * Project Euler Problem 222: Sphere Packing
 *
 * Find the shortest pipe (internal radius 50mm) that can contain 21 spheres
 * of radii 30..50 mm. Arrange by inserting evens at front, odds at back.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 50, K = 21;
    int order[21];
    int len = 0;

    /* Replicate Python: for i in range(30, 51): if even, insert at 0; else append */
    for (int i = N - K + 1; i <= N; i++) {
        if (i % 2 == 0) {
            /* Insert at position 0: shift everything right */
            for (int j = len; j > 0; j--)
                order[j] = order[j - 1];
            order[0] = i;
        } else {
            order[len] = i;
        }
        len++;
    }

    double length = (double)(order[0] + order[K - 1]);
    for (int i = 1; i < K; i++) {
        double sum_radii = order[i - 1] + order[i];
        double diff = 2.0 * N - sum_radii;
        length += sqrt(sum_radii * sum_radii - diff * diff);
    }

    printf("%ld\n", lround(1000.0 * length));
    return 0;
}
