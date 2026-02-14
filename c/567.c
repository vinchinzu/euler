/*
 * Project Euler Problem 567: Reciprocal Games I
 *
 * Sum_{n=1}^N (J_A(n) + J_B(n)) = 4*H_{N-1} - ln(4)
 * where H_n = harmonic number and N = 123456789.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    long long N = 123456789LL;

    /* Compute harmonic number H_{N-1} using Kahan summation for precision */
    double sum = 0.0;
    double c = 0.0;
    for (long long i = 1; i <= N - 1; i++) {
        double y = 1.0 / (double)i - c;
        double t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }

    double ans = 4.0 * sum - log(4.0);
    printf("%.8f\n", ans);
    return 0;
}
