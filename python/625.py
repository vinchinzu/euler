"""Project Euler Problem 625: Gcd sum.

Find the sum_{j=1}^N sum_{i=1}^j gcd(i,j).

Uses the identity: answer = sum_{g=1}^{N} g * S(floor(N/g))
where S(m) = sum_{k=1}^{m} phi(k).

Computes S(m) sub-linearly using inline C with N^{2/3} sieve.
"""

from __future__ import annotations

import os
import subprocess
import tempfile


def solve() -> int:
    """Solve Problem 625."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

#define NN 100000000000LL
#define MOD 998244353LL

ll mod_val(ll a) {
    return ((a % MOD) + MOD) % MOD;
}

ll power(ll base, ll exp) {
    ll result = 1;
    base = mod_val(base);
    while (exp > 0) {
        if (exp & 1) result = result * base % MOD;
        base = base * base % MOD;
        exp >>= 1;
    }
    return result;
}

int main() {
    ll inv2 = power(2, MOD - 2);

    /* Use N^{2/3} sieve limit */
    ll sieve_limit = (ll)(pow((double)NN, 2.0/3.0)) + 100;
    if (sieve_limit < 22000000LL) sieve_limit = 22000000LL;
    int slen = (int)sieve_limit + 1;

    int *phi = (int*)malloc(slen * sizeof(int));
    if (!phi) { fprintf(stderr, "malloc failed\n"); return 1; }
    for (int i = 0; i < slen; i++) phi[i] = i;
    for (int i = 2; i < slen; i++) {
        if (phi[i] == i) {
            for (int j = i; j < slen; j += i)
                phi[j] -= phi[j] / i;
        }
    }

    ll *prefix = (ll*)malloc(slen * sizeof(ll));
    if (!prefix) { fprintf(stderr, "malloc failed\n"); return 1; }
    prefix[0] = 0;
    for (int i = 1; i < slen; i++)
        prefix[i] = (prefix[i-1] + phi[i]) % MOD;

    /* big[t] = S(N/t) for small t where N/t > sieve_limit */
    int big_limit = (int)(NN / sieve_limit) + 2;
    ll *big = (ll*)calloc(big_limit + 1, sizeof(ll));
    if (!big) { fprintf(stderr, "malloc failed\n"); return 1; }

    for (int t = big_limit; t >= 1; t--) {
        ll n = NN / t;
        if (n < slen) {
            big[t] = prefix[n];
            continue;
        }
        ll result = (n % MOD) * ((n + 1) % MOD) % MOD * inv2 % MOD;

        ll d = 2;
        while (d <= n) {
            ll q = n / d;
            ll d2 = n / q;
            ll count = mod_val(d2 - d + 1);
            ll sq;
            if (q < slen)
                sq = prefix[q];
            else
                sq = big[(int)(NN / q)];
            result = mod_val(result - count * sq % MOD);
            d = d2 + 1;
        }
        big[t] = result;
    }

    /* Main computation: ans = sum_{k=1}^{N} k * S(N//k) */
    ll ans = 0;
    ll k = 1;
    while (k <= NN) {
        ll q = NN / k;
        ll k2 = NN / q;
        ll kmod = k % MOD;
        ll k2mod = k2 % MOD;
        ll range_sum = mod_val(kmod + k2mod) * mod_val(k2mod - kmod + 1) % MOD * inv2 % MOD;
        ll sn;
        if (q < slen)
            sn = prefix[q];
        else
            sn = big[(int)(NN / q)];
        ans = mod_val(ans + range_sum * sn % MOD);
        k = k2 + 1;
    }

    printf("%lld\n", ans);

    free(phi);
    free(prefix);
    free(big);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p625.c")
    exe_file = os.path.join(tmpdir, "p625")

    with open(c_file, "w") as f:
        f.write(c_code)

    subprocess.run(["gcc", "-O2", "-o", exe_file, c_file, "-lm"],
                   check=True, capture_output=True)

    result = subprocess.run([exe_file], capture_output=True, text=True, check=True)
    ans = int(result.stdout.strip())

    os.unlink(c_file)
    os.unlink(exe_file)
    os.rmdir(tmpdir)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
