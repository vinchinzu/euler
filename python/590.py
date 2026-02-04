"""Project Euler Problem 590: Sets with LCM.

H(n) = number of non-empty sets of positive integers with lcm = n.
HL(n) = H(L(n)) where L(n) = lcm(1,...,n).
Find HL(50000) mod 10^9.

Key insight: L(n) = prod_{p prime <= n} p^floor(log_p(n)).
H(n) = sum_{S} (-1)^|S| * (2^(prod a_i) - 1) where a_i = e_i+1 if not in S, e_i if in S.
Split primes into big (e>=2) and small (e=1). DP over big primes tracks product mod phi(5^9).
Small primes are batched analytically using binomial coefficients.
"""

import subprocess, tempfile, os
from sympy import primerange

def solve():
    N = 50000
    M = 10**9

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define MOD 1000000000LL
#define MOD5 1953125LL
#define PHI5 1562500

ll power(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

ll crt(ll a2, ll a5) {
    static ll inv357 = 0;
    if (inv357 == 0) inv357 = power(357, 255, 512);
    ll diff = ((a2 - a5) % 512 + 512) % 512;
    ll t = diff * inv357 % 512;
    ll result = (a5 + 1953125LL * t) % MOD;
    if (result < 0) result += MOD;
    return result;
}

int main() {
    int nbig, m_small;
    scanf("%d %d", &nbig, &m_small);
    int *big_e = (int *)malloc(nbig * sizeof(int));
    for (int i = 0; i < nbig; i++) scanf("%d", &big_e[i]);

    /* Precompute pow2_tab[x] = 2^x mod 5^9 */
    ll *pow2_tab = (ll *)malloc(PHI5 * sizeof(ll));
    pow2_tab[0] = 1;
    for (int i = 1; i < PHI5; i++)
        pow2_tab[i] = pow2_tab[i-1] * 2 % MOD5;

    /* DP over big primes: track product mod PHI5, signed coefficient mod MOD5 */
    ll *dp = (ll *)calloc(PHI5, sizeof(ll));
    dp[1] = 1;

    for (int i = 0; i < nbig; i++) {
        int ep = big_e[i];
        ll *ndp = (ll *)calloc(PHI5, sizeof(ll));
        for (int x = 0; x < PHI5; x++) {
            if (dp[x] == 0) continue;
            int nx0 = (int)((ll)x * (ep + 1) % PHI5);
            int nx1 = (int)((ll)x * ep % PHI5);
            ndp[nx0] = (ndp[nx0] + dp[x]) % MOD5;
            ndp[nx1] = (ndp[nx1] - dp[x] + MOD5) % MOD5;
        }
        free(dp);
        dp = ndp;
    }

    /* Compute coeff[k] = (-1)^(m-k) * C(m,k) mod 5^9 */
    ll *coeff = (ll *)malloc((m_small + 1) * sizeof(ll));
    {
        int vv = 0;
        ll unit = 1;
        for (int k = 0; k <= m_small; k++) {
            ll bval;
            if (k == 0) { bval = 1; }
            else {
                int num = m_small - k + 1, den = k;
                int v_num = 0, v_den = 0;
                int nn = num, dd = den;
                while (nn % 5 == 0) { nn /= 5; v_num++; }
                while (dd % 5 == 0) { dd /= 5; v_den++; }
                vv += v_num - v_den;
                unit = unit * nn % MOD5;
                unit = unit * power(dd, MOD5 - MOD5/5 - 1, MOD5) % MOD5;
                bval = (vv >= 9) ? 0 : unit * power(5, vv, MOD5) % MOD5;
            }
            int sign = ((m_small - k) % 2 == 0) ? 1 : -1;
            coeff[k] = (sign * bval % MOD5 + MOD5) % MOD5;
        }
    }

    /* Compute H5 = sum_x dp[x] * sum_k coeff[k] * pow2_tab[x * 2^k mod PHI5] */
    ll H5 = 0;
    for (int x = 0; x < PHI5; x++) {
        if (dp[x] == 0) continue;
        ll s = 0;
        int ak = x;
        for (int k = 0; k <= m_small; k++) {
            if (coeff[k] != 0)
                s = (s + coeff[k] * pow2_tab[ak]) % MOD5;
            ak = ak * 2 % PHI5;
        }
        H5 = (H5 + dp[x] * s) % MOD5;
    }
    H5 = (H5 % MOD5 + MOD5) % MOD5;

    /* H mod 512 = 0 (since 2^X >= 2^9 for all X, and sum of signs = 0) */
    printf("%lld\n", crt(0, H5));

    free(pow2_tab); free(dp); free(coeff); free(big_e);
    return 0;
}
"""

    primes_list = list(primerange(2, N + 1))
    big_primes = []
    m_small = 0
    for p in primes_list:
        e = 0
        pk = p
        while pk <= N:
            e += 1
            if pk > N // p: break
            pk *= p
        if e >= 2:
            big_primes.append(e)
        else:
            m_small += 1

    input_data = f"{len(big_primes)} {m_small}\n"
    input_data += " ".join(str(e) for e in big_primes) + "\n"

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        subprocess.run(["gcc", "-O2", "-o", exe, src], check=True, capture_output=True)
        result = subprocess.run([exe], input=input_data, capture_output=True, text=True, timeout=28)
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
