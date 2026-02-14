/*
 * Project Euler 788 - Dominating Numbers
 *
 * Count numbers with up to N digits where more than half the digits are equal.
 * Uses modular arithmetic with precomputed factorials.
 */
#include <stdio.h>

#define MOD 1000000007LL

typedef long long ll;

static ll powmod(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = result * base % m;
        base = base * base % m;
        exp >>= 1;
    }
    return result;
}

#define MAXN 2023

static ll fact[MAXN];
static ll inv_fact[MAXN];

static void precompute(void) {
    fact[0] = 1;
    for (int i = 1; i < MAXN; i++)
        fact[i] = fact[i - 1] * i % MOD;
    inv_fact[MAXN - 1] = powmod(fact[MAXN - 1], MOD - 2, MOD);
    for (int i = MAXN - 2; i >= 0; i--)
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD;
}

static ll nCr(int n, int k) {
    if (k < 0 || k > n) return 0;
    return fact[n] * inv_fact[k] % MOD * inv_fact[n - k] % MOD;
}

int main(void) {
    int N = 2022;
    int B = 10;
    precompute();

    ll ans = 0;

    for (int l = 1; l <= N; l++) {
        for (int k = l / 2 + 1; k <= l; k++) {
            /* Case 1: d != 0, first digit is d */
            ans = (ans + (B - 1) * nCr(l - 1, k - 1) % MOD *
                   powmod(B - 1, l - k, MOD) % MOD) % MOD;

            /* Case 2: d != 0, first digit is not d */
            ans = (ans + (B - 1) * nCr(l - 1, k) % MOD *
                   (B - 2) % MOD * powmod(B - 1, l - k - 1, MOD) % MOD) % MOD;

            /* Case 3: d = 0 */
            ans = (ans + nCr(l - 1, k) * powmod(B - 1, l - k, MOD) % MOD) % MOD;
        }
    }

    printf("%lld\n", ans % MOD);
    return 0;
}
