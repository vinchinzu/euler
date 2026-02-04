"""Project Euler Problem 531: Chinese leftovers.

Let g(a, n, b, m) be the smallest non-negative solution to x = a (mod n)
and x = b (mod m). Find sum_{N <= n < m < M} g(phi(n), n, phi(m), m).
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Extended GCD: returns gcd, sets *x, *y so a*x + b*y = gcd */
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

    /* Compute Euler's totient using sieve */
    int *phi = (int*)malloc(M * sizeof(int));
    for (int i = 0; i < M; i++) phi[i] = i;
    for (int i = 2; i < M; i++) {
        if (phi[i] == i) { /* i is prime */
            for (int j = i; j < M; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }

    ull ans = 0;  /* accumulate using unsigned to avoid overflow issues */

    for (int n = N; n < M; n++) {
        ll a = phi[n];
        for (int m = n + 1; m < M; m++) {
            ll b = phi[m];
            ll g = gcd((ll)n, (ll)m);
            ll diff = b - a;
            if (diff % g != 0) continue;

            ll n_g = (ll)n / g;
            ll m_g = (ll)m / g;
            ll lcm = n_g * (ll)m;  /* = lcm(n,m) */

            /* Solve: x = a (mod n), x = b (mod m)
               Need k such that a + k*n = b (mod m)
               k*n = (b-a) (mod m)
               k*(n/g) = (b-a)/g (mod m/g)
               k = (b-a)/g * inverse(n/g, m/g) mod m/g */
            ll rhs = diff / g;
            ll inv_x, inv_y;
            ext_gcd(n_g, m_g, &inv_x, &inv_y);
            /* inv_x is inverse of n_g mod m_g */
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
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path], check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True)
        print(result.stdout.strip())
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
