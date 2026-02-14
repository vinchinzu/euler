/*
 * Project Euler 638: Weighted Paths in a Grid
 *
 * C(a, b, k) = q-binomial coefficient with q = k.
 * Sum_{k=1}^{7} C(10^k + k, 10^k + k, k) mod 10^9+7.
 *
 * The q-analog of [n]_q = 1 + q + q^2 + ... + q^{n-1}.
 * q-factorial: [n]_q! = [1]_q * [2]_q * ... * [n]_q.
 * q-binomial: [a+b]_q! / ([a]_q! * [b]_q!)
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000007LL

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll modinv(ll a, ll mod) {
    return powmod(a, mod - 2, mod);
}

/* Compute q-binomial C(a+b, a) with q=k, mod M */
/* [n]_q = 1 + q + q^2 + ... + q^{n-1} = (q^n - 1)/(q-1) for q != 1 */
/* For k=1: it's just the regular binomial C(2*base, base) */
ll qbinom(ll a, ll b, ll k) {
    if (k == 1) {
        /* Regular binomial C(a+b, a) mod M */
        /* Use Lucas or direct computation */
        ll n = a + b;
        /* Compute C(n, a) mod M using factorial */
        /* But n can be up to 2*10^7 + 14, manageable */
        ll *fact = (ll *)malloc((n + 1) * sizeof(ll));
        fact[0] = 1;
        for (ll i = 1; i <= n; i++)
            fact[i] = (lll)fact[i-1] * i % M;
        ll result = (lll)fact[n] * modinv(fact[a], M) % M;
        result = (lll)result * modinv(fact[b], M) % M;
        free(fact);
        return result;
    }

    ll n = a + b;
    /* Compute q-factorials: qfact[i] = [1]_q * [2]_q * ... * [i]_q */
    /* [i]_q = (1 + q + ... + q^{i-1}) */
    /* We compute incrementally: pow_k = k^i, sum_pow_k += pow_k for [i+1] */
    ll *qfact = (ll *)malloc((n + 1) * sizeof(ll));
    qfact[0] = 1;
    ll pow_k = 1;
    ll sum_pow_k = 0;
    for (ll i = 1; i <= n; i++) {
        sum_pow_k = (sum_pow_k + pow_k) % M;
        qfact[i] = (lll)qfact[i-1] * sum_pow_k % M;
        pow_k = (lll)pow_k * k % M;
    }

    ll result = (lll)qfact[n] * modinv(qfact[a], M) % M;
    result = (lll)result * modinv(qfact[b], M) % M;
    free(qfact);
    return result;
}

int main() {
    ll ans = 0;
    for (int k = 1; k <= 7; k++) {
        ll base = 1;
        for (int i = 0; i < k; i++) base *= 10;
        base += k; /* 10^k + k */
        ans = (ans + qbinom(base, base, k)) % M;
    }
    printf("%lld\n", ans);
    return 0;
}
