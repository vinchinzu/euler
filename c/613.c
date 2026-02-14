/*
 * Project Euler Problem 613: Pythagorean Ant
 *
 * Probability that an ant in a right triangle exits through the hypotenuse.
 * Uses numerical integration (Simpson's rule).
 */
#include <stdio.h>
#include <math.h>

static double integrate(double (*f)(double), double a, double b, int n) {
    if (n % 2 == 1) n++;
    double h = (b - a) / n;
    double result = f(a) + f(b);
    for (int i = 1; i < n; i++) {
        double coeff = (i % 2 == 1) ? 4.0 : 2.0;
        result += coeff * f(a + i * h);
    }
    return result * h / 3.0;
}

#define A_LEG 30.0
#define B_LEG 40.0

static double f1(double x) {
    return 1.0 - B_LEG * tan(x) / A_LEG;
}

static double f2(double x) {
    return 1.0 - A_LEG * tan(x) / B_LEG;
}

int main(void) {
    double ans = (
        M_PI / 2.0
        + integrate(f1, 0.0, atan2(A_LEG, B_LEG), 10000)
        + integrate(f2, 0.0, atan2(B_LEG, A_LEG), 10000)
    ) / (2.0 * M_PI);

    printf("%.10f\n", ans);
    return 0;
}
