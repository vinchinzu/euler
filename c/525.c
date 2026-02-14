/*
 * Project Euler 525 - Rolling Ellipse
 *
 * Compute C(a,b) = length of path traced by center of ellipse
 * x^2/a^2 + (y-b)^2/b^2 = 1 as it rolls along the x-axis.
 * Answer: C(1,4) + C(3,4).
 */
#include <stdio.h>
#include <math.h>

#define L 1000000

static double C(double a, double b) {
    double prev_perim_x = 0.0;
    double prev_perim_y = b;
    double prev_x = 0.0;
    double prev_y = b;
    double c_val = 0.0;

    double step = M_PI / 2.0 / L;

    for (int i = 1; i <= L; i++) {
        double theta = step * i;
        double sin_val = sin(theta);
        double cos_val = cos(theta);
        double perim_x = a * sin_val;
        double perim_y = b * cos_val;
        double r = hypot(perim_x, perim_y);
        double alpha = atan(perim_x / perim_y) + atan((a * cos_val) / (b * sin_val));
        double x = r * cos(alpha);
        double y = r * sin(alpha);

        double dx_perim = perim_x - prev_perim_x;
        double dy_perim = perim_y - prev_perim_y;
        double arc = hypot(dx_perim, dy_perim);

        c_val += hypot(x - prev_x + arc, y - prev_y);

        prev_perim_x = perim_x;
        prev_perim_y = perim_y;
        prev_x = x;
        prev_y = y;
    }

    return 4.0 * c_val;
}

int main(void) {
    double ans = C(1.0, 4.0) + C(3.0, 4.0);
    printf("%.8f\n", ans);
    return 0;
}
