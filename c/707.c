/*
 * Project Euler Problem 707: Lights Out.
 *
 * Compute sum_{k=1}^N F(N, f_k) where F(w,h) = 2^(w*h - corank) and f_k
 * is the k-th Fibonacci number. The corank is computed via GCD of
 * polynomials over GF(2). We precompute p_h(x+1) mod p_N(x) and find
 * the period.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_DEG 256
typedef long long ll;

/* Polynomial over GF(2), stored as array of bits (coefficients mod 2) */
typedef struct {
    int deg;           /* degree, or -1 if zero */
    unsigned char c[MAX_DEG + 1];  /* c[i] = coefficient of x^i, 0 or 1 */
} Poly;

void poly_init_zero(Poly *p) { p->deg = -1; memset(p->c, 0, MAX_DEG + 1); }
void poly_init_one(Poly *p) { memset(p->c, 0, MAX_DEG + 1); p->c[0] = 1; p->deg = 0; }
void poly_init_x(Poly *p) { memset(p->c, 0, MAX_DEG + 1); p->c[1] = 1; p->deg = 1; }

int poly_is_zero(Poly *p) { return p->deg < 0; }

void poly_fix_deg(Poly *p) {
    int d = MAX_DEG;
    while (d >= 0 && p->c[d] == 0) d--;
    p->deg = d;
}

void poly_copy(Poly *dst, Poly *src) { memcpy(dst, src, sizeof(Poly)); }

/* a = a + b (mod 2) */
void poly_add_inplace(Poly *a, Poly *b) {
    int md = a->deg > b->deg ? a->deg : b->deg;
    for (int i = 0; i <= md; i++) a->c[i] ^= b->c[i];
    poly_fix_deg(a);
}

/* result = a shifted up by n (multiply by x^n) */
void poly_shift_up(Poly *result, Poly *a, int n) {
    poly_init_zero(result);
    if (a->deg < 0) return;
    for (int i = a->deg; i >= 0; i--) result->c[i + n] = a->c[i];
    result->deg = a->deg + n;
}

/* a = a mod divisor (mod 2 coefficients) */
void poly_mod(Poly *a, Poly *divisor) {
    if (a->deg < divisor->deg) return;
    while (a->deg >= divisor->deg) {
        if (a->c[a->deg] == 0) {
            a->deg--;
            continue;
        }
        int shift = a->deg - divisor->deg;
        for (int i = 0; i <= divisor->deg; i++) {
            a->c[shift + i] ^= divisor->c[i];
        }
        poly_fix_deg(a);
    }
}

/* GCD over GF(2) */
void poly_gcd(Poly *result, Poly *a, Poly *b) {
    Poly aa, bb, tmp;
    poly_copy(&aa, a);
    poly_copy(&bb, b);
    while (!poly_is_zero(&bb)) {
        poly_copy(&tmp, &bb);
        poly_mod(&aa, &bb);
        poly_copy(&bb, &aa);
        poly_copy(&aa, &tmp);
    }
    poly_copy(result, &aa);
}

ll pow_mod(ll base, ll exp, ll mod) {
    if (mod <= 0) return 0;
    if (mod == 1) return 0;
    base %= mod;
    if (base < 0) base += mod;
    ll result = 1;
    while (exp > 0) {
        if (exp & 1) result = (__int128)result * base % mod;
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* 2x2 matrix over mod */
typedef struct { ll a[2][2]; } Mat2;

Mat2 mat2_mult(Mat2 *a, Mat2 *b, ll mod) {
    Mat2 c;
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 2; j++) {
            c.a[i][j] = 0;
            for (int k = 0; k < 2; k++)
                c.a[i][j] = (c.a[i][j] + (__int128)a->a[i][k] * b->a[k][j]) % mod;
        }
    return c;
}

ll fibonacci(ll n, ll mod) {
    if (mod == 1) return 0;
    if (n <= 0) return 0;
    if (n == 1) return 1 % mod;
    Mat2 result = {{{1, 0}, {0, 1}}};
    Mat2 base = {{{1, 1}, {1, 0}}};
    ll exp = n - 1;
    while (exp > 0) {
        if (exp & 1) result = mat2_mult(&result, &base, mod);
        base = mat2_mult(&base, &base, mod);
        exp >>= 1;
    }
    return result.a[0][0] % mod;
}

int main() {
    int N = 199;
    ll M = 1000000007LL;

    /* Build p_n(x) for n=0..N */
    Poly *px = malloc((N + 1) * sizeof(Poly));
    poly_init_one(&px[0]);
    poly_init_x(&px[1]);
    for (int n = 2; n <= N; n++) {
        Poly shifted;
        poly_shift_up(&shifted, &px[n - 1], 1);
        poly_copy(&px[n], &shifted);
        poly_add_inplace(&px[n], &px[n - 2]);
    }

    /* Build p_h(x+1) mod p_N(x) over GF(2), find period */
    /* px1[0] = 1, px1[1] = x+1, px1[n+1] = (x+1)*px1[n] + px1[n-1] mod p_N(x) */
    int max_period = 100000;
    Poly *px1 = malloc((max_period + 2) * sizeof(Poly));
    poly_init_one(&px1[0]);
    /* px1[1] = x + 1 */
    poly_init_zero(&px1[1]);
    px1[1].c[0] = 1; px1[1].c[1] = 1; px1[1].deg = 1;

    int period = 2;
    while (!poly_is_zero(&px1[period - 1])) {
        Poly *last = &px1[period - 1];
        Poly *penult = &px1[period - 2];
        Poly shifted, tmp;
        poly_shift_up(&shifted, last, 1); /* x * last */
        poly_copy(&tmp, &shifted);
        poly_add_inplace(&tmp, last);      /* (x+1) * last */
        poly_add_inplace(&tmp, penult);     /* + penult */
        poly_mod(&tmp, &px[N]);
        poly_copy(&px1[period], &tmp);
        period++;
        if (period > max_period) break;
    }

    ll ans = 0;
    for (int k = 1; k <= N; k++) {
        ll fib_k_mod_period = fibonacci(k, period);
        ll fib_k_mod_m_minus_1 = fibonacci(k, M - 1);

        Poly gcd_poly;
        poly_gcd(&gcd_poly, &px[N], &px1[fib_k_mod_period]);
        int corank = gcd_poly.deg;
        if (corank < 0) corank = 0;

        ll exponent = ((ll)N * fib_k_mod_m_minus_1 - corank) % (M - 1);
        if (exponent < 0) exponent += M - 1;
        ans = (ans + pow_mod(2, exponent, M)) % M;
    }

    printf("%lld\n", ans);

    free(px);
    free(px1);
    return 0;
}
