/*
 * Project Euler Problem 805: Shifted Multiples.
 *
 * n = a*v3*(10^{k+1} - 1) / c where c = 10*v3 - u3.
 * k+1 = ord(10, c / gcd(a*v3, c)).
 * Sum N(u^3/v^3) mod 10^9+7 for coprime u,v <= 200.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef unsigned long long ull;

static const ll MOD = 1000000007LL;

static ll gcd(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static ll power(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Euler's totient function */
static ll euler_totient(ll n) {
    ll result = n;
    for (ll p = 2; p * p <= n; p++) {
        if (n % p == 0) {
            while (n % p == 0) n /= p;
            result -= result / p;
        }
    }
    if (n > 1) result -= result / n;
    return result;
}

/* Find multiplicative order of base mod m, assuming gcd(base, m) = 1 */
static ll mult_order(ll base, ll m) {
    if (m == 1) return 1;
    ll phi = euler_totient(m);

    /* Find prime factorization of phi */
    ll temp = phi;
    ll factors[64];
    int nfactors = 0;
    for (ll p = 2; p * p <= temp; p++) {
        if (temp % p == 0) {
            factors[nfactors++] = p;
            while (temp % p == 0) temp /= p;
        }
    }
    if (temp > 1) factors[nfactors++] = temp;

    ll ord = phi;
    for (int i = 0; i < nfactors; i++) {
        while (ord % factors[i] == 0) {
            if (power(base, ord / factors[i], m) == 1)
                ord /= factors[i];
            else
                break;
        }
    }
    return ord;
}

int main(void) {
    int N = 200;
    ll ans = 0;

    for (int v = 1; v <= N; v++) {
        for (int u = 1; u <= N; u++) {
            if (gcd(u, v) != 1) continue;

            ll u3 = (ll)u * u * u;
            ll v3 = (ll)v * v * v;
            ll c = 10 * v3 - u3;
            if (c <= 0) continue;

            ll best_k = -1;
            int best_a = 0;

            for (int a = 1; a <= 9; a++) {
                if ((ll)a * u3 >= c) continue;

                ll g = gcd((ll)a * v3, c);
                ll d = c / g;

                if (d == 1) {
                    ll k = 0;
                    if (best_k < 0 || k < best_k || (k == best_k && a < best_a)) {
                        best_k = k;
                        best_a = a;
                    }
                    continue;
                }

                if (gcd(10, d) != 1) continue;

                ll k = mult_order(10, d) - 1;
                if (best_k < 0 || k < best_k || (k == best_k && a < best_a)) {
                    best_k = k;
                    best_a = a;
                }
            }

            if (best_k >= 0) {
                ll k1 = best_k + 1;
                ll p10 = power(10, k1, MOD);
                ll num = (ll)best_a % MOD * (v3 % MOD) % MOD;
                num = num * ((p10 - 1 + MOD) % MOD) % MOD;
                ll inv_c = power(c % MOD, MOD - 2, MOD);
                num = num * inv_c % MOD;
                ans = (ans + num) % MOD;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
