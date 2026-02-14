/*
 * Project Euler 688 - Piles of Plates
 *
 * f(n,k) = floor((n - C(k,2)) / k) for valid k.
 * Sum all f(n,k) for 1 <= k, 1 <= n <= N.
 *
 * Iterate k from 1, computing the sum contribution using closed forms.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

int main(void) {
    ll N = 10000000000000000LL; /* 10^16 */
    ll n = N;
    ll ans = 0;
    ll k = 1;

    while (n > 0) {
        ll limit = (n / k) % MOD;
        /* term1 = k * limit * (limit - 1) / 2 mod MOD */
        ll term1 = (lll)k % MOD * limit % MOD * ((limit - 1 + MOD) % MOD) % MOD;
        /* Need to divide by 2 */
        ll inv2 = (MOD + 1) / 2;
        term1 = (lll)term1 * inv2 % MOD;
        /* term2 = (n % k + 1) * limit mod MOD */
        ll nk = n % k;
        ll term2 = (lll)((nk + 1) % MOD) * limit % MOD;
        ans = (ans + term1 + term2) % MOD;
        n -= k;
        k++;
    }

    printf("%lld\n", ans);
    return 0;
}
