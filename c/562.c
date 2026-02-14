/*
 * Project Euler Problem 562: Maximal Triangle Perimeter
 *
 * Find triangle with lattice point vertices within circle of radius N=10^7,
 * no other lattice points on boundary, maximum perimeter.
 * T(N) = R/N where R = abc/2 is the circumradius.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

static ll N_val = 10000000LL;
static int L = 20;

static ll sq(ll n) { return n * n; }

static ll gcd_func(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static void ext_gcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) { *x = 1; *y = 0; return; }
    ll x1, y1;
    ext_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
}

static double max_possible_perim(double a) {
    return a + 2.0 * hypot(a / 2.0, 1.0 / a);
}

int main(void) {
    ll N = N_val;
    double max_perim = 0.0;
    double best_abc = 0.0;
    ll isqrt_half = (ll)sqrt((double)sq(N) / 2.0);

    for (ll x1 = 0; x1 <= isqrt_half; x1++) {
        ll y1 = (ll)sqrt((double)(sq(N) - sq(x1)));
        while (sq(x1) + sq(y1) > sq(N)) y1--;

        double h1 = hypot((double)x1, (double)y1);
        if (max_possible_perim(N + h1) < max_perim) continue;

        ll x2_lo = -x1 - L;
        ll x2_hi_raw = -x1 + L;
        if (x2_hi_raw >= x1) x2_hi_raw = x1 - 1;

        for (ll x2 = x2_lo; x2 <= x2_hi_raw; x2++) {
            ll y2_lo = -y1 - L;
            ll y2_hi_raw = -y1 + L;
            if (y2_hi_raw >= y1) y2_hi_raw = y1 - 1;

            for (ll y2 = y2_lo; y2 <= y2_hi_raw; y2++) {
                if (sq(x2) + sq(y2) > sq(N)) continue;

                double a = hypot((double)(x1 - x2), (double)(y1 - y2));
                if (max_possible_perim(a) < max_perim) continue;
                if (gcd_func(x1 - x2, y1 - y2) != 1) continue;

                ll ex, ey;
                ext_gcd(y1 - y2, x2 - x1, &ex, &ey);
                ll x3 = x2 + ex;
                ll y3 = y2 + ey;

                if (sq(x3) + sq(y3) <= sq(N)) {
                    double b = hypot((double)(x1 - x3), (double)(y1 - y3));
                    double c = hypot((double)(x2 - x3), (double)(y2 - y3));
                    double perim = a + b + c;
                    if (perim > max_perim) {
                        max_perim = perim;
                        best_abc = a * b * c;
                    }
                }
            }
        }
    }

    ll ans = llround(best_abc / 2.0 / (double)N);
    printf("%lld\n", ans);
    return 0;
}
