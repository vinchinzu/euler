/*
 * Project Euler 531 - Chinese Leftovers
 *
 * Find sum of g(phi(n), n, phi(m), m) for 1000000 <= n < m < 1005000,
 * where g(a, n, b, m) is the smallest non-negative solution to
 * x = a (mod n) and x = b (mod m).
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static ll ext_gcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    ll x1, y1;
    ll g = ext_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

int main(void) {
    const int N = 1000000;
    const int M = 1005000;

    int *phi = (int*)malloc(M * sizeof(int));
    for (int i = 0; i < M; i++) phi[i] = i;
    for (int i = 2; i < M; i++) {
        if (phi[i] == i) {
            for (int j = i; j < M; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }

    ull ans = 0;

    for (int n = N; n < M; n++) {
        ll a = phi[n];
        for (int m = n + 1; m < M; m++) {
            ll b = phi[m];
            ll g = gcd((ll)n, (ll)m);
            ll diff = b - a;
            if (diff % g != 0) continue;

            ll n_g = (ll)n / g;
            ll m_g = (ll)m / g;
            ll lcm = n_g * (ll)m;

            ll rhs = diff / g;
            ll inv_x, inv_y;
            ext_gcd(n_g, m_g, &inv_x, &inv_y);
            ll k = ((lll)rhs % m_g * (inv_x % m_g) % m_g + m_g) % m_g;
            ll x = ((lll)a + (lll)k * n) % lcm;
            if (x < 0) x += lcm;
            ans += (ull)x;
        }
    }

    printf("%llu\n", ans);
    free(phi);
    return 0;
}
