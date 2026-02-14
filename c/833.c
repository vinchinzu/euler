/*
 * Project Euler 833: Triangular Square
 *
 * Find sum of c for all (a,b,c) with c<=N, a<b, T_a*T_b = c^2.
 * Uses Pell equation solutions and polynomial evaluation via
 * Lagrange interpolation for sum of powers.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

#define MVAL 136101521LL
#define MAX_POLY 200
#define MAX_YS 30

static ll mod_inv(ll a, ll m) {
    ll g = m, x = 0, y = 1;
    ll a0 = a;
    while (a0 != 0) {
        ll q = g / a0;
        ll t = g - q * a0; g = a0; a0 = t;
        t = x - q * y; x = y; y = t;
    }
    return (x % m + m) % m;
}

/* Sum of k-th powers from 1 to n, mod MVAL, using Lagrange interpolation */
static ll sum_powers(ll n, int k) {
    if (n <= 0) return 0;
    if (k == 0) return n % MVAL;

    /* Polynomial of degree k+1, interpolate at k+2 points 0..k+1 */
    int npts = k + 2;
    ll y[MAX_POLY + 2];
    y[0] = 0;
    ll pw = 0;
    for (int j = 1; j < npts; j++) {
        ll jk = 1;
        for (int i = 0; i < k; i++) jk = (i128)jk * j % MVAL;
        pw = (pw + jk) % MVAL;
        y[j] = pw;
    }

    ll n_mod = n % MVAL;

    /* Compute num_prod = prod_{i=0}^{k+1} (n - i) */
    ll num_prod = 1;
    for (int i = 0; i < npts; i++)
        num_prod = (i128)num_prod * ((n_mod - i + MVAL) % MVAL) % MVAL;

    /* Precompute factorials */
    ll fact[MAX_POLY + 2];
    fact[0] = 1;
    for (int i = 1; i < npts; i++) fact[i] = (i128)fact[i-1] * i % MVAL;

    ll result = 0;
    for (int j = 0; j < npts; j++) {
        ll nj = (n_mod - j + MVAL) % MVAL;
        if (nj == 0) {
            /* n == j mod MVAL: compute product directly */
            ll term = y[j];
            for (int i = 0; i < npts; i++) {
                if (i != j) {
                    term = (i128)term * ((n_mod - i + MVAL) % MVAL) % MVAL;
                    ll d = (j - i + MVAL) % MVAL;
                    if ((j - i) < 0) d = MVAL - (ll)((ll)(i - j) % MVAL);
                    d = ((ll)(j - i) % MVAL + MVAL) % MVAL;
                    term = (i128)term * mod_inv(d, MVAL) % MVAL;
                }
            }
            result = (result + term) % MVAL;
        } else {
            /* denom = (-1)^(k+1-j) * j! * (k+1-j)! */
            ll denom = (i128)fact[j] * fact[npts - 1 - j] % MVAL;
            if ((npts - 1 - j) % 2 == 1) denom = (MVAL - denom) % MVAL;

            ll term = (i128)y[j] * num_prod % MVAL;
            term = (i128)term * mod_inv(nj, MVAL) % MVAL;
            term = (i128)term * mod_inv(denom, MVAL) % MVAL;
            result = (result + term) % MVAL;
        }
    }
    return result;
}

/* Polynomial: coefficients stored as rational numbers (big integers).
 * Since the Pell polynomial coefficients grow, we keep them as big integers
 * and reduce mod MVAL only when doing final sums. */

/* For the Pell approach, we need exact (big) polynomial arithmetic
 * for the y-polynomial coefficients, then binary search on a using
 * exact big integer evaluation.
 *
 * We'll represent polynomials with double (for binary search) and
 * ll (for mod MVAL evaluation).
 */

/* Polynomial with exact ll coefficients (may overflow for large polynomials).
 * We'll use double for the binary search and ll%MVAL for the modular sum. */

typedef struct {
    double c_dbl[MAX_POLY]; /* double coefficients for magnitude estimation */
    ll c_mod[MAX_POLY];     /* mod MVAL coefficients */
    int deg;
} Poly;

static Poly poly_zero(void) {
    Poly p;
    memset(&p, 0, sizeof(p));
    p.deg = 0;
    return p;
}

static Poly poly_const(ll v) {
    Poly p = poly_zero();
    p.c_dbl[0] = (double)v;
    p.c_mod[0] = ((v % MVAL) + MVAL) % MVAL;
    p.deg = 0;
    return p;
}

