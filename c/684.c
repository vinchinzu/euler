/*
 * Project Euler 684 - Inverse Digit Sum
 *
 * s(n) = smallest number with digit sum n.
 * S(k) = sum_{n=1}^k s(n).
 * Answer = sum_{i=2}^{90} S(fib(i)) mod 10^9+7.
 *
 * s(n) = digit (n%9) followed by floor(n/9) nines.
 * Closed-form sum for S(n).
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll power_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = (lll)r * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return r;
}

/* Store Fibonacci numbers - they get huge, but we only need them mod stuff.
   Actually, the formula uses n mod 9 and pow(10, n/9) mod M.
   n can be up to fib(90) ~ 2.88e18. We need n%9 and n/9 (for exponent).
   We can track fib numbers mod 9 and track n/9 for the exponent.
   Actually let's just use __int128 for Fibonacci since fib(90) < 2^63. */

int main(void) {
    int N = 90;
    ll B = 10;

    /* Precompute Fibonacci numbers. fib(90) ~ 2.88e18, fits in ll. */
    ll fib[91];
    fib[1] = 1; fib[2] = 1;
    for (int i = 3; i <= N; i++)
        fib[i] = fib[i-1] + fib[i-2];

    ll ans = 0;
    for (int i = 2; i <= N; i++) {
        ll n = fib[i];
        ll r = n % 9;  /* n mod (B-1) */
        ll q = n / 9;  /* n / (B-1) */
        /* S(n) = (6 + r + r*(r+1)/2) * 10^q - (6 + n) mod M */
        ll coeff = (6 + r + r * (r + 1) / 2) % MOD;
        ll pw = power_mod(B, q, MOD);
        ll term = ((lll)coeff * pw % MOD - (6 + n % MOD) % MOD + MOD) % MOD;
        ans = (ans + term) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
