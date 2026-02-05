#!/usr/bin/env python3
"""
Project Euler 447 - Retractions C

F(N) = sum_{n=2}^N R(n) where R(n) = prod(1 + p^e) - n for factorization n = prod(p^e)

Using Mobius inversion:
F(N) = sum_{g=1}^{sqrt(N)} g*mu(g) * sum_floor_quotients(N/g^2) - N*(N+1)/2

where sum_floor_quotients(M) = sum_{x=1}^M x * floor(M/x)
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

const ll N = 100000000000000LL;  // 10^14
const ll MOD = 1000000007LL;

int L;
int8_t *mu;

// Compute sum_{x=1}^M x * floor(M/x) mod MOD
ll sum_floor_quotients(ll M) {
    if (M <= 0) return 0;

    ll result = 0;
    ll sqrt_M = (ll)sqrtl((long double)M);

    // Ensure sqrt_M is accurate
    while ((sqrt_M + 1) * (sqrt_M + 1) <= M) sqrt_M++;
    while (sqrt_M * sqrt_M > M) sqrt_M--;

    // For x from 1 to sqrt(M)
    for (ll x = 1; x <= sqrt_M; x++) {
        result = (result + (x % MOD) * ((M / x) % MOD)) % MOD;
    }

    // For quotients q from 1 to sqrt(M) where M/q > sqrt(M)
    for (ll q = 1; q <= sqrt_M; q++) {
        ll x_lo = M / (q + 1) + 1;
        ll x_hi = M / q;
        if (x_lo <= sqrt_M) {
            x_lo = sqrt_M + 1;  // Already counted
        }
        if (x_hi >= x_lo) {
            // sum of x from x_lo to x_hi = x_hi*(x_hi+1)/2 - x_lo*(x_lo-1)/2
            lll sum_x = ((lll)x_hi * (x_hi + 1) / 2 - (lll)x_lo * (x_lo - 1) / 2) % MOD;
            if (sum_x < 0) sum_x += MOD;
            result = (result + (ll)sum_x * (q % MOD)) % MOD;
        }
    }

    return result;
}

// Triangular number n*(n+1)/2 mod MOD
ll tr(ll n) {
    ll n_mod = n % MOD;
    ll np1_mod = (n + 1) % MOD;
    ll inv2 = (MOD + 1) / 2;
    return (lll)n_mod * np1_mod % MOD * inv2 % MOD;
}

int main() {
    L = (int)sqrtl((long double)N) + 1;

    // Allocate arrays
    mu = (int8_t *)calloc(L + 1, sizeof(int8_t));
    int *spf = (int *)malloc((L + 1) * sizeof(int));

    // Compute Mobius function
    mu[1] = 1;
    for (int i = 0; i <= L; i++) spf[i] = i;

    for (int i = 2; i <= L; i++) {
        if (spf[i] == i) {  // prime
            mu[i] = -1;
            for (ll j = (ll)i * i; j <= L; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        } else {
            int p = spf[i];
            int q = i / p;
            if (q % p == 0) {
                mu[i] = 0;
            } else {
                mu[i] = -mu[q];
            }
        }
    }

    free(spf);

    // Main computation
    ll ans = 0;
    for (int g = 1; g <= L; g++) {
        if (mu[g] != 0) {
            ll M = N / ((ll)g * g);
            if (M > 0) {
                ll sfq = sum_floor_quotients(M);
                lll contribution = (lll)g * mu[g] * sfq % MOD;
                ans = (ans + contribution) % MOD;
            }
        }
    }

    // Subtract triangular number N*(N+1)/2
    ans = (ans - tr(N)) % MOD;
    if (ans < 0) ans += MOD;

    printf("%lld\n", ans);

    free(mu);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.check_output([exe]).decode().strip()
        print(result)
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)

if __name__ == "__main__":
    solve()
