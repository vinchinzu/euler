/* Project Euler Problem 747: Triangular Pizza.
 * Translated from python/747.py
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

ll ncr(ll n, ll r, ll mod) {
    if (r < 0 || r > n) return 0;
    if (r == 0 || r == n) return 1;
    if (r > n - r) r = n - r;
    ll result = 1;
    for (ll i = 0; i < r; i++) {
        result = (lll)result * ((n - i) % mod) % mod;
        result = (lll)result * pow_mod(i + 1, mod - 2, mod) % mod;
    }
    return result;
}

ll tr(ll n) {
    return n * (n + 1) / 2;
}

ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll r = (ll)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

int is_sq(ll n) {
    ll r = isqrt_ll(n);
    return r * r == n;
}

int main() {
    ll n = 100000000LL; /* 10^8 */
    ll m = MOD;

    ll ans = ncr(n, 3, m);
    ans = (ans + 6 * (tr(n - 2) % m)) % m;

    ll sqrt_2n = isqrt_ll(2 * n);
    for (ll a = 1; a <= sqrt_2n; a++) {
        ll min_n = (2 * a + 1) * (2 * a + 1);
        if (min_n <= n) {
            ans = (ans + 6 * ((n - min_n) % m) % m + 3) % m;
        }

        ll b = a + 1;
        while (1) {
            ll prod = 4LL * (a + 1) * (b + 1) * a * b;
            ll sq_root = isqrt_ll(prod);
            ll min_n2 = (a + 1) * (b + 1) + a * b + sq_root;
            if (min_n2 > n) break;
            ans = (ans + 12 * ((n - min_n2) % m)) % m;
            if (sq_root * sq_root == prod) {
                ans = (ans + 6) % m;
            }
            b++;
        }
    }

    printf("%lld\n", (ans % m + m) % m);
    return 0;
}
