/*
 * Project Euler Problem 394: Eating Pie
 *
 * E(x) = (6*ln(x) + 2/x^3 + 7) / 9
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 40;
    double result = (6.0 * log(N) + 2.0 / ((double)N * N * N) + 7.0) / 9.0;
    printf("%.10f\n", result);
    return 0;
}
