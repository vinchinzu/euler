/* Project Euler Problem 743: Window into a Matrix.
 * Translated from python/743.py
 *
 * Find the number of 2xN matrices of 0s and 1s such that the sum of the
 * entries in every 2xK sub-matrix is K.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

int main() {
    ll n = 10000000000000000LL; /* 10^16 */
    ll k = 100000000LL;         /* 10^8 */
    ll m = MOD;
    ll half_k = k / 2;

    /* Precompute modular inverses using the standard trick:
     * inv[1] = 1, inv[i] = -(m/i) * inv[m%i] mod m */
    ll *inv = (ll *)malloc((half_k + 2) * sizeof(ll));
    inv[1] = 1;
    for (ll i = 2; i <= half_k + 1; i++) {
        inv[i] = (m - (m / i) * inv[m % i] % m) % m;
    }

    /* base = 2^{-2n/k} mod m */
    ll base = pow_mod(pow_mod(2, 2 * n / k, m), m - 2, m);

    ll res = pow_mod(2, n, m);
    ll ans = 0;

    for (ll i = 0; i <= half_k; i++) {
        ans = (ans + res) % m;
        if (2 * i < k) {
            /* res *= (k-2i)*(k-2i-1) / (i+1)^2 * base */
            ll inv_ip1 = inv[i + 1];
            ll inv_sq = (lll)inv_ip1 * inv_ip1 % m;
            res = (lll)res * inv_sq % m;
            res = (lll)res * ((k - 2 * i) % m) % m;
            res = (lll)res * ((k - 2 * i - 1) % m) % m;
            res = (lll)res * base % m;
        }
    }

    printf("%lld\n", ans % m);

    free(inv);
    return 0;
}
