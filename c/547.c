/*
 * Project Euler Problem 547: Distance of Random Points on Hollow Square Laminae.
 *
 * Compute the sum of expected distances for all hollow square laminae of size N=40.
 * Uses closed-form integrals for expected distance between unit squares.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define N 40

static double hypot_(double a, double b) {
    return sqrt(a * a + b * b);
}

static double i_hypot(double x, double y) {
    if (x == 0.0 && y == 0.0) return 0.0;
    double h = hypot_(x, y);
    double res = 4.0 * x * y * y * y / h
               + 4.0 * x * x * x * y / h
               - 2.0 * y * y * y / 3.0;
    if (x != 0.0) {
        double yx = y / x;
        res += x * x * x * x * hypot_(yx, 1.0) * asinh(yx) / h
             + 3.0 * x * x * x * log(h + y)
             - 2.0 * x * x * x * atanh(y / h);
    }
    if (y != 0.0) {
        res += 2.0 * y * y * y * log(h + x);
    }
    return res / 12.0;
}

static double i_x_hypot(double x, double y) {
    double h = hypot_(x, y);
    double res = 5.0 * x * x * y + 2.0 * y * y * y;
    if (x != 0.0) {
        double yx = y / x;
        res += 3.0 * x * x * x * asinh(yx) / hypot_(yx, 1.0);
    }
    return res * h / 24.0;
}

static double i_xy_hypot(double x, double y) {
    double h = hypot_(x, y);
    return h * h * h * h * h / 15.0;
}

typedef double (*IntFn)(double, double);

static double definite_integral(double xl, double xh, double yl, double yh, IntFn f) {
    return f(xh, yh) - f(xh, yl) - f(xl, yh) + f(xl, yl);
}

static double e_val(int dx, int dy, int w, int h) {
    double res = 0.0;
    int signs_w[2] = {-w, w};
    int signs_h[2] = {-h, h};

    for (int sw = 0; sw < 2; sw++) {
        for (int sh = 0; sh < 2; sh++) {
            double dxw = dx + signs_w[sw];
            double dyh = dy + signs_h[sh];

            res += definite_integral(dx, dxw, dy, dyh, i_xy_hypot)
                 - dyh * definite_integral(dx, dxw, dy, dyh, i_x_hypot)
                 - dxw * definite_integral(dy, dyh, dx, dxw, i_x_hypot)
                 + dxw * dyh * definite_integral(dx, dxw, dy, dyh, i_hypot);
        }
    }
    return res;
}

static long long sq(long long n) { return n * n; }

int main(void) {
    /* unit_to_unit[dx][dy] */
    double unit_to_unit[N][N];
    for (int dx = 0; dx < N; dx++)
        for (int dy = 0; dy < N; dy++)
            unit_to_unit[dx][dy] = e_val(dx, dy, 1, 1);

    /* full_to_unit[x1][y1] = sum over all (x2,y2) in NxN grid */
    double full_to_unit[N][N];
    for (int x1 = 0; x1 < N; x1++) {
        for (int y1 = 0; y1 < N; y1++) {
            double s = 0.0;
            for (int x2 = 0; x2 < N; x2++)
                for (int y2 = 0; y2 < N; y2++)
                    s += unit_to_unit[abs(x1 - x2)][abs(y1 - y2)];
            full_to_unit[x1][y1] = s;
        }
    }

    /* region_to_itself[w][h] = e(0, 0, w, h) */
    double region_to_itself[N + 1][N + 1];
    for (int w = 1; w <= N; w++)
        for (int h = 1; h <= N; h++)
            region_to_itself[w][h] = e_val(0, 0, w, h);

    double ans = 0.0;

    for (int xl = 1; xl < N; xl++) {
        for (int yl = 1; yl < N; yl++) {
            /* full_to_region[xh][yh] = sum of full_to_unit for cells in rectangle [xl..xh-1] x [yl..yh-1] */
            double full_to_region[N][N];
            for (int i = 0; i < N; i++)
                for (int j = 0; j < N; j++)
                    full_to_region[i][j] = 0.0;

            for (int xh = xl + 1; xh < N; xh++) {
                for (int yh = yl + 1; yh < N; yh++) {
                    full_to_region[xh][yh] = full_to_unit[xh - 1][yh - 1]
                        + full_to_region[xh][yh - 1]
                        + full_to_region[xh - 1][yh]
                        - full_to_region[xh - 1][yh - 1];

                    long long area = sq(N) - (long long)(xh - xl) * (yh - yl);
                    ans += (region_to_itself[N][N]
                          - 2.0 * full_to_region[xh][yh]
                          + region_to_itself[xh - xl][yh - yl])
                         / (double)sq(area);
                }
            }
        }
    }

    printf("%.4f\n", ans);
    return 0;
}
