/*
 * Project Euler Problem 596: Number of integer quadruples.
 *
 * Find the number of integer quadruples x,y,z,t such that x^2+y^2+z^2+t^2 <= N.
 * Uses Jacobi's Four Square Theorem.
 * sigma2(n) = sum_{k=1}^{sqrt(n)} (n/k)*k + sum of (arithmetic series)*t corrections
 */

#include <stdio.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

#define MOD 1000000007LL

ll sigma2(ll n, ll mod) {
    ll val = 0;
    ll sq = (ll)sqrt((double)n);
    /* Adjust sq for precision */
    while (sq * sq > n) sq--;
    while ((sq + 1) * (sq + 1) <= n) sq++;

    for (ll k = 1; k <= sq; k++) {
        val = (val + (i128)(n / k) % mod * (k % mod)) % mod;
    }

    for (ll t = 1; t < sq; t++) {
        ll low = n / (t + 1) + 1;
        ll high = n / t;
        ll count = high - low + 1;
        /* sum of low..high = (low+high)*count/2 */
        ll sum_range;
        if ((low + high) % 2 == 0) {
            sum_range = (i128)((low + high) / 2) % mod * (count % mod) % mod;
        } else {
            sum_range = (i128)((low + high) % mod) * ((count / 2) % mod) % mod;
        }
        val = (val + (i128)sum_range * (t % mod)) % mod;
    }

    return (val % mod + mod) % mod;
}

int main() {
    ll N = 100000000LL;  /* 10^8 */
    ll M = MOD;

    ll n_sq = (i128)N * N;  /* 10^16 */

    ll s1 = sigma2(n_sq, M);
    ll s2 = sigma2(n_sq / 4, M);

    ll ans = (1 + 8 * s1 % M - 32 * s2 % M + 2 * M) % M;

    printf("%lld\n", ans);
    return 0;
}
