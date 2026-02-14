/* Project Euler 726: Falling Bottles.
 * Recurrence for a(n), b(n), and Young tableau formula f(n) = a(n) * tr(n)! / b(n).
 * All mod M = 10^9 + 33.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define NMAX 10000
#define MOD 1000000033LL

ll tr(int n) { return (ll)n * (n + 1) / 2; }

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll mod_inv(ll a, ll m) {
    return pow_mod(a, m - 2, m);
}

int main() {
    int n = NMAX;
    ll m = MOD;
    ll trn = tr(n);

    /* Precompute factorials mod m */
    ll *fact = malloc((trn + 1) * sizeof(ll));
    fact[0] = 1;
    for (ll i = 1; i <= trn; i++)
        fact[i] = (lll)fact[i - 1] * (i % m) % m;

    /* Compute a[i] */
    ll *a = malloc((n + 1) * sizeof(ll));
    a[0] = 1; a[1] = 1;
    for (int i = 2; i <= n; i++) {
        ll sq = (lll)a[i - 1] * a[i - 1] % m;
        ll p2 = (pow_mod(2, i, m) - 1 + m) % m;
        ll inv_prev = mod_inv(a[i - 2], m);
        a[i] = (lll)sq * p2 % m * inv_prev % m;
    }

    /* Compute b[i] */
    ll *b = malloc((n + 1) * sizeof(ll));
    b[0] = 1; b[1] = 1;
    for (int i = 2; i <= n; i++) {
        ll sq = (lll)b[i - 1] * b[i - 1] % m;
        ll odd = (2 * i - 1) % m;
        ll inv_prev = mod_inv(b[i - 2], m);
        b[i] = (lll)sq * odd % m * inv_prev % m;
    }

    /* Sum f(i) for i = 1..n */
    ll ans = 0;
    for (int i = 1; i <= n; i++) {
        ll tri = tr(i);
        ll term = (lll)a[i] * fact[tri] % m;
        term = (lll)term * mod_inv(b[i], m) % m;
        ans = (ans + term) % m;
    }

    printf("%lld\n", ans);

    free(fact);
    free(a);
    free(b);
    return 0;
}
