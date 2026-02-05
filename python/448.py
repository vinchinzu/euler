#!/usr/bin/env python3
"""
Project Euler 448 - Average least common multiple

S(N) = sum_{k=1}^N A(k) where A(k) = average of LCM(k, i) for 1 <= i <= k

Formula: S(N) = (N + sum_{k=1}^N floor(N/k) * k * phi(k)) / 2

The sum is split into:
- Direct terms for k <= N/L (approximately sqrt(N))
- Grouped terms for q = 1, 2, ..., L-1 where floor(N/k) = q
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

const ll N = 99999999019LL;
const ll MOD = 999999017LL;
int L;
int threshold;  // N / L

int *phi;
ll *k_phi_sum_small;

// Extended Euclidean algorithm for modular inverse
ll mod_inv(ll a, ll m) {
    ll g = m, x = 0, y = 1;
    while (a != 0) {
        ll q = g / a;
        ll t = g - q * a; g = a; a = t;
        t = x - q * y; x = y; y = t;
    }
    return (x % m + m) % m;
}

ll inv6;

// sum of squares: m(m+1)(2m+1)/6 mod MOD
ll sum_sq(ll m) {
    lll mm = m % MOD;
    return (lll)mm * (mm + 1) % MOD * (2 * mm + 1) % MOD * inv6 % MOD;
}

// Hash table for memoization
#define HASH_SIZE 1000003
typedef struct {
    ll key;
    ll val;
    int used;
} HashEntry;
HashEntry *cache;

ll hash_get(ll key, int *found) {
    ull h = (ull)key % HASH_SIZE;
    while (cache[h].used) {
        if (cache[h].key == key) {
            *found = 1;
            return cache[h].val;
        }
        h = (h + 1) % HASH_SIZE;
    }
    *found = 0;
    return 0;
}

void hash_set(ll key, ll val) {
    ull h = (ull)key % HASH_SIZE;
    while (cache[h].used) {
        if (cache[h].key == key) {
            cache[h].val = val;
            return;
        }
        h = (h + 1) % HASH_SIZE;
    }
    cache[h].key = key;
    cache[h].val = val;
    cache[h].used = 1;
}

// Compute T(n) = sum_{k=1}^n k*phi(k) mod MOD using recursion
// Identity: sum_{d=1}^n T(n/d) * d = n(n+1)(2n+1)/6
ll k_phi_sum(ll n) {
    if (n <= 0) return 0;
    if (n <= L) return k_phi_sum_small[n];

    int found;
    ll cached = hash_get(n, &found);
    if (found) return cached;

    ll result = sum_sq(n);
    ll sqrt_n = (ll)sqrtl((long double)n);
    while ((sqrt_n + 1) * (sqrt_n + 1) <= n) sqrt_n++;
    while (sqrt_n * sqrt_n > n) sqrt_n--;

    for (ll d = 2; d <= sqrt_n; d++) {
        result = (result - k_phi_sum(n / d) * (d % MOD)) % MOD;
    }

    for (ll q = 1; q <= sqrt_n; q++) {
        if (n / q > sqrt_n) {
            ll d_lo = n / (q + 1) + 1;
            ll d_hi = n / q;
            lll sum_d = ((lll)d_hi * (d_hi + 1) / 2 - (lll)d_lo * (d_lo - 1) / 2) % MOD;
            if (sum_d < 0) sum_d += MOD;
            result = (result - k_phi_sum(q) * (ll)sum_d) % MOD;
        }
    }

    if (result < 0) result += MOD;
    hash_set(n, result);
    return result;
}

int main() {
    L = (int)sqrtl((long double)N) + 10;
    threshold = N / L;

    inv6 = mod_inv(6, MOD);

    // Compute phi for small values using sieve
    phi = (int *)malloc((L + 1) * sizeof(int));
    for (int i = 0; i <= L; i++) phi[i] = i;
    for (int i = 2; i <= L; i++) {
        if (phi[i] == i) {
            for (int j = i; j <= L; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }

    // Prefix sums of k * phi(k)
    k_phi_sum_small = (ll *)malloc((L + 1) * sizeof(ll));
    k_phi_sum_small[0] = 0;
    for (int k = 1; k <= L; k++) {
        k_phi_sum_small[k] = (k_phi_sum_small[k - 1] + (ll)k * phi[k]) % MOD;
    }

    // Initialize cache
    cache = (HashEntry *)calloc(HASH_SIZE, sizeof(HashEntry));

    // Main computation using the formula from Java:
    // For k from 1 to N/L (threshold), direct computation
    // For q from 1 to L-1, use (T(N/q) - T(N/(q+1))) * q

    ll ans = 0;

    // Direct sum for k = 1 to threshold
    for (int k = 1; k <= threshold; k++) {
        ll term = (ll)((N / k) % MOD) * (k % MOD) % MOD * phi[k] % MOD;
        ans = (ans + term) % MOD;
    }

    // Grouped sum for quotients q = 1 to L-1
    // T(N/q) - T(N/(q+1)) gives sum of k*phi(k) for k where floor(N/k) = q
    // Multiply by q since all these k have floor(N/k) = q
    for (int q = 1; q < L; q++) {
        ll t1 = k_phi_sum(N / q);
        ll t2 = k_phi_sum(N / (q + 1));
        ll diff = (t1 - t2) % MOD;
        if (diff < 0) diff += MOD;
        ans = (ans + diff * (q % MOD)) % MOD;
    }

    // Final: S(N) = (N + ans) / 2
    // ans + N should be even, use integer division
    ans = (ans + N % MOD) % MOD;
    // Now divide by 2 using modular inverse since we're in mod arithmetic
    ll inv2 = mod_inv(2, MOD);
    ans = ans * inv2 % MOD;

    printf("%lld\n", ans);

    free(phi);
    free(k_phi_sum_small);
    free(cache);
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
