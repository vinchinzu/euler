/*
 * Project Euler Problem 236: Luxury Hampers
 *
 * Find the largest ratio m such that the bad-product ratio can be uniform.
 * Uses exact rational arithmetic throughout.
 */
#include <stdio.h>
#include <math.h>

static long long gcd_val(long long a, long long b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

/* Fraction: num/den, always reduced, den > 0 */
typedef struct { long long num, den; } Frac;

static Frac frac_new(long long n, long long d) {
    if (d < 0) { n = -n; d = -d; }
    if (d == 0) { Frac f = {0, 1}; return f; }
    long long g = gcd_val(n < 0 ? -n : n, d);
    Frac f = {n / g, d / g};
    return f;
}

static long long ext_gcd(long long a, long long b, long long *x, long long *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    long long x1, y1;
    long long g = ext_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

static void lin_comb(long long a, long long b, long long c, long long *px, long long *py) {
    if (b == 0) {
        *px = (a != 0) ? c / a : 0;
        *py = 0;
        return;
    }
    long long x1, y1;
    long long g = ext_gcd(a, b, &x1, &y1);
    if (g < 0) { g = -g; x1 = -x1; y1 = -y1; }
    if (c % g != 0) {
        *px = 0; *py = 0;
        return;
    }
    *px = x1 * (c / g);
    *py = y1 * (c / g);
}

/* Floor division for doubles -> long long */
static long long floor_ll(double x) {
    long long r = (long long)x;
    if ((double)r > x) r--;
    return r;
}

static long long ceil_ll(double x) {
    long long r = (long long)x;
    if ((double)r < x) r++;
    return r;
}

int main(void) {
    long long A1 = 5248, A2 = 7872, A3 = 5760;
    long long B1 = 640, B2 = 11328, B3 = 3776;

    long long best_num = 1, best_den = 1;

    for (long long a1 = 1; a1 < A1; a1++) {
        for (long long b1 = 1; b1 < B1; b1++) {
            long long u = b1 * A1; /* numerator of m */
            long long v = a1 * B1; /* denominator of m */

            /* Check m > best: u/v > best_num/best_den => u*best_den > v*best_num */
            /* Use <= to skip if not strictly better */
            if (u * best_den <= v * best_num) continue;

            /* r2 = Fraction(A2*v, B2*u) */
            Frac r2 = frac_new(A2 * v, B2 * u);
            Frac r3 = frac_new(A3 * v, B3 * u);

            if (r2.num > A2 || r2.den > B2 || r3.num > A3 || r3.den > B3)
                continue;
            /* Also need r2.num > 0 and r3.num > 0 since a2,b2,a3,b3 must be positive */
            if (r2.num <= 0 || r3.num <= 0) continue;

            /* s = Fraction((A1+A2+A3)*u, (B1+B2+B3)*v) */
            Frac s = frac_new((A1 + A2 + A3) * u, (B1 + B2 + B3) * v);

            long long ca = s.den * r2.num - s.num * r2.den;
            long long cb = s.den * r3.num - s.num * r3.den;
            long long cc = s.num * b1 - s.den * a1;

            long long g_ab = gcd_val(ca < 0 ? -ca : ca, cb < 0 ? -cb : cb);
            if (g_ab == 0) {
                if (cc == 0) { best_num = u; best_den = v; }
                continue;
            }
            if (cc % g_ab != 0) continue;

            long long px, py;
            lin_comb(ca, cb, cc, &px, &py);
            if (px == 0 && py == 0 && cc != 0) continue;

            /* General solution: t2 = px + (cb/g_ab)*t, t3 = py - (ca/g_ab)*t */
            /* where g_ab = gcd(ca, cb), and ext_gcd was computed internally */
            /* But lin_comb already gives a particular solution (px, py) to ca*x + cb*y = cc */

            /* Need: 1 <= t2 <= min(A2/r2.num, B2/r2.den) */
            /* and:  1 <= t3 <= min(A3/r3.num, B3/r3.den) */
            /* t2 = px + (cb/g)*t */
            /* t3 = py - (ca/g)*t */
            /* where g = gcd(ca, cb) and this g == g_ab */

            /* Recompute g via ext_gcd to get consistent particular solution */
            long long ex, ey;
            long long g = ext_gcd(ca, cb, &ex, &ey);
            if (g < 0) { g = -g; ex = -ex; ey = -ey; }
            if (cc % g != 0) continue;

            px = ex * (cc / g);
            py = ey * (cc / g);

            long long step_t2 = cb / g;
            long long step_t3 = -(ca / g);

            long long max_t2 = A2 / r2.num;
            long long tmp = B2 / r2.den;
            if (tmp < max_t2) max_t2 = tmp;

            long long max_t3 = A3 / r3.num;
            tmp = B3 / r3.den;
            if (tmp < max_t3) max_t3 = tmp;

            /* Find range of t such that:
             * 1 <= px + step_t2 * t <= max_t2
             * 1 <= py + step_t3 * t <= max_t3
             */
            double t_lo = -1e18, t_hi = 1e18;

            if (step_t2 > 0) {
                double lo = (1.0 - px) / (double)step_t2;
                double hi = (max_t2 - px) / (double)step_t2;
                if (lo > t_lo) t_lo = lo;
                if (hi < t_hi) t_hi = hi;
            } else if (step_t2 < 0) {
                double lo = (max_t2 - px) / (double)step_t2;
                double hi = (1.0 - px) / (double)step_t2;
                if (lo > t_lo) t_lo = lo;
                if (hi < t_hi) t_hi = hi;
            } else {
                if (px < 1 || px > max_t2) continue;
            }

            if (step_t3 > 0) {
                double lo = (1.0 - py) / (double)step_t3;
                double hi = (max_t3 - py) / (double)step_t3;
                if (lo > t_lo) t_lo = lo;
                if (hi < t_hi) t_hi = hi;
            } else if (step_t3 < 0) {
                double lo = (max_t3 - py) / (double)step_t3;
                double hi = (1.0 - py) / (double)step_t3;
                if (lo > t_lo) t_lo = lo;
                if (hi < t_hi) t_hi = hi;
            } else {
                if (py < 1 || py > max_t3) continue;
            }

            long long t_min = ceil_ll(t_lo);
            long long t_max = floor_ll(t_hi);

            if (t_min <= t_max) {
                best_num = u;
                best_den = v;
            }
        }
    }

    long long g = gcd_val(best_num, best_den);
    printf("%lld/%lld\n", best_num / g, best_den / g);
    return 0;
}