static Poly poly_add(Poly a, Poly b) {
    Poly r = poly_zero();
    r.deg = (a.deg > b.deg) ? a.deg : b.deg;
    for (int i = 0; i <= r.deg; i++) {
        double ad = (i <= a.deg) ? a.c_dbl[i] : 0;
        double bd = (i <= b.deg) ? b.c_dbl[i] : 0;
        r.c_dbl[i] = ad + bd;
        ll am = (i <= a.deg) ? a.c_mod[i] : 0;
        ll bm = (i <= b.deg) ? b.c_mod[i] : 0;
        r.c_mod[i] = (am + bm + MVAL) % MVAL;
    }
    while (r.deg > 0 && r.c_dbl[r.deg] == 0.0 && r.c_mod[r.deg] == 0) r.deg--;
    return r;
}

static Poly poly_mul(Poly a, Poly b) {
    Poly r = poly_zero();
    r.deg = a.deg + b.deg;
    if (r.deg >= MAX_POLY) r.deg = MAX_POLY - 1;
    for (int i = 0; i <= a.deg; i++) {
        for (int j = 0; j <= b.deg && i + j < MAX_POLY; j++) {
            r.c_dbl[i + j] += a.c_dbl[i] * b.c_dbl[j];
            r.c_mod[i + j] = (r.c_mod[i + j] + (i128)a.c_mod[i] * b.c_mod[j]) % MVAL;
        }
    }
    return r;
}

static double poly_eval_dbl(Poly *p, double x) {
    double r = 0, xp = 1;
    for (int i = 0; i <= p->deg; i++) {
        r += p->c_dbl[i] * xp;
        xp *= x;
    }
    return r;
}

static ll gcd(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    /* N = 10^35 */
    /* We need to search for max 'a' using doubles, then compute sums mod MVAL */

    /* D = a(a+1) represented as polynomial [0, 1, 1] */
    Poly D = poly_zero(); D.deg = 2; D.c_dbl[1] = 1; D.c_dbl[2] = 1; D.c_mod[1] = 1; D.c_mod[2] = 1;
    /* base_x = 2a+1 */
    Poly base_x = poly_zero(); base_x.deg = 1; base_x.c_dbl[0] = 1; base_x.c_dbl[1] = 2; base_x.c_mod[0] = 1; base_x.c_mod[1] = 2;
    /* base_y = 2 */
    Poly base_y = poly_const(2);

    Poly x = base_x;
    Poly y = base_y;

    /* Store y polynomials */
    Poly ys[MAX_YS];
    int nys = 0;

    /* Check y(1): for a=1, D=2, so actual value grows exponentially.
     * We stop when y.eval(1) exceeds N = 10^35. */
    /* y(1) at step k grows like (2+sqrt(3))^k, so about 30 iterations needed */
    while (poly_eval_dbl(&y, 1.0) < 1e35 && nys < MAX_YS) {
        ys[nys++] = y;
        Poly new_x = poly_add(poly_mul(x, base_x), poly_mul(D, poly_mul(y, base_y)));
        Poly new_y = poly_add(poly_mul(x, base_y), poly_mul(y, base_x));
        x = new_x;
        y = new_y;
    }

    ll ans = 0;
    for (int i = 0; i < nys; i++) {
        for (int j = i + 1; j < nys; j++) {
            if (gcd(i + 1, j + 1) != 1) continue;

            /* prod = D * ys[i] * ys[j] */
            Poly prod = poly_mul(D, poly_mul(ys[i], ys[j]));

            /* Binary search for max a such that prod(a)/8 <= 10^35 */
            double lo = 0, hi = 1e20;
            while (hi - lo > 0.5) {
                double mid = (lo + hi) / 2.0;
                if (poly_eval_dbl(&prod, mid) / 8.0 <= 1e35)
                    lo = mid;
                else
                    hi = mid;
            }
            ll max_a = (ll)lo;
            /* Verify by adjusting */
            while (max_a > 0 && poly_eval_dbl(&prod, (double)(max_a + 1)) / 8.0 <= 1e35)
                max_a++;
            while (max_a > 0 && poly_eval_dbl(&prod, (double)max_a) / 8.0 > 1e35)
                max_a--;

            if (max_a < 1) continue;

            /* Sum prod(a) for a=1..max_a, each term modularly */
            for (int e = 0; e <= prod.deg; e++) {
                if (prod.c_mod[e] != 0) {
                    ll sp = sum_powers(max_a, e);
                    ans = (ans + (i128)sp * prod.c_mod[e]) % MVAL;
                }
            }
        }
    }

    ans = (i128)ans * mod_inv(8, MVAL) % MVAL;
    printf("%lld\n", ans);
    return 0;
}
