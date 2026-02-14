/* Project Euler 498 - Remainder of polynomial division
 * Translated from python/498.py
 *
 * Coefficient of x^D in remainder when x^N is divided by (x-1)^K.
 * Uses Lucas theorem for nCk mod prime.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define MOD 999999937LL

ll mulmod(ll a, ll b, ll m) {
    return (lll)a * b % m;
}

ll powmod(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m;
    if (base < 0) base += m;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, m);
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    return result;
}

/* nCk mod prime, small k (k < mod) */
ll ncr_small(ll n, ll k, ll mod) {
    if (k < 0 || k > n) return 0;
    if (k > n - k) k = n - k;
    if (k == 0) return 1;
    ll num = 1, den = 1;
    for (ll i = 1; i <= k; i++) {
        num = mulmod(num, (n - k + i) % mod, mod);
        den = mulmod(den, i % mod, mod);
    }
    return mulmod(num, powmod(den, mod - 2, mod), mod);
}

/* Lucas theorem for nCk mod prime */
ll ncr_lucas(ll n, ll k, ll mod) {
    if (k < 0 || k > n) return 0;
    ll result = 1;
    while (n > 0 || k > 0) {
        ll ni = n % mod;
        ll ki = k % mod;
        if (ki > ni) return 0;
        result = mulmod(result, ncr_small(ni, ki, mod), mod);
        n /= mod;
        k /= mod;
    }
    return result;
}

int main() {
    ll N = 10000000000000LL;   /* 10^13 */
    ll K = 1000000000000LL;    /* 10^12 */
    ll D = 10000LL;            /* 10^4 */

    ll n1 = N - D - 1;
    ll k1 = K - 1 - D;

    ll coeff = ncr_lucas(N, D, MOD);
    coeff = mulmod(coeff, ncr_lucas(n1, k1, MOD), MOD);

    printf("%lld\n", coeff);
    return 0;
}
