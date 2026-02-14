/*
 * Project Euler 449 - Chocolate covered candy
 *
 * Volume of chocolate = S(A,B)*T + A^2*B*S(1/A,1/B)*T^2 + 4*pi/3*T^3
 * where S is the surface area of the ellipsoid.
 */
#include <stdio.h>
#include <math.h>

static double sq(double n) { return n * n; }
static double cb(double n) { return n * n * n; }

static double surface_area(double a, double b) {
    if (a >= b) {
        double e = sqrt(1.0 - sq(b / a));
        return M_PI * (2.0 * sq(a) + sq(b) / e * log((1.0 + e) / (1.0 - e)));
    } else {
        double e = sqrt(1.0 - sq(a / b));
        return 2.0 * M_PI * (sq(a) + a * b / e * asin(e));
    }
}

int main(void) {
    double A = 3.0, B = 1.0, T = 1.0;

    double ans = surface_area(A, B) * T
               + sq(A) * B * surface_area(1.0/A, 1.0/B) * sq(T)
               + 4.0 * M_PI / 3.0 * cb(T);

    printf("%.8f\n", ans);
    return 0;
}
