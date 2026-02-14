/*
 * Project Euler 443 - GCD sequence
 *
 * g(4) = 13, g(n) = g(n-1) + gcd(n, g(n-1))
 * Find g(N) for N = 10^15.
 *
 * Key insight: consecutive differences are 1 until gcd(n', g(n')) > 1.
 * We can jump ahead by factoring g(n) - (n+1) and finding divisors.
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

typedef long long ll;

static ll gcd_ll(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Simple trial division factorization */
static int factorize(ll n, ll *factors) {
    int nf = 0;
    if (n < 0) n = -n;
    for (ll d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            factors[nf++] = d;
            while (n % d == 0) n /= d;
        }
    }
    if (n > 1) factors[nf++] = n;
    return nf;
}

int main(void) {
    const ll N = 1000000000000000LL; /* 10^15 */
    const int L = 1000;

    ll ans = 13;
    ll n = 4;

    while (n < N) {
        /* Try small increments first */
        int found = 0;
        for (int d = 0; d < L; d++) {
            if (gcd_ll(ans + d, n + d + 1) > 1) {
                ans = ans + d;
                n += d + 1;
                ans += gcd_ll(n, ans);
                found = 1;
                break;
            }
        }

        if (!found) {
            ll diff = ans - (n + 1);
            if (diff == 0) {
                n += 1;
                ans += 1;
            } else {
                ll factors[64];
                int nf = factorize(diff < 0 ? -diff : diff, factors);
                ll next_val = ans + N - n - 1;
                for (int i = 0; i < nf; i++) {
                    ll p = factors[i];
                    ll candidate = ((ans / p) + 1) * p;
                    if (candidate < next_val)
                        next_val = candidate;
                }
                ll jump = next_val - ans;
                n += jump + 1;
                ans = next_val + gcd_ll(n, next_val);
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
