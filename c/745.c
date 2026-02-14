/* Project Euler Problem 745: Sum of Squares.
 * Translated from python/745.py
 *
 * Find sum_{n=1}^N g(n), where g(n) is the largest perfect square dividing n.
 * Uses sieve for square-free counts and Lucy DP for large values.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

static ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll r = (ll)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

static ll icbrt_ll(ll n) {
    if (n <= 0) return 0;
    ll r = (ll)cbrt((double)n);
    while (r > 0 && r * r * r > n) r--;
    while ((r + 1) * (r + 1) * (r + 1) <= n) r++;
    return r;
}

int main() {
    ll n = 100000000000000LL; /* 10^14 */
    ll m = MOD;

    ll l = (ll)pow((double)n, 2.0 / 7.0);
    /* Adjust l to be safe */
    while ((l + 1) * (l + 1) * (l + 1) <= n) l++;

    ll l2 = n / (l * l);

    /* Sieve square-free numbers up to l2 */
    char *is_sq_free = (char *)malloc(l2 + 1);
    memset(is_sq_free, 1, l2 + 1);
    ll sq_lim = isqrt_ll(l2);
    for (ll i = 2; i <= sq_lim; i++) {
        ll isq = i * i;
        for (ll j = isq; j <= l2; j += isq)
            is_sq_free[j] = 0;
    }

    /* Prefix sums of square-free counts */
    ll *small = (ll *)malloc((l2 + 1) * sizeof(ll));
    small[0] = 0;
    for (ll i = 1; i <= l2; i++)
        small[i] = small[i - 1] + is_sq_free[i];

    free(is_sq_free);

    /* big[i] = number of square-free numbers up to n/(i*i) */
    ll *big = (ll *)calloc(l + 1, sizeof(ll));
    for (ll i = l; i >= 1; i--) {
        ll n_val = n / (i * i);
        big[i] = n_val;
        ll lim = icbrt_ll(n_val);
        ll sq_lim2 = isqrt_ll(n_val / (lim > 0 ? lim : 1));
        for (ll k = 2; k <= sq_lim2; k++) {
            ll k_sq = k * k;
            if (i * k <= l) {
                big[i] -= big[i * k];
            } else {
                big[i] -= small[n_val / k_sq];
            }
        }
        for (ll t = 1; t < lim; t++) {
            big[i] -= (isqrt_ll(n_val / t) - isqrt_ll(n_val / (t + 1))) * small[t];
        }
    }

    ll ans = 0;
    ll k = 1;
    while (k * k <= n) {
        ll k_sq = k * k % m;
        if (k <= l) {
            ans = (ans + (lll)k_sq % m * (big[k] % m)) % m;
        } else {
            ans = (ans + (lll)k_sq % m * (small[n / (k * k)] % m)) % m;
        }
        k++;
    }

    printf("%lld\n", (ans % m + m) % m);

    free(small);
    free(big);
    return 0;
}
