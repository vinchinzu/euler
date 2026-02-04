#!/usr/bin/env python3
"""Project Euler Problem 427: n-sequences.

Find sum of L(S) over all sequences of length N with values 1..N,
where L(S) is the longest contiguous run of the same value.
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

#define N 7500000
#define MOD 1000000009LL

static ll fact[N + 1];
static ll inv_fact[N + 1];
static ll pow_n[N + 2];
static ll pow_nm1[N + 2];

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

static void precompute() {
    fact[0] = 1;
    for (int i = 1; i <= N; i++)
        fact[i] = fact[i - 1] * i % MOD;

    inv_fact[N] = power(fact[N], MOD - 2, MOD);
    for (int i = N - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD;

    pow_n[0] = 1;
    for (int i = 1; i <= N + 1; i++)
        pow_n[i] = pow_n[i - 1] * N % MOD;

    pow_nm1[0] = 1;
    for (int i = 1; i <= N + 1; i++)
        pow_nm1[i] = pow_nm1[i - 1] * (N - 1) % MOD;
}

static ll nCr(int n, int r) {
    if (r < 0 || r > n) return 0;
    return fact[n] % MOD * inv_fact[r] % MOD * inv_fact[n - r] % MOD;
}

int main() {
    precompute();

    /* f[k] = number of sequences where longest run <= k */
    /* We compute f[k] - f[k-1] incrementally */
    /* ans = sum_{k=1}^{N} (f[k] - f[k-1]) * k = sum_{k=1}^{N} k * delta_k */

    ll ans = 0;
    ll prev_f = 0;

    for (int k = 1; k <= N; k++) {
        ll fk = 0;
        for (int i = 0; ; i++) {
            if ((ll)i * (k + 1) > N) break;
            int A = N - i * k - 1;
            if (A < 0) break;

            ll term = 0;
            /* First part: C(A, i) * (N-1)^i * N^(A-i+1) */
            {
                int exp1 = A - i + 1;
                if (exp1 < 0) exp1 = 0;
                ll t = nCr(A, i);
                t = t * pow_nm1[i] % MOD;
                t = t * pow_n[exp1] % MOD;
                term = (term + t) % MOD;
            }
            /* Second part: C(A, i-1) * (N-1)^(i-1) * N^(A-i+2), only if i >= 1 */
            if (i >= 1) {
                int exp2 = A - i + 2;
                if (exp2 < 0) exp2 = 0;
                ll t = nCr(A, i - 1);
                t = t * pow_nm1[i - 1] % MOD;
                t = t * pow_n[exp2] % MOD;
                term = (term + t) % MOD;
            }

            if (i % 2 == 0)
                fk = (fk + term) % MOD;
            else
                fk = (fk - term + MOD) % MOD;
        }

        ll delta = (fk - prev_f + MOD) % MOD;
        ans = (ans + delta % MOD * k) % MOD;
        prev_f = fk;
    }

    printf("%lld\n", ans);
    return 0;
}
""";

def solve():
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p427.c")
    exe_file = os.path.join(tmpdir, "p427")

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
