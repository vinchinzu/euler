/* Project Euler 496 - Incenter and circumcircle
 * Translated from python/496.py
 *
 * Uses Mobius function + number theory.
 */
#include <stdio.h>
#include <math.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define SQRT_LIMIT 31624  /* > sqrt(10^9) */

int mu[SQRT_LIMIT + 1];

void compute_mobius(int limit) {
    for (int i = 0; i <= limit; i++) mu[i] = 1;
    char is_prime[SQRT_LIMIT + 1];
    for (int i = 0; i <= limit; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((ll)j % ((ll)i * i) == 0)
                    mu[j] = 0;
                else
                    mu[j] = -mu[j];
            }
        }
    }
}

ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)sqrt((double)n);
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

ll tr(ll n) {
    return n * (n + 1) / 2;
}

int main() {
    ll N = 1000000000LL;
    int sq = (int)isqrt_ll(N);
    compute_mobius(sq);

    ll ans = 0;

    for (ll g = 1; g <= sq; g++) {
        if (mu[g] == 0) continue;

        ll n = N / (g * g);
        ll L = (ll)(cbrt((double)n / 2.0)) + 1;
        ll res = 0;

        for (ll x = 1; x < L; x++) {
            ll max_y = 2 * x - 1;
            if (max_y > n / x) max_y = n / x;
            for (ll y = x + 1; y <= max_y; y++) {
                if (x * y <= n) {
                    res += tr(n / x / y) * x * y;
                }
            }
        }

        ll sq_n = isqrt_ll(n);
        for (ll x = L; x <= sq_n; x++) {
            ll max_y = 2 * x - 1;
            if (max_y > n / x) max_y = n / x;
            if (max_y <= x) continue;

            ll q = n / x / max_y;
            while ((lll)x * x * q <= n) {
                ll upper = n / x / q;
                if (upper > max_y) upper = max_y;
                ll lower_candidate = (q + 1 > 0) ? n / x / (q + 1) : 0;
                ll lower = (lower_candidate > x) ? lower_candidate : x;
                if (upper > lower) {
                    res += tr(q) * x * (tr(upper) - tr(lower));
                }
                q++;
            }
        }

        ans += mu[g] * g * g * res;
    }

    printf("%lld\n", ans);
    return 0;
}
