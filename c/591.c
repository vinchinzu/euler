/*
 * Project Euler Problem 591: Best approximation to pi.
 *
 * For each non-square d < 100, find the closest quadratic integer a + b*sqrt(d)
 * to pi with |a| <= N = 10^13 and |b| <= N, then sum |I_d| where I_d = a.
 *
 * Uses lattice reduction via the Euclidean algorithm.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include <float.h>

typedef long long ll;
typedef __int128 i128;

int is_square(int n) {
    int r = (int)sqrt((double)n);
    if (r * r == n) return 1;
    if ((r + 1) * (r + 1) == n) return 1;
    return 0;
}

int main() {
    ll N = 10000000000000LL;  /* 10^13 */
    int K = 100;
    int D = 5;

    /* High-precision pi using long double */
    long double PI = 3.14159265358979323846264338327950288419716939937510L;

    ll ans = 0;

    for (int d = 2; d < K; d++) {
        if (is_square(d)) continue;

        long double sqrt_d = sqrtl((long double)d);

        /* Lattice basis: u = (sqrt_d, 0, 1), v = (1, 1, 0) */
        /* We track (x, ai, bi) where the vector is x = ai + bi*sqrt_d */
        long double u_x = sqrt_d;
        ll u_ai = 0, u_bi = 1;
        long double v_x = 1.0L;
        ll v_ai = 1, v_bi = 0;

        ll I = -1;
        long double min_error = 1.0L;

        while (llabs(u_ai) <= N) {
            if (fabsl(v_x) < 1e-30L) break;
            ll q = (ll)(u_x / v_x);
            if (q < 0) q--;  /* floor division */

            long double r_x = u_x - (long double)q * v_x;
            ll r_ai = u_ai - q * v_ai;
            ll r_bi = u_bi - q * v_bi;

            u_x = v_x; u_ai = v_ai; u_bi = v_bi;
            v_x = r_x; v_ai = r_ai; v_bi = r_bi;

            long double det = u_x * (long double)v_ai - v_x * (long double)u_ai;
            if (fabsl(det) < 1e-10L) break;

            ll ui0 = (ll)(PI * (long double)v_ai / det);
            ll vi0 = -(ll)(PI * (long double)u_ai / det);

            for (ll ui = ui0 - D; ui <= ui0 + D; ui++) {
                for (ll vi = vi0 - D; vi <= vi0 + D; vi++) {
                    ll a = ui * u_ai + vi * v_ai;
                    ll b = ui * u_bi + vi * v_bi;
                    long double error = fabsl((long double)a + (long double)b * sqrt_d - PI);
                    if (llabs(a) <= N && error < min_error) {
                        I = a;
                        min_error = error;
                    }
                }
            }
        }

        ans += llabs(I);
    }

    printf("%lld\n", ans);
    return 0;
}
