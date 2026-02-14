/*
 * Project Euler 658 - Incomplete Words II
 * Sum over 1 <= k <= K of I(k), where I(k) = number of words of length <= N
 * containing some but not all of k letters.
 * N = 10^12, K = 10^7, M = 10^9+7.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define MOD 1000000007LL

static ll power_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

static ll mod_inverse(ll a, ll m) {
    return power_mod(a, m - 2, m);
}

int main(void) {
    ll N = 1000000000000LL;
    int K = 10000000;

    /* Smallest prime factor sieve for fast power computation */
    int *spf = (int *)malloc((K + 1) * sizeof(int));
    for (int i = 0; i <= K; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= K; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= K; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }

    /* Compute t^(N+1) mod MOD using multiplicativity */
    ll *pows = (ll *)calloc(K + 1, sizeof(ll));
    pows[0] = 0;
    pows[1] = 1;
    for (int i = 2; i <= K; i++) {
        if (spf[i] == i) { /* prime */
            pows[i] = power_mod(i, N + 1, MOD);
        } else {
            pows[i] = pows[spf[i]] * pows[i / spf[i]] % MOD;
        }
    }
    free(spf);

    /* Precompute modular inverses */
    ll *invs = (ll *)malloc((K + 1) * sizeof(ll));
    invs[0] = 0;
    invs[1] = 1;
    for (int i = 2; i <= K; i++) {
        invs[i] = (MOD - (MOD / i) * invs[MOD % i] % MOD) % MOD;
    }

    ll inv2 = mod_inverse(2, MOD);

    ll ans = 0;
    ll nCr = 1;
    ll inner_sum = K % 2; /* (-1)^K, but we need the initial value */
    /* inner_sum = sum_{k=t}^{K} (-1)^{K-k} C(K,k)
       For t=0: inner_sum = sum_{k=0}^K (-1)^{K-k} C(K,k)
       = sum_{k=0}^K (-1)^{K-k} C(K,k) = (1-1)^K... no.
       Actually per the Python: inner_sum starts as K % 2 */

    for (int t = 0; t < K; t++) {
        ll num_words;
        if (t == 0) {
            num_words = 1;
        } else if (t == 1) {
            num_words = (N + 1) % MOD;
        } else {
            num_words = (pows[t] - 1 + MOD) % MOD * invs[t - 1] % MOD;
        }

        ans = (ans + num_words % MOD * inner_sum % MOD) % MOD;

        if (t < K - 1) {
            ll new_nCr = nCr * ((K - t) % MOD) % MOD * invs[t + 1] % MOD;
            ll parity_val = ((K - t) % 2 == 0) ? 1 : MOD - 1;
            inner_sum = (inner_sum + 1 + parity_val * ((nCr + new_nCr) % MOD) % MOD) % MOD;
            inner_sum = inner_sum * inv2 % MOD;
            nCr = new_nCr;
        }
    }

    printf("%lld\n", ans % MOD);

    free(pows);
    free(invs);
    return 0;
}
