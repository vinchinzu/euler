/*
 * Project Euler 453 - Simple quadrilaterals
 *
 * Count simple quadrilaterals with vertices on lattice points
 * (x,y) with 0 <= x <= W, 0 <= y <= H.
 *
 * Translated from Python. Uses Mobius function and modular arithmetic.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define W 12345
#define H 6789
#define M 135707531LL

static int mu[H + 1];

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

static ll mod_inv(ll a, ll m) {
    ll t = 0, new_t = 1;
    ll r = m, new_r = a % m;
    if (new_r < 0) new_r += m;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    return (t % m + m) % m;
}

static ll sum_powers(int n, int k) {
    ll result = 0;
    for (int i = 1; i <= n; i++) {
        result = (result + pow_mod(i, k, M)) % M;
    }
    return result;
}

static ll f(int a, int b) {
    return pow_mod(W + 1, a, M) * pow_mod(H + 1, b, M) % M;
}

static ll g(int a, int b, int c) {
    int L = H < W ? H : W;  /* min(W, H) */
    ll res = 0;
    for (int gv = 1; gv <= L; gv++) {
        for (int gp = 1; gp <= L / gv; gp++) {
            if (mu[gp] == 0) continue;
            ll term = (ll)mu[gp] * pow_mod(gv, a + b + c, M) % M;
            if (term < 0) term += M;
            term = term * pow_mod(gp, a + b, M) % M;
            term = term * sum_powers(W / gv / gp, a) % M;
            term = term * sum_powers(H / gv / gp, b) % M;
            res = (res + term) % M;
        }
    }
    return res;
}

int main(void) {
    int L = H < W ? H : W;

    /* Compute Mobius function */
    char is_prime[H + 1];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 0; i <= L; i++) mu[i] = 1;

    for (int i = 2; i <= L; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= L; j += i) {
                is_prime[j] = 0;
                if ((ll)j % ((ll)i * i) == 0)
                    mu[j] = 0;
                else
                    mu[j] = -mu[j];
            }
        }
    }

    ll inv3 = mod_inv(3, M);
    ll inv12 = mod_inv(12, M);
    ll inv18 = mod_inv(18, M);
    ll inv432 = mod_inv(432, M);

    /* Precompute g values */
    ll g112 = g(1, 1, 2);
    ll g102 = g(1, 0, 2);
    ll g012 = g(0, 1, 2);
    ll g002 = g(0, 0, 2);
    ll g101 = g(1, 0, 1);
    ll g011 = g(0, 1, 1);
    ll g001 = g(0, 0, 1);
    ll g111 = g(1, 1, 1);

    /* Precompute f values */
    ll f00 = f(0, 0);
    ll f01 = f(0, 1);
    ll f10 = f(1, 0);
    ll f11 = f(1, 1);
    ll f12 = f(1, 2);
    ll f21 = f(2, 1);
    ll f22 = f(2, 2);
    ll f13 = f(1, 3);
    ll f31 = f(3, 1);
    ll f14 = f(1, 4);
    ll f41 = f(4, 1);
    ll f24 = f(2, 4);
    ll f42 = f(4, 2);
    ll f33 = f(3, 3);
    ll f44 = f(4, 4);

    ll ans = 0;

    /* Term 1: 20/3 * (...) */
    ll t1 = ((lll)f00 * g112 % M - (lll)f01 * g102 % M - (lll)f10 * g012 % M + (lll)f11 * g002 % M + 4*M) % M;
    ans = (ans + 20 * inv3 % M * t1) % M;

    /* Term 2: 7 * (...) */
    ll t2 = ((lll)f01 * g101 % M + (lll)f10 * g011 % M - (lll)f11 * g001 % M - (lll)f00 * g111 % M + 4*M) % M;
    ans = (ans + 7 * t2) % M;

    /* Term 3: 4 * (...) */
    ll t3 = ((lll)f12 * g101 % M + (lll)f21 * g011 % M - (lll)f22 * g001 % M - (lll)f11 * g111 % M + 4*M) % M;
    ans = (ans + 4 * t3) % M;

    /* Remaining terms */
    ans = (ans + 7 * inv12 % M * f11) % M;
    ans = (ans - 5 * inv18 % M * ((f12 + f21) % M) % M + M) % M;
    ans = (ans - 7 * inv12 % M * ((f13 + f31) % M) % M + M) % M;
    ans = (ans + 269 * inv432 % M * f22) % M;
    ans = (ans + 5 * inv18 % M * ((f14 + f41) % M)) % M;
    ans = (ans - 149 * inv432 % M * ((f24 + f42) % M) % M + M) % M;
    ans = (ans + 7 * inv12 % M * f33) % M;
    ans = (ans + 29 * inv432 % M * f44) % M;

    ans = ((ans % M) + M) % M;

    printf("%lld\n", ans);
    return 0;
}
