/* Project Euler 735: Divisors of 2n^2.
 * Mobius function sieve + counting lattice points in O(N^{2/3}) per term.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdint.h>

typedef long long ll;

#define LIMIT 1000001  /* sqrt(10^12) + 1 */

static int mobius[LIMIT];

void pre_mobius(int limit) {
    static char is_prime[LIMIT];
    for (int i = 0; i <= limit; i++) { mobius[i] = 1; is_prime[i] = 1; }
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j != i) is_prime[j] = 0;
                mobius[j] *= -1;
            }
            for (ll j = (ll)i * i; j <= limit; j += (ll)i * i) {
                mobius[j] = 0;
            }
        }
    }
}

static inline ll isq(ll n) { return n * n; }
static inline ll icb(ll n) { return n * n * n; }

int main() {
    ll N = 1000000000000LL; /* 10^12 */
    ll L = (ll)sqrt((double)N);
    if (L * L > N) L--;
    while ((L+1)*(L+1) <= N) L++;

    pre_mobius((int)L);

    ll ans = N;

    for (ll g = 1; isq(g) < N; g++) {
        if (mobius[g] == 0) continue;

        int t = 0;
        while (isq(g) * (1LL << t) <= N) {
            ll n_val = (N / isq(g)) >> t;
            if (n_val < 1) { t++; continue; }

            ll res = 0;

            /* x*y*z <= n_val, y > x */
            {
                ll cbrt_n = (ll)cbrt((double)n_val);
                while (icb(cbrt_n + 1) <= n_val) cbrt_n++;
                while (icb(cbrt_n) > n_val) cbrt_n--;

                for (ll x = 1; icb(x) <= n_val; x++) {
                    ll nox = n_val / x;
                    ll sq_nox = (ll)sqrt((double)nox);
                    while (sq_nox * sq_nox > nox) sq_nox--;
                    while ((sq_nox+1)*(sq_nox+1) <= nox) sq_nox++;

                    for (ll y = x + 1; y <= sq_nox; y++) {
                        res += n_val / (x * y) - y;
                    }
                }

                for (ll x = 1; icb(x) <= n_val; x++) {
                    ll nox = n_val / x;
                    ll sq_nox = (ll)sqrt((double)nox);
                    while (sq_nox * sq_nox > nox) sq_nox--;
                    while ((sq_nox+1)*(sq_nox+1) <= nox) sq_nox++;

                    for (ll z = x + 1; z <= sq_nox; z++) {
                        res += n_val / (x * z) - (z - 1);
                    }
                }

                for (ll z = 1; icb(z) <= n_val; z++) {
                    ll noz = n_val / z;
                    ll sq_noz = (ll)sqrt((double)noz);
                    while (sq_noz * sq_noz > noz) sq_noz--;
                    while ((sq_noz+1)*(sq_noz+1) <= noz) sq_noz++;

                    for (ll x = z; x <= sq_noz; x++) {
                        res += n_val / (x * z) - x;
                    }
                }
            }

            /* x*y*z <= n_val, y > 2x */
            {
                for (ll x = 1; icb(x) <= n_val; x++) {
                    ll nox = n_val / x;
                    ll sq_nox = (ll)sqrt((double)nox);
                    while (sq_nox * sq_nox > nox) sq_nox--;
                    while ((sq_nox+1)*(sq_nox+1) <= nox) sq_nox++;

                    for (ll y = 2 * x + 1; y <= sq_nox; y++) {
                        res += n_val / (x * y) - y;
                    }
                }

                for (ll x = 1; icb(x) <= n_val; x++) {
                    ll nox = n_val / x;
                    ll sq_nox = (ll)sqrt((double)nox);
                    while (sq_nox * sq_nox > nox) sq_nox--;
                    while ((sq_nox+1)*(sq_nox+1) <= nox) sq_nox++;

                    for (ll z = x + 1; z <= sq_nox; z++) {
                        if (2 * z * isq(x) > n_val) break;
                        ll maxv = 2 * x;
                        if (z - 1 > maxv) maxv = z - 1;
                        res += n_val / (x * z) - maxv;
                    }
                }

                for (ll z = 1; icb(z) <= n_val; z++) {
                    ll half_noz = n_val / (2 * z);
                    ll sq_half = (ll)sqrt((double)half_noz);
                    while (sq_half * sq_half > half_noz) sq_half--;
                    while ((sq_half+1)*(sq_half+1) <= half_noz) sq_half++;

                    for (ll x = z; 2 * z * isq(x) <= n_val; x++) {
                        res += n_val / (x * z) - 2 * x;
                    }
                }
            }

            int parity = (t % 2 == 0) ? 1 : -1;
            ans += res * parity * mobius[g];
            t++;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
