/*
 * Project Euler 431 - Square Space Silo
 *
 * Grain poured into cylinder from offset x. Find sum of x values where
 * empty space volume is a perfect square. Uses Simpson's rule + binary search.
 * Translated from python/431.py.
 */
#include <stdio.h>
#include <math.h>

#define PI 3.14159265358979323846

double R_val = 6.0;
double alpha_rad;

double integrand(double theta, double x) {
    double beta = asin(x * sin(theta) / R_val);
    double pq = x * cos(theta) + R_val * cos(beta);
    return pq * pq * pq;
}

double integrate_simpson(double x, double a, double b, int n) {
    double h = (b - a) / n;
    double result = integrand(a, x) + integrand(b, x);
    for (int i = 1; i < n; i++) {
        double t = a + i * h;
        if (i % 2 == 0)
            result += 2.0 * integrand(t, x);
        else
            result += 4.0 * integrand(t, x);
    }
    return result * h / 3.0;
}

double V(double x) {
    return integrate_simpson(x, 0.0, 2.0 * PI, 1000) * tan(alpha_rad) / 3.0;
}

int is_square(double n) {
    int root = (int)sqrt(n);
    return fabs((double)root * root - n) < 1e-9;
}

int main(void) {
    alpha_rad = 40.0 * PI / 180.0;

    double lower = V(0.0);
    double higher = V(R_val);
    double ans = 0.0;

    for (int v_val = 1; v_val <= (int)higher; v_val++) {
        if ((double)v_val > lower && is_square((double)v_val)) {
            double low = 0.0, high = R_val;
            while (fabs(high - low) > 1e-12) {
                double mid = (low + high) / 2.0;
                if (V(mid) < (double)v_val)
                    low = mid;
                else
                    high = mid;
            }
            ans += low;
        }
    }

    printf("%.9f\n", ans);
    return 0;
}
