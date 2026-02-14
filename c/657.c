/*
 * Project Euler 657 - Incomplete Words
 * Inclusion-exclusion on distinct letters in a word of length <= N
 * using K letters. N = 10^12, K = 10^7, M = 10^9+7.
 */
#include <stdio.h>
#include <stdlib.h>

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

int main(void) {
    ll N = 1000000000000LL;
    int K = 10000000;

    /* Precompute t^(N+1) mod MOD for t = 0..K */
    ll *pows = (ll *)malloc((K + 1) * sizeof(ll));
    pows[0] = 0;
    for (int i = 1; i <= K; i++) {
        pows[i] = power_mod(i, N + 1, MOD);
    }

    /* Precompute modular inverses for 1..K */
    ll *invs = (ll *)malloc((K + 1) * sizeof(ll));
    invs[0] = 0;
    invs[1] = 1;
    for (int i = 2; i <= K; i++) {
        invs[i] = (MOD - (MOD / i) * invs[MOD % i] % MOD) % MOD;
    }

    ll ans = 0;
    ll num_choices = 1; /* C(K, t) */

    for (int t = 0; t < K; t++) {
        ll num_words;
        if (t == 0) {
            num_words = 1;
        } else if (t == 1) {
            num_words = (N + 1) % MOD;
        } else {
            num_words = (pows[t] - 1 + MOD) % MOD * invs[t - 1] % MOD;
        }

        ll sign = ((K - t) % 2 == 0) ? 1 : MOD - 1;
        ans = (ans - sign * num_words % MOD * num_choices % MOD + MOD) % MOD;

        if (t < K - 1) {
            num_choices = num_choices * ((K - t) % MOD) % MOD * invs[t + 1] % MOD;
        }
    }

    printf("%lld\n", ans % MOD);

    free(pows);
    free(invs);
    return 0;
}
