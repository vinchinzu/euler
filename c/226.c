/*
 * Project Euler Problem 226: A Scoop of Blancmange
 *
 * Find the area between the Blancmange curve and a circle.
 */
#include <stdio.h>
#include <math.h>

static int feq(double a, double b) {
    return fabs(a - b) < 1e-13;
}

int main(void) {
    double low_x = 0.0;
    double high_x = 0.5;
    double high_y = 0.5;
    double blancmange_area = 0.0;

    while (!feq(low_x, high_x)) {
        double mid_x = (low_x + high_x) / 2.0;
        double mid_y = 0.0;

        double pow_val = 1.0;
        for (int n = 0; n < 50; n++) {
            mid_y += fabs(mid_x - round(pow_val * mid_x) / pow_val);
            pow_val *= 2.0;
        }

        double circle_y = 0.5 - sqrt(0.0625 - (0.25 - mid_x) * (0.25 - mid_x));

        if (mid_y < circle_y) {
            low_x = mid_x;
        } else {
            blancmange_area += ((high_x - mid_x) * (high_y + mid_y) +
                               (high_x - mid_x) * (high_x - mid_x)) / 2.0;
            high_x = mid_x;
            high_y = mid_y;
        }
    }

    double trapezoid_area = (0.5 - high_x) * (0.5 + high_y) / 2.0;
    double segment_area = acos(4.0 * high_x - 1.0) / 32.0 - (0.5 - high_y) / 8.0;
    double ans = blancmange_area - trapezoid_area + segment_area;

    printf("%.8f\n", ans);
    return 0;
}
