/*
 * Project Euler 528 - Constrained Sums
 *
 * S(n, k, b) = number of solutions to x_1+...+x_k <= n with 0 <= x_m <= b^m.
 * Find sum_{k=10}^{15} S(10^k, k, k) mod 10^9+7.
 *
 * Uses inclusion-exclusion with modular arithmetic.
 * For each subset of constraints, compute nCr(n - d + k, k) mod p.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

static ll power(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m;
    if (base < 0) base += m;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % m;
        base = (lll)base * base % m;
        exp >>= 1;
    }
    return result;
}

/*
 * nCr mod p for large n, small k.
 * Compute n*(n-1)*...*(n-k+1) / k! mod p.
 * n can be huge (up to 10^15 + 15), k is at most 15.
 */
static ll ncr_mod(ll n, int k) {
    if (k < 0 || (ll)k > n) return 0;
    if (k == 0) return 1;

    ll num = 1;
    for (int i = 0; i < k; i++) {
        num = (lll)num % MOD * (((n - i) % MOD + MOD) % MOD) % MOD;
    }

    ll den = 1;
    for (int i = 1; i <= k; i++) {
        den = (lll)den * i % MOD;
    }

    return (lll)num % MOD * power(den, MOD - 2, MOD) % MOD;
}

static ll ipow(ll base, int exp) {
    ll result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
        /* Check for overflow: if result > 10^18, it's too large for d */
        if (result < 0) return -1; /* overflow sentinel */
    }
    return result;
}

static ll S(ll n, int k, int b) {
    ll result = 0;

    for (int subset = 0; subset < (1 << k); subset++) {
        ll d = 0;
        int bits = 0;
        int overflow = 0;

        for (int i = 0; i < k; i++) {
            if (subset & (1 << i)) {
                ll pw = ipow(b, i + 1);
                if (pw < 0) { overflow = 1; break; }
                d += pw + 1;
                if (d < 0) { overflow = 1; break; } /* overflow */
                bits++;
            }
        }

        if (overflow || d > n + k) continue;

        ll rem = n - d + k;
        if (rem < 0) continue;

        ll term = ncr_mod(rem, k);
        if (bits % 2 == 0)
            result = (result + term) % MOD;
        else
            result = (result - term + MOD) % MOD;
    }

    return result;
}

int main(void) {
    ll ans = 0;

    for (int k = 10; k <= 15; k++) {
        ll n = 1;
        for (int i = 0; i < k; i++) n *= 10; /* 10^k */
        ans = (ans + S(n, k, k)) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
