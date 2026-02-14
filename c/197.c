/* Project Euler 197: Investigating the behaviour of a recursive sequence. */
#include <stdio.h>
#include <math.h>

int main(void) {
    double u = -1.0;
    for (int i = 0; i < 1000; i++) {
        double power = 30.403243784 - u * u;
        u = floor(pow(2.0, power)) * 1e-9;
    }
    /* After 1000 iterations, sequence has settled into a 2-cycle */
    double u_n = u;
    double power = 30.403243784 - u_n * u_n;
    double u_n1 = floor(pow(2.0, power)) * 1e-9;

    double sum = u_n + u_n1;
    printf("%.9f\n", sum);
    return 0;
}
