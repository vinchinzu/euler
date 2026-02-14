/* Project Euler Problem 916 - Permutation Subsequences
 * P(n) = C_n^2 * (1 + (3n/(n+2))^2) mod 10^9+7
 * where C_n is the nth Catalan number.
 * Need n! and (2n)! mod p for n=10^8.
 */
#include <stdio.h>

typedef long long ll;
#define MOD 1000000007LL

ll power(ll base, ll exp, ll mod) {
    ll res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) res = res * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return res;
}

ll inverse(ll a, ll mod) {
    return power(a, mod - 2, mod);
}

int main(void) {
    ll n = 100000000LL; /* 10^8 */

    /* Compute n! and (2n)! mod MOD */
    ll fact_n = 1;
    ll fact_2n = 1;
    for (ll i = 1; i <= 2 * n; i++) {
        fact_2n = fact_2n * (i % MOD) % MOD;
        if (i == n) fact_n = fact_2n;
    }

    /* C_n = (2n)! / ((n+1)! * n!) = (2n)! * inv(n+1) * inv(n!)^2 */
    ll inv_fact_n = inverse(fact_n, MOD);
    ll inv_n_plus_1 = inverse((n + 1) % MOD, MOD);

    ll Cn = fact_2n * inv_n_plus_1 % MOD;
    Cn = Cn * inv_fact_n % MOD;
    Cn = Cn * inv_fact_n % MOD;

    /* term2_val = 3n / (n+2) mod p */
    ll term2_num = (3 * (n % MOD)) % MOD;
    ll term2_den = inverse((n + 2) % MOD, MOD);
    ll term2_val = term2_num * term2_den % MOD;

    /* P(n) = Cn^2 * (1 + term2_val^2) */
    ll Cn_sq = Cn * Cn % MOD;
    ll term2_sq = term2_val * term2_val % MOD;
    ll bracket = (1 + term2_sq) % MOD;

    ll ans = Cn_sq * bracket % MOD;
    printf("%lld\n", ans);
    return 0;
}
