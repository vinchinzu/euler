/* Project Euler Problem 748: Upside Down Diophantine Equation.
 * Extracted from embedded C in python/748.py
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

static const ll N = 10000000000000000LL;  /* 10^16 */
static const ll M = 1000000000LL;         /* 10^9 */

static ll ans = 0;

static ll sq(ll x) { return x * x; }

static ll gcd_func(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static void process(ll m, ll n, int g) {
    ll a = sq(m) + sq(n);
    ll b = -2*sq(m) + 6*m*n + 2*sq(n);
    ll c = 3*sq(m) + 4*m*n - 3*sq(n);
    ll x = a * b / g;
    ll y = a * c / g;
    ll z = b * c / g;
    if (y <= N && z <= N && y > 0 && z > 0) {
        ans = (ans + x + y + z) % M;
    }
}

int main(void) {
    double A = (sqrt(6.5) - 2.0) / (3.0 - sqrt(6.5));
    double B = 2.0 / (sqrt(13.0) - 3.0);

    /* Section 1: bound 4*N */
    for (ll n = 1; 8 * sq(n) * sq(n) <= 4 * N; n++) {
        for (ll m = n + 1; m < B * n && (sq(m)+sq(n)) * (3*sq(m)+4*m*n-3*sq(n)) <= 4*N; m++) {
            if (m > A * n && gcd_func(m % n, n) == 1 && (2*m - 3*n) % 13 != 0) {
                process(m, n, (m + n) % 2 == 0 ? 4 : 1);
            }
        }
    }

    /* Section 2: bound 676*N, only m = 8n (mod 13) */
    for (ll n = 1; 8 * sq(n) * sq(n) <= 676 * N; n++) {
        for (ll m = n + (7*n) % 13; m < B * n && (sq(m)+sq(n)) * (3*sq(m)+4*m*n-3*sq(n)) <= 676*N; m += 13) {
            if (m > A * n && gcd_func(m, n) == 1) {
                process(m, n, (m + n) % 2 == 0 ? 676 : 169);
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
