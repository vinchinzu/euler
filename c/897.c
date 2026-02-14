#include <stdio.h>
#include <math.h>
#include <stdlib.h>

double calculate_area(double *x, int len) {
    double total_trap = 0.0;
    for (int i = 0; i < len - 1; i++) {
        double u = x[i], v = x[i + 1];
        double h = v - u;
        double u4 = u * u * u * u;
        double v4 = v * v * v * v;
        double avg_height = (u4 + v4) / 2.0;
        total_trap += h * avg_height;
    }
    return 2.0 - total_trap;
}

double cbrt_signed(double y) {
    if (y >= 0) return cbrt(y);
    return -cbrt(-y);
}

double solve_for_n(int n) {
    int m = n - 1;
    double *x = (double *)malloc((m + 1) * sizeof(double));
    /* Initialize with linear symmetric guess */
    for (int i = 0; i <= m; i++) {
        x[i] = -1.0 + 2.0 * i / m;
    }

    /* Iterative coordinate descent (Gauss-Seidel) */
    for (int iter = 0; iter < 100000; iter++) {
        double max_diff = 0.0;
        for (int k = 1; k < m; k++) {
            double xp = x[k + 1];
            double xm = x[k - 1];
            double rhs = (xp * xp + xm * xm) * (xp + xm);
            double new_xk = cbrt_signed(rhs / 4.0);
            double diff = fabs(new_xk - x[k]);
            if (diff > max_diff) max_diff = diff;
            x[k] = new_xk;
        }
        if (max_diff < 1e-13) break;
    }

    double result = calculate_area(x, m + 1);
    free(x);
    return result;
}

int main(void) {
    double result = solve_for_n(101);
    printf("%.9f\n", result);
    return 0;
}
