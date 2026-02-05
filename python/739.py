"""Project Euler Problem 739: Summation of Summations."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define N 100000000
#define M 1000000007LL

typedef long long ll;
typedef __int128 lll;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

int main() {
    // Compute modular inverses using batch inversion
    // inv[i] = (N-1)! / i! * inv((N-1)!) = ...
    // Simpler: use Fermat's little theorem for individual inverses is slow
    // Use the recurrence: inv[i] = -(M/i) * inv[M%i] mod M
    // or precompute factorial and inv_factorial

    ll *mod_invs = malloc((N + 1) * sizeof(ll));
    mod_invs[1] = 1;
    for (int i = 2; i <= N; i++) {
        mod_invs[i] = M - (M / i) * mod_invs[M % i] % M;
    }

    // Lucas sequence
    ll *lucas = malloc((N + 1) * sizeof(ll));
    lucas[1] = 1;
    lucas[2] = 3;
    for (int i = 3; i <= N; i++) {
        lucas[i] = (lucas[i - 2] + lucas[i - 1]) % M;
    }

    ll nCr1 = 1, nCr2 = 1;
    ll ans = lucas[N];

    for (int k = N - 1; k >= 2; k--) {
        ll mult = (ll)(2 * N - 2 - k) * mod_invs[N - k] % M;
        nCr1 = (lll)nCr1 * mult % M;
        ans = (ans + (lll)lucas[k] * ((nCr1 - nCr2 + M) % M)) % M;
        nCr2 = (nCr2 + nCr1) % M;
    }

    printf("%lld\n", ans);

    free(mod_invs);
    free(lucas);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file], check=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    print(result)

if __name__ == "__main__":
    solve()
