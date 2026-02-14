/*
 * Project Euler 210: Obtuse Angled Triangles
 *
 * Count lattice points (x,y) with |x|+|y| <= N such that
 * (0,0), (N/4, N/4), (x,y) form an obtuse triangle.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    long long N = 1000000000LL;
    long long q = N / 4;

    /* Base count from the diamond minus the circle region */
    long long ans = 3LL * N / 2 * N + (q + 1) * (q + 1) - (q + 1) - 2;

    double sqrt2 = sqrt(2.0);
    long long r = N / 8;
    int min_x = (int)(-(double)r * (sqrt2 - 1.0));

    for (int x = min_x; x < 0; x++) {
        double dy = sqrt(2.0 * (double)r * (double)r - ((double)r - (double)x) * ((double)r - (double)x));
        int idy = (int)ceil(dy);
        ans += 4LL * (2 * idy - 1);
    }

    printf("%lld\n", ans);
    return 0;
}
