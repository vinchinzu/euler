/* Project Euler Problem 746: A Messy Dinner.
 * Translated from python/746.py
 *
 * Precompute factorials and inverse factorials, then for each k from 2 to N,
 * compute M(k) using inclusion-exclusion.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define MAXN 2021
#define MAX_FACT (4 * MAXN + 1)

ll fact[MAX_FACT + 1];
ll inv_fact[MAX_FACT + 1];

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

void precompute(void) {
    fact[0] = 1;
    for (int i = 1; i <= MAX_FACT; i++)
        fact[i] = (lll)fact[i - 1] * i % MOD;
    inv_fact[MAX_FACT] = pow_mod(fact[MAX_FACT], MOD - 2, MOD);
    for (int i = MAX_FACT - 1; i >= 0; i--)
        inv_fact[i] = (lll)inv_fact[i + 1] * (i + 1) % MOD;
}

ll nCr(int n, int r) {
    if (r < 0 || r > n) return 0;
    return (lll)fact[n] % MOD * inv_fact[r] % MOD * inv_fact[n - r] % MOD;
}

ll sq(ll n) {
    return (lll)n * n % MOD;
}

int main() {
    int N = MAXN;
    precompute();

    ll ans = 0;
    for (int k = 2; k <= N; k++) {
        ll res = sq(fact[2 * k]);
        for (int r = 1; r <= k; r++) {
            ll f_k = nCr(k, r);
            f_k = (lll)f_k * (4 * k) % MOD;
            f_k = (lll)f_k * nCr(4 * k - 3 * r - 1, r - 1) % MOD;
            f_k = (lll)f_k * fact[r - 1] % MOD;
            f_k = (lll)f_k * pow_mod(4, r, MOD) % MOD;
            f_k = (lll)f_k * sq(fact[2 * (k - r)]) % MOD;
            if (r % 2 == 0)
                res = (res + f_k) % MOD;
            else
                res = (res - f_k + MOD) % MOD;
        }
        ans = (ans + 2 * res) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
