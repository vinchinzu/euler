/* Project Euler Problem 944 - Sum of Elevisors
 * S(n) = sum contributions from divisor pairs.
 * Uses sqrt decomposition.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define MOD 1234567891LL
#define MOD_EXP (MOD - 1)

ll power(ll base, ll exp, ll mod) {
    ll res = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) res = (lll)res * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return res;
}

int main(void) {
    ll n = 100000000000000LL; /* 10^14 */
    ll s = (ll)sqrt((double)n);
    while (s * s > n) s--;
    while ((s+1)*(s+1) <= n) s++;

    ll n_mod_exp = n % MOD_EXP;
    ll sum1 = 0;

    /* Part 1: d = 2 to s */
    for (ll d = 2; d <= s; d++) {
        ll n_div_d = n / d;
        /* C(floor(n/d)) = floor(n/d) * (floor(n/d)+1) / 2 mod MOD */
        ll nd_mod = n_div_d % MOD;
        ll nd1_mod = (n_div_d + 1) % MOD;
        ll c_val;
        if (nd_mod % 2 == 0)
            c_val = (lll)(nd_mod / 2) * nd1_mod % MOD;
        else
            c_val = (lll)nd_mod * (nd1_mod / 2) % MOD;

        /* 2^(n-d) mod MOD */
        ll exp = (n_mod_exp - d % MOD_EXP + MOD_EXP) % MOD_EXP;
        ll power_val = power(2, exp, MOD);

        ll term = (lll)power_val * c_val % MOD;
        sum1 = (sum1 + term) % MOD;
    }

    /* Part 2: d from s+1 to n, transformed to sum over k */
    ll k_max = n / (s + 1);

    /* C(k_max) * 2^(n-s) */
    ll km_mod = k_max % MOD;
    ll km1_mod = (k_max + 1) % MOD;
    ll c_k_max;
    if (km_mod % 2 == 0)
        c_k_max = (lll)(km_mod / 2) * km1_mod % MOD;
    else
        c_k_max = (lll)km_mod * (km1_mod / 2) % MOD;

    ll exp_s = (n - s) % MOD_EXP;
    if (exp_s < 0) exp_s += MOD_EXP;
    ll term1 = (lll)c_k_max * power(2, exp_s, MOD) % MOD;

    /* sum k * 2^(n - floor(n/k)) for k=1..k_max */
    ll sum_k = 0;
    for (ll k = 1; k <= k_max; k++) {
        ll n_div_k = n / k;
        ll exp = (n - n_div_k) % MOD_EXP;
        if (exp < 0) exp += MOD_EXP;
        ll pv = power(2, exp, MOD);
        ll tk = (lll)(k % MOD) * pv % MOD;
        sum_k = (sum_k + tk) % MOD;
    }

    ll sum2 = (term1 - sum_k % MOD + MOD) % MOD;
    ll result = (sum1 + sum2) % MOD;

    printf("%lld\n", result);
    return 0;
}
