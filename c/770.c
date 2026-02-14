/*
 * Project Euler 770 - Deliberate Strategy
 *
 * Find minimum guaranteed amount from game with GIVE/TAKE strategy.
 * f(n,n) ~ 2 - 2/(1+sqrt(pi*n)), solve for n >= (2/R-1)^-2 / pi.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double R = 1.9999;
    double val = 2.0 / R - 1.0;
    double n = 1.0 / (val * val) / M_PI;
    long long ans = (long long)ceil(n);
    printf("%lld\n", ans);
    return 0;
}
