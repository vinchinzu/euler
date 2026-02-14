#include <stdio.h>

/*
 * Project Euler 860 - Fair arrangements of stacks
 *
 * 4 stack types with scaled values: GG=4, GS=1, SG=-1, SS=-4.
 * Count sequences of n stacks where scaled sum = 0.
 * Uses multinomial approach from the Python solution:
 *   a+b+c+d = n, a-d even, specific conditions.
 *   Iterate over j (half-difference), compute sum of multinomials.
 */

typedef long long ll;

#define MOD 989898989LL
#define MAXN 10000

ll fact[MAXN + 1];
ll invfact[MAXN + 1];

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

int main(void) {
    int n = 9898;

    /* Precompute factorials */
    fact[0] = 1;
    for (int i = 1; i <= n; i++) {
        fact[i] = fact[i - 1] * i % MOD;
    }
    invfact[n] = power(fact[n], MOD - 2, MOD);
    for (int i = n - 1; i >= 0; i--) {
        invfact[i] = invfact[i + 1] * (i + 1) % MOD;
    }

    ll total = 0;

    for (int j = 0; j <= n / 5; j += 2) {
        int s = (n + 3 * j) / 2;
        if ((n + 3 * j) % 2 != 0) continue;
        int low_c = 4 * j;
        if (s < low_c) continue;

        ll sum_contrib = 0;
        for (int c = low_c; c <= s; c++) {
            int a = s - c;
            int b = a + j;
            int d = c - 4 * j;
            if (a < 0 || b < 0 || c < 0 || d < 0) continue;
            if (a + b + c + d != n) continue;

            ll term = fact[n] % MOD;
            term = term * invfact[a] % MOD;
            term = term * invfact[b] % MOD;
            term = term * invfact[c] % MOD;
            term = term * invfact[d] % MOD;
            sum_contrib = (sum_contrib + term) % MOD;
        }

        if (j == 0) {
            total = (total + sum_contrib) % MOD;
        } else {
            total = (total + 2 * sum_contrib) % MOD;
        }
    }

    printf("%lld\n", total);
    return 0;
}
