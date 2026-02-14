/*
 * Project Euler 025 - 1000-digit Fibonacci Number
 * Find the index of the first Fibonacci number with 1000 digits.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double sqrt5 = sqrt(5.0);
    double phi = (1.0 + sqrt5) / 2.0;
    double log10_phi = log10(phi);
    double log10_sqrt5 = log10(sqrt5);

    int digits = 1000;
    double x = (digits - 1 + log10_sqrt5) / log10_phi;
    int n = (int)ceil(x);

    printf("%d\n", n);
    return 0;
}
