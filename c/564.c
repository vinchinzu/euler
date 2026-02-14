/*
 * Project Euler Problem 564: Expected Maximal Polygon Area
 *
 * Enumerate all compositions of n sides summing to 2n-3, compute the maximal
 * cyclic polygon area for each, weight by multinomial count, and average over
 * C(2n-4, n-1) total compositions. Sum E(n) for n=3..50.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAXN 51
#define MAXSIDES 50

typedef struct { int val; int count; } Side;

static double ffactorial(int n) {
    double r = 1.0;
    for (int i = 2; i <= n; i++) r *= i;
    return r;
}

static double fnCr(int n, int r) {
    if (r < 0 || r > n) return 0.0;
    if (r == 0 || r == n) return 1.0;
    if (r > n - r) r = n - r;
    double result = 1.0;
    for (int i = 0; i < r; i++)
        result = result * (n - i) / (i + 1);
    return result;
}

static int feq(double a, double b) {
    return fabs(a - b) < 1e-10;
}

static double fsq(double x) { return x * x; }

static int center_outside(Side *sides, int nsides) {
    if (sides[nsides-1].count > 1) return 0;
    double angle = 0.0;
    double max_side = sides[nsides-1].val;
    for (int i = 0; i < nsides; i++)
        angle += sides[i].count * asin((double)sides[i].val / max_side);
    return angle < M_PI;
}

static double ans;

static void helper(int n, int rem_sides, int rem_perim, Side *sides, int nsides) {
    if (rem_sides == 0) {
        if (rem_perim != 0) return;

        int co = center_outside(sides, nsides);
        double low = (double)sides[nsides-1].val;
        double high = 2.0 * n;
        double prev = 0.0;

        while (!feq(low, high)) {
            double mid = (low + high) / 2.0;
            double angle = 0.0;
            for (int i = 0; i < nsides; i++) {
                prev = sides[i].count * asin((double)sides[i].val / mid);
                angle += prev;
            }
            if (co) angle = M_PI + 2 * prev - angle;
            if (angle > M_PI) low = mid; else high = mid;
        }

        double area = 0.0;
        for (int i = 0; i < nsides; i++) {
            prev = sides[i].count * sides[i].val * sqrt(fsq(low) - fsq(sides[i].val)) / 4.0;
            area += prev;
        }
        if (co) area -= 2 * prev;

        area *= ffactorial(n);
        for (int i = 0; i < nsides; i++)
            area /= ffactorial(sides[i].count);
        ans += area / fnCr(2 * n - 4, n - 1);
        return;
    }

    int start_val = (nsides == 0) ? 1 : sides[nsides-1].val + 1;
    for (int val = start_val; val <= rem_perim / rem_sides; val++) {
        for (int count = 1; count <= rem_sides; count++) {
            if (val * count <= rem_perim) {
                sides[nsides].val = val;
                sides[nsides].count = count;
                helper(n, rem_sides - count, rem_perim - val * count, sides, nsides + 1);
            }
        }
    }
}

int main(void) {
    ans = 0.0;
    Side sides[MAXSIDES];

    for (int n = 3; n <= 50; n++) {
        helper(n, n, 2 * n - 3, sides, 0);
    }

    printf("%.6f\n", ans);
    return 0;
}
