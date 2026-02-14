/*
 * Project Euler 435 - Polynomials of Fibonacci numbers
 *
 * F_n(x) = sum_{i=0}^n f_i * x^i. Find sum_{x=0}^{100} F_n(x) mod 15!
 * where n = 10^15. Uses 3x3 matrix exponentiation + CRT.
 * Translated from python/435.py.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

typedef struct { ll a[3][3]; } Mat;

static Mat mat_mult(Mat A, Mat B, ll mod) {
    Mat R;
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++) {
            R.a[i][j] = 0;
            for (int k = 0; k < 3; k++)
                R.a[i][j] = (R.a[i][j] + (lll)A.a[i][k] * B.a[k][j]) % mod;
        }
    return R;
}

static Mat mat_pow(Mat A, ll n, ll mod) {
    Mat result = {{{1,0,0},{0,1,0},{0,0,1}}};
    while (n > 0) {
        if (n & 1) result = mat_mult(result, A, mod);
        A = mat_mult(A, A, mod);
        n >>= 1;
    }
    return result;
}

static ll F_n(ll x, ll N_val, ll m) {
    if (x == 0) return 0;
    ll xm = x % m;
    ll x2m = (lll)xm * xm % m;
    Mat A = {{{1, 0, 0}, {0, 0, 1}, {xm, x2m, xm}}};
    Mat An = mat_pow(A, N_val, m);
    ll result = (An.a[1][0] + (lll)xm * An.a[1][2]) % m;
    return (result + m) % m;
}

static ll extended_gcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    ll x1, y1;
    ll g = extended_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

static ll crt(ll *remainders, ll *moduli, int n) {
    ll M_total = 1;
    for (int i = 0; i < n; i++) M_total *= moduli[i];

    lll result = 0;
    for (int i = 0; i < n; i++) {
        ll Mi = M_total / moduli[i];
        ll inv, dummy;
        extended_gcd(Mi % moduli[i], moduli[i], &inv, &dummy);
        inv = ((inv % moduli[i]) + moduli[i]) % moduli[i];
        result = (result + (lll)remainders[i] * Mi % M_total * inv) % M_total;
    }
    return (ll)((result % M_total + M_total) % M_total);
}

int main(void) {
    ll N_val = 1;
    for (int i = 0; i < 15; i++) N_val *= 10;  /* 10^15 */
    int K = 100;
    ll M = 1307674368000LL;  /* 15! */

    /* 15! = 2^11 * 3^6 * 5^3 * 7^2 * 11 * 13 */
    ll prime_powers[] = {2048, 729, 125, 49, 11, 13};
    int n_pp = 6;

    ll ans = 0;
    for (int x = 0; x <= K; x++) {
        ll remainders[6];
        for (int i = 0; i < n_pp; i++)
            remainders[i] = F_n(x, N_val, prime_powers[i]);
        ll fx = crt(remainders, prime_powers, n_pp);
        ans = (ans + fx) % M;
    }

    printf("%lld\n", ans);
    return 0;
}
