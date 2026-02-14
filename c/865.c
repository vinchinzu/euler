#include <stdio.h>

/*
 * Project Euler 865 - Triplicate Numbers
 *
 * T(10^4) mod 998244353.
 * Recurrence-based DP with dp, prim, v arrays.
 * O(N^2) where N=10000/3=3333.
 */

typedef long long ll;

#define MOD 998244353LL
#define LIMIT 3334

ll dp[LIMIT + 1];
ll prim[LIMIT + 1];
ll v[LIMIT + 1];

ll power(ll base, ll exp, ll mod) {
    ll res = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) res = res * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return res;
}

int main(void) {
    int N = 10000;
    int limit = N / 3;

    ll inv10 = power(10, MOD - 2, MOD);

    dp[0] = 1;
    v[0] = 1;
    prim[0] = 0;

    for (int m = 1; m <= limit; m++) {
        /* 1. Compute dp[m] */
        /* dp[m] = 10 * sum_{c=0..m-1} dp[c] * (sum_{a+b=m-1-c} v[a]*v[b]) */
        ll sum_dp = 0;
        for (int c = 0; c < m; c++) {
            int rem = (m - 1) - c;
            ll conv_v_v = 0;
            for (int a = 0; a <= rem; a++) {
                int b = rem - a;
                conv_v_v = (conv_v_v + v[a] * v[b]) % MOD;
            }
            sum_dp = (sum_dp + dp[c] * conv_v_v) % MOD;
        }
        dp[m] = 10 * sum_dp % MOD;

        /* 2. Compute prim[m] */
        ll sum_prim_dp = 0;
        for (int k = 1; k < m; k++) {
            sum_prim_dp = (sum_prim_dp + prim[k] * dp[m - k]) % MOD;
        }
        prim[m] = (dp[m] - sum_prim_dp % MOD + MOD) % MOD;

        /* 3. Compute v[m] */
        ll sum_v = 0;
        for (int k = 1; k <= m; k++) {
            ll p_val = prim[k] % MOD * 9 % MOD * inv10 % MOD;
            sum_v = (sum_v + p_val * v[m - k]) % MOD;
        }
        v[m] = sum_v;
    }

    /* Calculate T(n) */
    ll total = 0;
    for (int m = 1; m <= limit; m++) {
        ll term = dp[m] % MOD * 9 % MOD * inv10 % MOD;
        total = (total + term) % MOD;
    }

    printf("%lld\n", total);
    return 0;
}
