"""Project Euler Problem 559: Permutation matrices.

Compute Q(50000) mod 1000000123.
Uses C extension for the heavy computation.
"""

import subprocess, tempfile, os

def solve():
    N = 50000
    M = 1000000123

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

#define NVAL 50000
#define MOD 1000000123LL

ll factorials[NVAL+1];
ll inv_factorials[NVAL+1];
ll pow_inv_fact[NVAL+1];
ll dp[NVAL+2];
int parts[NVAL+2];

ll power(ll base, ll exp, ll mod) {
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

int main() {
    int N = NVAL;
    ll M = MOD;

    factorials[0] = 1;
    for (int i = 1; i <= N; i++)
        factorials[i] = factorials[i-1] * i % M;

    inv_factorials[N] = power(factorials[N], M - 2, M);
    for (int i = N - 1; i >= 0; i--)
        inv_factorials[i] = inv_factorials[i+1] * (i+1) % M;

    for (int i = 0; i <= N; i++)
        pow_inv_fact[i] = power(inv_factorials[i], N, M);

    ll ans = 0;

    for (int k = 1; k <= N; k++) {
        /* Build parts array: 0, k, 2k, ..., last < N, then N */
        int np = 0;
        for (int v = 0; v < N; v += k)
            parts[np++] = v;
        parts[np++] = N;
        /* np = num_parts */

        dp[0] = 1;
        for (int i = 1; i < np; i++) {
            ll val = 0;
            for (int s = 0; s < i; s++) {
                int length = parts[i] - parts[s];
                val = (val - pow_inv_fact[length] % M * (dp[s] % M)) % M;
            }
            dp[i] = (val % M + M) % M;
        }

        /* parity(np) = (-1)^np */
        /* result = -parity(np) * dp[np-1] = -(-1)^np * dp[np-1] = (-1)^(np+1) * dp[np-1] */
        ll sign = ((np + 1) % 2 == 0) ? 1 : -1;
        ll pk = (sign * (ll)dp[np-1]) % M;
        if (pk < 0) pk += M;
        ans = (ans + pk) % M;
    }

    ans = ans * power(factorials[N], N, M) % M;
    printf("%lld\n", ans);
    return 0;
}
"""

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        subprocess.run(["gcc", "-O2", "-o", exe, src], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=30)
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
