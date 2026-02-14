/*
 * Project Euler Problem 246: Tangents to an ellipse
 *
 * Count lattice points P outside an ellipse where the angle between
 * the two tangent lines from P exceeds 45 degrees.
 */
#include <stdio.h>
#include <math.h>

static double fsq(double n) { return n * n; }

int main(void) {
    double R = 15000.0;
    double Gx = 8000.0;
    double Gy = 1500.0;
    double Mx = -2000.0;
    /* My = Gy */

    double A2 = fsq(R / 2.0);
    double B2 = fsq(R / 2.0) - fsq((Gx - Mx) / 2.0);

    long long ans = 0;
    int y = 0;

    while (1) {
        double y2 = fsq((double)y);
        double x2 = A2 + 3.0 * B2 - y2 + 2.0 * sqrt(2.0 * fsq(B2) + (A2 - B2) * y2);

        if (x2 < 0) break;

        int x = (int)ceil(sqrt(x2));
        int num_points;
        if (y2 > B2) {
            num_points = 2 * x - 1;
        } else {
            num_points = 2 * (x - (int)floor(sqrt(A2 * (1.0 - y2 / B2))) - 1);
        }

        ans += (long long)(y == 0 ? 1 : 2) * num_points;
        y++;
    }

    printf("%lld\n", ans);
    return 0;
}
