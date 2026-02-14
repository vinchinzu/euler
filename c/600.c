/*
 * Project Euler Problem 600: Integer sided equi-angular hexagons.
 *
 * Count distinct (up to congruence via D6 symmetry) hexagons with perimeter <= N.
 * Uses Burnside's lemma with brute force for small values and Lagrange interpolation.
 *
 * H(N) is a quasi-polynomial of period 12 and degree 4.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

/*
 * Brute-force H(n) for small n using Burnside's lemma.
 * Count all valid labeled hexagons (a,b,c,d,e,f) with:
 *   a+b = d+e, b+c = e+f, all sides >= 1, perimeter <= n.
 * Free variables: a, b, c (>= 1), e determined by constraints.
 */
ll H_brute(int n) {
    ll id_count = 0;
    ll rot60_count = 0;
    ll rot120_count = 0;
    ll rot180_count = 0;
    ll refl_vert_count = 0;
    ll refl_mid_count = 0;

    for (int a = 1; a <= n; a++) {
        for (int b = 1; a + b <= n; b++) {
            for (int c = 1; a + b + c <= n; c++) {
                int e_max_1 = a + b - 1;  /* d >= 1 */
                int e_max_2 = b + c - 1;  /* f >= 1 */
                int e_max = e_max_1 < e_max_2 ? e_max_1 : e_max_2;
                int e_min = 2 * a + 3 * b + 2 * c - n;
                if (e_min < 1) e_min = 1;
                if (e_min > e_max) continue;

                /* Identity */
                id_count += e_max - e_min + 1;

                /* rot60: a=b=c, e=a */
                if (a == b && b == c) {
                    int e_val = a;
                    if (e_val >= e_min && e_val <= e_max)
                        rot60_count++;
                }

                /* rot120: a=c, e=a */
                if (a == c) {
                    int e_val = a;
                    if (e_val >= e_min && e_val <= e_max)
                        rot120_count++;
                }

                /* rot180: e=b (gives d=a, f=c) */
                {
                    int e_val = b;
                    if (e_val >= e_min && e_val <= e_max)
                        rot180_count++;
                }

                /* reflection through vertex: a=c and e=b */
                if (a == c) {
                    int e_val = b;
                    if (e_val >= e_min && e_val <= e_max)
                        refl_vert_count++;
                }

                /* reflection through midpoint: e=c */
                {
                    int e_val = c;
                    if (e_val >= e_min && e_val <= e_max)
                        refl_mid_count++;
                }
            }
        }
    }

    return (id_count + 2 * rot60_count + 2 * rot120_count + rot180_count +
            3 * refl_vert_count + 3 * refl_mid_count) / 12;
}

/*
 * Lagrange interpolation using exact rational arithmetic with __int128.
 * Given points (xs[i], ys[i]) for i=0..n-1, evaluate at x.
 */
ll lagrange_interp(int *xs, ll *ys, int npts, ll x) {
    /* Use __int128 for intermediate computation */
    /* The result must be an integer. */
    /* Compute: sum_i ys[i] * prod_{j!=i} (x - xs[j]) / (xs[i] - xs[j]) */

    /* Compute common denominator: prod of all (xs[i] - xs[j]) for j != i */
    /* Actually, it's simpler to use the formula with rational arithmetic. */

    /* Since npts is small (~ 9), and values fit in 64-bit, let's use
     * __int128 and compute the numerator/denominator explicitly. */

    /* Compute the Lagrange form step by step */
    /* num/den = sum_i ys[i] * prod_{j!=i}(x - xs[j]) / prod_{j!=i}(xs[i] - xs[j]) */

    /* Compute D = prod_{j!=i}(xs[i] - xs[j]) for each i */
    i128 num = 0;
    i128 den_common = 1;

    /* First compute the common denominator as lcm of all individual denoms */
    /* Actually, for Lagrange interpolation with integer xs:
     * result = sum_i ys[i] * L_i(x)
     * L_i(x) = prod_{j!=i} (x - xs[j]) / (xs[i] - xs[j])
     *
     * The denominators are w_i = prod_{j!=i} (xs[i] - xs[j])
     * Common denominator is lcm of all w_i.
     *
     * Since xs are equally spaced (0,1,2,...,n-1), w_i = (-1)^{n-1-i} * i! * (n-1-i)!
     */

    /* Compute sum using fraction arithmetic with __int128 */
    /* num/den approach */
    i128 result_num = 0;
    i128 result_den = 1;

    for (int i = 0; i < npts; i++) {
        i128 numer = ys[i];
        i128 denom = 1;
        for (int j = 0; j < npts; j++) {
            if (j != i) {
                numer *= (x - xs[j]);
                denom *= (xs[i] - xs[j]);
            }
        }
        /* Add numer/denom to result_num/result_den */
        /* result = result_num/result_den + numer/denom */
        result_num = result_num * denom + numer * result_den;
        result_den = result_den * denom;

        /* Simplify to prevent overflow */
        i128 g = result_num;
        i128 h = result_den;
        if (g < 0) g = -g;
        if (h < 0) h = -h;
        while (h != 0) { i128 t = h; h = g % h; g = t; }
        if (g > 1) {
            result_num /= g;
            result_den /= g;
        }
    }

    /* result should be an integer */
    if (result_den < 0) { result_num = -result_num; result_den = -result_den; }
    return (ll)(result_num / result_den);
}

int main() {
    int N = 55106;

    /* Compute H(n) for n = 1..100 using brute force */
    ll vals[101];
    for (int n = 0; n <= 100; n++) {
        if (n < 6) vals[n] = 0;
        else vals[n] = H_brute(n);
    }

    /* Determine residue class */
    int r = N % 12;  /* 55106 % 12 = 2 */
    int target_k = (N - r) / 12;

    /* Collect interpolation points for residue r */
    int xs[20];
    ll ys[20];
    int npts = 0;
    for (int k = 0; k <= 8; k++) {
        int nn = 12 * k + r;
        if (nn >= 1 && nn <= 100) {
            xs[npts] = k;
            ys[npts] = vals[nn];
            npts++;
        }
    }

    ll result = lagrange_interp(xs, ys, npts, (ll)target_k);

    printf("%lld\n", result);
    return 0;
}
