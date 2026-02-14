/*
 * Project Euler 436 - Unfair wager
 *
 * P(y > x) = (1 + 14*e - 5*e^2) / 4.
 * Translated from python/436.py.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double e = M_E;
    double ans = (1.0 + 14.0 * e - 5.0 * e * e) / 4.0;
    printf("%.10f\n", ans);
    return 0;
}
