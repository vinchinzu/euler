"""Project Euler Problem 602: Product of Head Counts.

Embedded C solution for speed. Loop from t=0 to K=4M computing t^N mod M,
binomial coefficients, and accumulating the answer mod 10^9+7.
"""
import subprocess, tempfile, os, sys

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD 1000000007LL
#define NN 10000000LL
#define K 4000000

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

int main(void) {
    /* Precompute modular inverses 1..K */
    ll *inv = malloc((K + 1) * sizeof(ll));
    if (!inv) return 1;
    inv[1] = 1;
    for (int i = 2; i <= K; i++)
        inv[i] = (MOD - (MOD / i) * inv[MOD % i] % MOD) % MOD;

    /* Compute binomial coefficients nCr(N+1, j) for j=0..K */
    ll *nCr = malloc((K + 1) * sizeof(ll));
    if (!nCr) return 1;
    nCr[0] = 1;
    for (int i = 1; i <= K; i++)
        nCr[i] = nCr[i - 1] % MOD * ((NN + 2 - i) % MOD + MOD) % MOD * inv[i] % MOD;

    /* Compute t^N mod MOD for t=0..K */
    ll *pows = malloc((K + 1) * sizeof(ll));
    if (!pows) return 1;
    pows[0] = 0;
    for (int t = 1; t <= K; t++)
        pows[t] = power((ll)t, NN, MOD);

    ll ans = 0;
    for (int t = 0; t <= K; t++) {
        ll sign = ((K - t) & 1) ? MOD - 1 : 1;
        ll coeff = nCr[K - t] * sign % MOD * pows[t] % MOD;
        ans = (ans + coeff) % MOD;
    }

    printf("%lld\n", ans);

    free(inv);
    free(nCr);
    free(pows);
    return 0;
}
"""

def main():
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                timeout=280)
        print(result.stdout.strip())
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    main()
