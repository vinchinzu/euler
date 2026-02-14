/*
 * Project Euler 667 - Moving Pentagon
 *
 * Find maximum area of equal-sided pentagon pushable through L-shaped
 * corridor of width 1. Uses ternary search.
 */
#include <stdio.h>
#include <math.h>

static double corridor_ratio_at(double sofa_a, double sofa_b, double t) {
    double Dx = cos(sofa_a + sofa_b - t);
    double Dy = sin(t) + sin(sofa_a + sofa_b - t);
    double Ox = cos(sofa_a - t) / (2.0 * cos(sofa_a));
    double Oy = sin(sofa_a + t) / (2.0 * cos(sofa_a));
    return (Oy / (Dy - Oy) - Ox / (Dx - Ox)) /
           (1.0 / (Dy - Oy) - 1.0 / (Dx - Ox));
}

static double ternary_search_ratio(double sofa_a, double sofa_b,
                                   double left, double right) {
    double eps = 1e-12;
    while (right - left > eps) {
        double m1 = left + (right - left) / 3.0;
        double m2 = right - (right - left) / 3.0;
        double f1 = corridor_ratio_at(sofa_a, sofa_b, m1);
        double f2 = corridor_ratio_at(sofa_a, sofa_b, m2);
        if (f1 < f2) left = m1;
        else right = m2;
    }
    return corridor_ratio_at(sofa_a, sofa_b, (left + right) / 2.0);
}

static double max_area(double a) {
    double b = acos(1.0 / (4.0 * cos(a)));

    double start_ratio = 0.5 * tan(a) + sin(b - a);
    double middle_ratio = ternary_search_ratio(a, b, 0, M_PI / 2.0);

    double s_denom = start_ratio > middle_ratio ? start_ratio : middle_ratio;
    double s = 1.0 / s_denom;
    double area = s * s * (tan(a) / 4.0 + tan(b) / (8.0 * cos(a) * cos(a)));
    return area;
}

int main() {
    double left = 0.0, right = M_PI / 3.0;
    double eps = 1e-12;
    while (right - left > eps) {
        double m1 = left + (right - left) / 3.0;
        double m2 = right - (right - left) / 3.0;
        double f1 = max_area(m1);
        double f2 = max_area(m2);
        if (f1 < f2) left = m1;
        else right = m2;
    }
    double result = max_area((left + right) / 2.0);
    printf("%.10f\n", result);
    return 0;
}
