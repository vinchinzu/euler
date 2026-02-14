/*
 * Project Euler Problem 711: Binary Blackboard.
 *
 * Count starting values n <= 2^N for which Eric can guarantee even popcount
 * at sum 2n. N = 12345678, mod = 10^9+7.
 *
 * Precompute powers of 2 mod M, then iterate:
 * For i = 2, 4, 6, ..., n-1 (even):
 *   ans = (pow2[i/2] - 2) * pow2[i] + 2*ans + pow2[i] - 1 + pow2[i]
 * For i = 1, 3, 5, ..., n-1 (odd):
 *   ans += pow2[i] - 1
 * Finally add pow2[n] - 1 + pow2[n].
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    int n = 12345678;
    ll m = MOD;

    /* We can't store 12M+ powers. Instead, compute pow2[i] on the fly.
     * We need pow2[i] and pow2[i/2] for even i.
     * Let's iterate with two running powers. */

    ll ans = 0;

    /* For even i from 2 to n-1 (step 2) */
    /* pow2_i = 2^i, pow2_half = 2^(i/2) */
    ll pow2_i = 4;       /* 2^2 */
    ll pow2_half = 2;    /* 2^1 */
    for (int i = 2; i < n; i += 2) {
        /* ans = (pow2_half - 2) * pow2_i + 2*ans + pow2_i - 1 + pow2_i */
        /* From Python: ans = ((pow2_half - 2) * pow2_i + ans + pow2_i - 1 + pow2_i) */
        /* Actually re-reading the Python:
         *   ans = (ans + (pow2s[i // 2] - 2) * pow2s[i] + ans) % m
         *   ans = (ans + pow2s[i] - 1) % m
         *   ans = (ans + pow2s[i]) % m
         * So: new_ans = 2*old_ans + (pow2_half - 2)*pow2_i + pow2_i - 1 + pow2_i
         */
        ans = (2 * ans % m + (pow2_half - 2 + m) % m * pow2_i % m + pow2_i - 1 + pow2_i) % m;
        if (ans < 0) ans += m;

        /* Update powers for next iteration */
        pow2_i = pow2_i * 4 % m;     /* 2^(i+2) */
        pow2_half = pow2_half * 2 % m; /* 2^(i/2 + 1) */
    }

    /* For odd i from 1 to n-1 (step 2) */
    ll pow2_odd = 2; /* 2^1 */
    for (int i = 1; i < n; i += 2) {
        ans = (ans + pow2_odd - 1 + m) % m;
        pow2_odd = pow2_odd * 4 % m; /* 2^(i+2) */
    }

    /* Add pow2[n] - 1 and pow2[n] */
    ll pow2_n = pow_mod(2, n, m);
    ans = (ans + pow2_n - 1 + m) % m;
    ans = (ans + pow2_n) % m;

    printf("%lld\n", ans);
    return 0;
}
