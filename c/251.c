/*
 * Project Euler Problem 251: Cardano Triplets
 *
 * Count Cardano triplets (a,b,c) with a+b+c <= N=110000000 where
 * cbrt(a + b*sqrt(c)) + cbrt(a - b*sqrt(c)) = 1.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

/* Extended GCD: returns gcd, sets *x, *y such that a*x + b*y = gcd */
static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Modular inverse of a mod m (m odd, gcd(a,m)=1) using extended Euclidean */
static ll mod_inverse(ll a, ll m) {
    ll g = m, x = 0, y = 1;
    ll aa = a;
    while (aa != 0) {
        ll q = g / aa;
        ll t = g - q * aa; g = aa; aa = t;
        t = x - q * y; x = y; y = t;
    }
    return (x % m + m) % m;
}

int main(void) {
    ll N = 110000000LL;
    ll ans = 0;

    ll max_r = (ll)sqrt(8.0 * N / 3.0);
    if (max_r % 2 == 0) max_r--;

    for (ll r = 1; r <= max_r; r += 2) {
        ll r2 = r * r;

        /* min_t: smallest positive t such that r^2 * t = 5 (mod 8)
         * Since r is odd, r^2 mod 8 = 1, so we need t = 5 mod 8 */
        ll min_t = (5 * r2) % 8;
        if (min_t == 0) min_t = 8;

        /* Upper bound on s */
        double s_limit_sq = (double)N / min_t - 3.0 * r2 / 8.0;
        if (s_limit_sq < 1.0) continue;
        ll max_s = (ll)sqrt(s_limit_sq);

        for (ll s = 1; s <= max_s; s++) {
            if (gcd(r, s) != 1) continue;

            ll a8s = 8 * s;
            /* modular inverse of 8s mod r^2 (r^2 is odd since r is odd) */
            ll g_inv = mod_inverse(a8s, r2);
            ll g = (g_inv * 3) % r2;
            if (g == 0) g = r2;

            /* t = (8*s*g - 3) / r^2 */
            ll t_val = (a8s * g - 3) / r2;

            ll s2 = s * s;
            ll start = 3 * g * s - 1 + g * r + s2 * t_val;
            if (start <= N) {
                ll increment = (3 * s + r) * r2 + 8 * s2 * s;
                ans += (N - start) / increment + 1;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
