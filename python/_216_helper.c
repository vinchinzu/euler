
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

ll mulmod(ll a, ll b, ll m) {
    return ((__int128)a * b) % m;
}

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, mod);
        base = mulmod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

ll sqrt_mod(ll n, ll p) {
    if (p % 4 == 3)
        return powmod(n, (p + 1) / 4, p);
    ll Q = p - 1;
    int S = 0;
    while (Q % 2 == 0) { Q /= 2; S++; }
    ll z = 2;
    while (powmod(z, (p - 1) / 2, p) + 1 != p) z++;
    int M = S;
    ll c = powmod(z, Q, p);
    ll t = powmod(n, Q, p);
    ll R = powmod(n, (Q + 1) / 2, p);
    while (t != 1) {
        int i = 1;
        ll tmp = mulmod(t, t, p);
        while (tmp != 1) { tmp = mulmod(tmp, tmp, p); i++; }
        ll b = powmod(c, 1LL << (M - i - 1), p);
        M = i;
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        R = mulmod(R, b, p);
    }
    return R;
}

int main() {
    int N = 50000000;
    int L = (int)(sqrt(2.0) * N);
    char *is_prime = calloc(L + 1, 1);
    memset(is_prime, 1, L + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= L; i++)
        if (is_prime[i])
            for (int j = i * i; j <= L; j += i)
                is_prime[j] = 0;
    char *sieve = calloc(N + 1, 1);
    memset(sieve, 1, N + 1);
    sieve[0] = sieve[1] = 0;
    for (int p = 3; p <= L; p++) {
        if (!is_prime[p]) continue;
        if ((p % 8 != 1) && (p % 8 != 7)) continue;
        ll half = (p + 1) / 2;
        ll r = sqrt_mod(half, p);
        ll start1 = (2 * r * r - 1 == p) ? r + p : r;
        for (ll i = start1; i <= N; i += p) sieve[i] = 0;
        ll r2 = p - r;
        ll start2 = (2 * r2 * r2 - 1 == p) ? r2 + p : r2;
        if (start2 > 0)
            for (ll i = start2; i <= N; i += p) sieve[i] = 0;
    }
    int ans = 0;
    for (int i = 0; i <= N; i++)
        if (sieve[i]) ans++;
    printf("%d\n", ans);
    free(is_prime);
    free(sieve);
    return 0;
}
