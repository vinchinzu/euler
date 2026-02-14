/*
 * Project Euler 447 - Retractions C
 *
 * Extracted from embedded C in python/447.py.
 * F(N) = sum_{g=1}^{sqrt(N)} g*mu(g) * sum_floor_quotients(N/g^2) - N*(N+1)/2
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

const ll N = 100000000000000LL;  /* 10^14 */
const ll MOD = 1000000007LL;

int L;
int8_t *mu;

ll sum_floor_quotients(ll M) {
    if (M <= 0) return 0;

    ll result = 0;
    ll sqrt_M = (ll)sqrtl((long double)M);
    while ((sqrt_M + 1) * (sqrt_M + 1) <= M) sqrt_M++;
    while (sqrt_M * sqrt_M > M) sqrt_M--;

    for (ll x = 1; x <= sqrt_M; x++) {
        result = (result + (x % MOD) * ((M / x) % MOD)) % MOD;
    }

    for (ll q = 1; q <= sqrt_M; q++) {
        ll x_lo = M / (q + 1) + 1;
        ll x_hi = M / q;
        if (x_lo <= sqrt_M) {
            x_lo = sqrt_M + 1;
        }
        if (x_hi >= x_lo) {
            lll sum_x = ((lll)x_hi * (x_hi + 1) / 2 - (lll)x_lo * (x_lo - 1) / 2) % MOD;
            if (sum_x < 0) sum_x += MOD;
            result = (result + (ll)sum_x * (q % MOD)) % MOD;
        }
    }

    return result;
}

ll tr(ll n) {
    ll n_mod = n % MOD;
    ll np1_mod = (n + 1) % MOD;
    ll inv2 = (MOD + 1) / 2;
    return (lll)n_mod * np1_mod % MOD * inv2 % MOD;
}

int main() {
    L = (int)sqrtl((long double)N) + 1;

    mu = (int8_t *)calloc(L + 1, sizeof(int8_t));
    int *spf = (int *)malloc((L + 1) * sizeof(int));

    mu[1] = 1;
    for (int i = 0; i <= L; i++) spf[i] = i;

    for (int i = 2; i <= L; i++) {
        if (spf[i] == i) {
            mu[i] = -1;
            for (ll j = (ll)i * i; j <= L; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        } else {
            int p = spf[i];
            int q = i / p;
            if (q % p == 0) {
                mu[i] = 0;
            } else {
                mu[i] = -mu[q];
            }
        }
    }

    free(spf);

    ll ans = 0;
    for (int g = 1; g <= L; g++) {
        if (mu[g] != 0) {
            ll M = N / ((ll)g * g);
            if (M > 0) {
                ll sfq = sum_floor_quotients(M);
                lll contribution = (lll)g * mu[g] * sfq % MOD;
                ans = (ans + contribution) % MOD;
            }
        }
    }

    ans = (ans - tr(N)) % MOD;
    if (ans < 0) ans += MOD;

    printf("%lld\n", ans);

    free(mu);
    return 0;
}
