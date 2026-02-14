/*
 * Project Euler 147 - Rectangles in cross-hatched grids
 *
 * Count upright and diagonal rectangles in a 47x43 grid.
 */
#include <stdio.h>

static long long upright(int m, int n) {
    return ((long long)m * (m + 1) / 2) * ((long long)n * (n + 1) / 2);
}

static long long diagonal(int m, int n) {
    if (m < n) return diagonal(n, m);
    return (long long)n * ((long long)(2*m - n) * (4*n*n - 1) - 3) / 6;
}

int main(void) {
    long long total = 0;
    for (int a = 1; a <= 47; a++) {
        for (int b = 1; b <= 43; b++) {
            total += upright(a, b) + diagonal(a, b);
        }
    }
    printf("%lld\n", total);
    return 0;
}
