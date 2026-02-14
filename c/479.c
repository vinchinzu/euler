/*
 * Project Euler 479 - Roots on the Rise
 *
 * Let a_k, b_k, c_k be the three solutions to 1/x = (k/x)^2*(k+x^2) - k*x.
 * Find sum_{p=1}^N sum_{k=1}^N (a_k+b_k)^p * (b_k+c_k)^p * (c_k+a_k)^p mod 10^9+7.
 *
 * After algebraic simplification, the product (a+b)(b+c)(c+a) for the polynomial
 * turns out to be (1-k^2). So the inner sum over p is a geometric series:
 *   sum_{p=1}^N (1-k^2)^p = (1-k^2) * (1-(1-k^2)^N) / k^2
 */

#include <stdio.h>

typedef long long ll;

#define MOD 1000000007LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll mod_inv(ll a, ll m) {
    return pow_mod(a, m - 2, m);
}

int main(void) {
    ll N = 1000000;
    ll ans = 0;

    for (ll k = 1; k <= N; k++) {
        ll k_sq = k % MOD * (k % MOD) % MOD;
        ll term = (1 - k_sq % MOD + MOD) % MOD;  /* (1 - k^2) mod M */
        ll geo = (1 - pow_mod(term, N, MOD) + MOD) % MOD;  /* 1 - (1-k^2)^N */
        ll val = term % MOD * geo % MOD;
        val = val * mod_inv(k_sq, MOD) % MOD;
        ans = (ans + val) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
