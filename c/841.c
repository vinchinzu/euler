/*
 * Project Euler 841: Regular Star Polygons
 *
 * Sum_{n=3}^{34} A(F_{n+1}, F_{n-1}) where A(p,q) is the area of
 * alternating shading of {p/q} with inradius 1.
 *
 * A(p,q) = T_q + 2 * sum_{k=1}^{q-1} (-1)^(q-k) * T_k
 * where T_k = p * tan(k * pi / p)
 *
 * Uses long double for ~18 digit precision.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    /* Fibonacci numbers */
    long long F[38];
    F[0] = 0; F[1] = 1;
    for (int i = 2; i <= 36; i++) F[i] = F[i-1] + F[i-2];

    long double total = 0.0L;
    long double PI = 3.14159265358979323846264338327950288419716939937510L;

    for (int n = 3; n <= 34; n++) {
        long long p = F[n + 1];
        long long q = F[n - 1];

        /* T(k) = p * tan(k * pi / p) */
        /* Sum = sum_{k=1}^{q-1} (-1)^(q-k) * T(k) */
        long double s = 0.0L;
        int sign = ((q - 1) % 2 == 0) ? 1 : -1;

        for (long long k = 1; k < q; k++) {
            long double term = (long double)p * tanl((long double)k * PI / (long double)p);
            if (sign > 0)
                s += term;
            else
                s -= term;
            sign = -sign;
        }

        long double Tq = (long double)p * tanl((long double)q * PI / (long double)p);
        long double area = Tq + 2.0L * s;
        total += area;
    }

    /* Round to 10 digits after decimal point */
    printf("%.10Lf\n", total);
    return 0;
}
