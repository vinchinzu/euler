/*
 * Project Euler Problem 515: Dissonant Numbers.
 * Sum d(p, p-1, K) for all primes A <= p < A+B.
 * d(p, p-1, K) = modular inverse of (K-1) mod p.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

int is_prime(ll n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    ll sq = (ll)sqrt((double)n);
    for (ll i = 3; i <= sq; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

ll mod_inv(ll a, ll m) {
    if (m == 1) return 0;
    ll m0 = m, x0 = 0, x1 = 1;
    while (a > 1) {
        ll q = a / m;
        ll t = m;
        m = a % m;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if (x1 < 0) x1 += m0;
    return x1;
}

int main() {
    ll A = 1000000000LL;
    ll B = 100000LL;
    ll K = 100000LL;

    ll ans = 0;
    for (ll p = A; p < A + B; p++) {
        if (is_prime(p)) {
            ans += mod_inv(K - 1, p);
        }
    }

    printf("%lld\n", ans);
    return 0;
}
