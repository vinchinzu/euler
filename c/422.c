/*
 * Project Euler 422 - Sequence of points on a hyperbola
 *
 * Fibonacci-based closed form on hyperbola 12x^2 + 7xy - 12y^2 = 625.
 * Uses matrix exponentiation for Fibonacci numbers mod (M-1), then
 * modular exponentiation for the final answer.
 * Translated from python/422.py.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000007LL

typedef struct { ll a[2][2]; } Mat;

static Mat mat_mult(Mat a, Mat b, ll mod) {
    Mat r;
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 2; j++) {
            r.a[i][j] = 0;
            for (int k = 0; k < 2; k++)
                r.a[i][j] = (r.a[i][j] + (lll)a.a[i][k] * b.a[k][j]) % mod;
        }
    return r;
}

static Mat mat_pow(Mat m, ll exp, ll mod) {
    Mat result = {{{1, 0}, {0, 1}}};
    while (exp > 0) {
        if (exp & 1) result = mat_mult(result, m, mod);
        m = mat_mult(m, m, mod);
        exp >>= 1;
    }
    return result;
}

static ll fibonacci(ll n, ll mod) {
    if (n == 0) return 0;
    if (n == 1) return 1;
    Mat fib_mat = {{{1, 1}, {1, 0}}};
    Mat r = mat_pow(fib_mat, n - 1, mod);
    return r.a[0][0];
}

static ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    /* N = 11^14 */
    ll N = 1;
    for (int i = 0; i < 14; i++) N *= 11;

    ll F_N1 = fibonacci(N - 1, M - 1);
    ll F_N2 = fibonacci(N - 2, M - 1);

    ll a = -(pow_mod(-3, 2 * F_N1 - 1, M) - pow_mod(2, 4 * F_N2 + 2 * F_N1 - 2, M));
    a = ((a % M) + M) % M;

    ll b = (lll)(M - pow_mod(2, 2 * F_N2 + F_N1 - 2, M)) % M * pow_mod(-3, F_N1 - 1, M) % M;
    b = (b % M + M) % M;

    ll c = (pow_mod(-3, 2 * F_N1 + 1, M) + pow_mod(2, 4 * F_N2 + 2 * F_N1 + 2, M)) % M;

    ll d = (lll)pow_mod(2, 2 * F_N2 + F_N1, M) * pow_mod(-3, F_N1, M) % M;

    ll ans = (a + b + c + d) % M;
    ans = (ans + M) % M;

    printf("%lld\n", ans);
    return 0;
}
