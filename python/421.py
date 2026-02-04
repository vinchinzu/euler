#!/usr/bin/env python3
"""Project Euler Problem 421: Prime factors of n^15+1.

Let s(n, m) = sum of distinct prime factors of n^15+1 that are <= m.
Find sum_{n=1}^{10^11} s(n, 10^8).

For each prime p <= K, the contribution is p * (number of n in [1,N] with p | n^15+1).
n^15 ≡ -1 (mod p) iff (-n)^15 ≡ 1 (mod p).

So the roots are n ≡ -r (mod p) where r is a 15th root of unity mod p.
But we also need (-n)^15 = -n^15, so actually n^15 = -1 means n = g^((2k+1)(p-1)/(2*gcd(p-1,30)))
for appropriate generator g.

Simpler: iterate the gcd(p-1,15) roots of unity r, and for each, n = p - r gives
n^15 ≡ (-r)^15 = -1 (mod p). Count of such n in [1,N] is (N + r) / p.

Uses C for performance.
"""
import os
import subprocess
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __uint128_t u128;

#define KMAX 100000001

static char is_composite[KMAX];

void sieve(int limit) {
    memset(is_composite, 0, limit + 1);
    is_composite[0] = is_composite[1] = 1;
    for (int i = 2; (ll)i * i <= limit; i++) {
        if (!is_composite[i]) {
            for (ll j = (ll)i * i; j <= limit; j += i)
                is_composite[j] = 1;
        }
    }
}

static inline ull pow_mod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll gcd_ll(ll a, ll b) { while (b) { ll t = b; b = a % b; a = t; } return a; }

int main() {
    ll N = 100000000000LL;  /* 10^11 */
    int K = 100000000;       /* 10^8 */
    int R = 15;

    sieve(K);

    ll ans = 0;

    for (ll p = 2; p <= K; p++) {
        if (is_composite[p]) continue;

        ll g_val = gcd_ll(p - 1, R);  /* number of 15th roots of 1 mod p */

        /* Find a primitive g_val-th root of unity */
        ull nth_root = 1;
        for (ull g = 1; g < (ull)p; g++) {
            if (g == 1) {
                nth_root = 1;
            } else {
                nth_root = pow_mod(g, (p - 1) / g_val, p);
            }
            /* Check if nth_root has order g_val */
            ull r = nth_root;
            int e = 1;
            while (r != 1) {
                r = (u128)r * nth_root % p;
                e++;
            }
            if (e == g_val) break;
        }

        /* Enumerate 15th roots of 1: nth_root^0, nth_root^1, ..., nth_root^(g_val-1) */
        /* For each root r, n ≡ p - r (mod p) gives n^15 ≡ -1 (mod p) */
        /* Count of n in [1, N]: (N + r) / p */
        ull r = 1;
        for (int e = 0; e < g_val; e++) {
            ans += p * ((N + (ll)r) / p);
            r = (u128)r * nth_root % p;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""

def solve():
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p421.c")
    exe_file = os.path.join(tmpdir, "p421")

    with open(c_file, "w") as f:
        f.write(C_CODE)

    subprocess.run(
        ["gcc", "-O3", "-march=native", "-o", exe_file, c_file, "-lm"],
        check=True, capture_output=True
    )

    result = subprocess.run(
        [exe_file], capture_output=True, text=True, check=True,
        timeout=60
    )
    return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
