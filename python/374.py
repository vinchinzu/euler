#!/usr/bin/env python3
"""
Project Euler Problem 374: Maximum Integer Partition Product — Embedded C version.

For partitions into distinct parts, find sum of f(n)*m(n) for 1 <= n <= 10^14 mod 982451653.
"""

import subprocess, tempfile, os

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>

#define MOD 982451653ULL

typedef unsigned long long ull;
typedef long long ll;

int main(void) {
    ll N = 100000000000000LL; /* 10^14 */

    /* Find K such that T_K <= N < T_{K+1} */
    ll K = 1;
    {
        /* Approximate: K ~ sqrt(2N) */
        ll lo = 1, hi = 20000000;
        while (lo < hi) {
            ll mid = lo + (hi - lo + 1) / 2;
            /* T_mid = mid*(mid+1)/2 */
            if ((__int128)mid * (mid + 1) / 2 <= N) lo = mid;
            else hi = mid - 1;
        }
        K = lo;
    }

    ll max_k = K + 10;

    /* Precompute factorials mod MOD */
    ll sz = max_k + 3;
    ull *fact = (ull *)malloc(sz * sizeof(ull));
    fact[0] = 1;
    for (ll i = 1; i < sz; i++)
        fact[i] = fact[i-1] * (ull)i % MOD;

    /* Precompute modular inverses using linear sieve */
    ull *inv = (ull *)malloc(sz * sizeof(ull));
    inv[0] = 0;
    inv[1] = 1;
    for (ll i = 2; i < sz; i++)
        inv[i] = (MOD - MOD / (ull)i) * inv[MOD % (ull)i] % MOD;

    /* Precompute harmonic sums: harmonic_sum[k] = sum_{j=2}^{k} 1/j mod MOD */
    ull *harmonic = (ull *)malloc(sz * sizeof(ull));
    harmonic[0] = 0;
    harmonic[1] = 0;
    for (ll i = 2; i < sz; i++)
        harmonic[i] = (harmonic[i-1] + inv[i]) % MOD;

    ull total = 0;

    /* Handle k=1: n=1,2 */
    if (N >= 1) total = (total + 1) % MOD;
    if (N >= 2) total = (total + 2) % MOD;

    /* Handle k=2: n=3,4,5 */
    if (N >= 3) total = (total + 3) % MOD;
    if (N >= 4) total = (total + 4) % MOD;
    if (N >= 5) total = (total + 12) % MOD;

    ull inv2 = inv[2];

    for (ll k = 3; k <= K; k++) {
        ll T_k = k * (k + 1) / 2;
        ll r_max;
        if (k < K) r_max = k;
        else r_max = N - T_k;
        if (r_max > k) r_max = k;

        /* Case 1: r from 0 to min(k-2, r_max) */
        ll r1 = k - 2;
        if (r1 > r_max) r1 = r_max;
        if (r1 >= 0) {
            ll j_min = k - r1;
            ull sum_inv;
            if (j_min <= 1) {
                sum_inv = (1 + harmonic[k]) % MOD;
            } else {
                sum_inv = (harmonic[k] - harmonic[j_min - 1] + MOD) % MOD;
            }
            ull contrib = fact[k + 1] % MOD * ((ull)(k - 1) % MOD) % MOD * sum_inv % MOD;
            total = (total + contrib) % MOD;
        }

        /* Case 2: r = k-1 */
        if (r_max >= k - 1) {
            ull contrib = fact[k + 2] % MOD * ((ull)(k - 1) % MOD) % MOD;
            contrib = contrib * inv2 % MOD;
            contrib = contrib * inv[k + 1] % MOD;
            total = (total + contrib) % MOD;
        }

        /* Case 3: r = k */
        if (r_max >= k) {
            ull contrib = fact[k + 1] % MOD * ((ull)k % MOD) % MOD;
            total = (total + contrib) % MOD;
        }
    }

    printf("%llu\n", total);
    return 0;
}
"""

def solve():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p374.c")
        exe = os.path.join(tmpdir, "p374")
        with open(src, "w") as f:
            f.write(C_CODE)
        subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
