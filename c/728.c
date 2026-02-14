/* Project Euler 728: Circle of Coins.
 * Sum 2^{(i-1)*g} * mult(i) for g=1..N/i, using Euler totient sieve.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define N 10000000
#define MOD 1000000007LL

static int phi[N + 1];
static ll pow2s[N + 1];

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll mod_inv(ll a, ll m) {
    /* Extended Euclidean */
    ll t = 0, new_t = 1;
    ll r = m, new_r = a % m;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

int main() {
    /* Sieve phi */
    for (int i = 0; i <= N; i++) phi[i] = i;
    for (int i = 2; i <= N; i++) {
        if (phi[i] == i) { /* prime */
            for (int j = i; j <= N; j += i)
                phi[j] -= phi[j] / i;
        }
    }

    /* Precompute powers of 2 mod M */
    pow2s[0] = 1;
    for (int i = 1; i <= N; i++)
        pow2s[i] = pow2s[i - 1] * 2 % MOD;

    /* Threshold for geometric series optimization */
    int l = N / 30;  /* approximate: M.bit_length() - 1 ~ 30 */

    ll ans = 0;
    for (int i = 1; i <= N; i++) {
        ll res = 0;
        if (i > 1 && i < l) {
            /* Geometric series: sum_{g=1}^{N/i} 2^{(i-1)*g} */
            ll base = pow2s[i - 1];
            int max_g = N / i;
            if (base == 1) {
                res = max_g % MOD;
            } else {
                ll numerator = (pow_mod(base, max_g + 1, MOD) - 1 + MOD) % MOD;
                ll denominator = mod_inv((base - 1 + MOD) % MOD, MOD);
                res = ((lll)numerator * denominator % MOD - 1 + MOD) % MOD;
            }
        } else {
            /* Direct computation */
            for (int g = 1; g <= N / i; g++) {
                ll idx = (ll)(i - 1) * g;
                if (idx <= N)
                    res = (res + pow2s[idx]) % MOD;
                else
                    res = (res + pow_mod(2, idx, MOD)) % MOD;
            }
        }

        ll multiplier;
        if (i == 1 || i % 2 == 0)
            multiplier = 2LL * phi[i] % MOD;
        else
            multiplier = 3LL * phi[i] / 2 % MOD;

        ans = (ans + (lll)res % MOD * multiplier) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
