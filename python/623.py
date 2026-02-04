"""Project Euler Problem 623: Lambda Terms.

Count closed lambda terms with at most N=2000 characters (mod 10^9+7),
where alpha-equivalent terms are identified.

DP on (num_chars, num_bound_vars). Iterate b from high to low.
Uses C for speed.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <string.h>

#define NN 2000
#define MOD 1000000007LL
#define MAXB 401

/* T[c][b] = number of lambda terms with exactly c chars and b bound vars in scope */
static long long T[NN+1][MAXB];

int main(void) {
    memset(T, 0, sizeof(T));

    /* Iterate b from high to low.
       T[1][b] = b
       T[c][b] = T[c-5][b+1]  (abstraction, if c>=6 and b+1<MAXB)
               + sum_{l=1}^{c-3} T[l][b] * T[c-2-l][b]  (application, if c>=4)

       For a fixed b, the convolution only uses T[*][b] at the same b level,
       and T[c-5][b+1] which was computed in the previous (higher b) iteration.
    */

    for (int b = MAXB - 1; b >= 0; b--) {
        T[1][b] = b % MOD;

        for (int c = 2; c <= NN; c++) {
            long long val = 0;

            /* Abstraction: costs 5 chars, adds 1 bound var */
            if (c >= 6 && b + 1 < MAXB) {
                val = T[c-5][b+1];
            }

            /* Application: costs 2 chars, split c-2 chars into two parts */
            if (c >= 4) {
                int rem = c - 2;
                long long conv = 0;
                for (int l = 1; l < rem; l++) {
                    conv = (conv + T[l][b] % MOD * (T[rem - l][b] % MOD)) % MOD;
                }
                val = (val + conv) % MOD;
            }

            T[c][b] = val;
        }
    }

    long long ans = 0;
    for (int c = 1; c <= NN; c++) {
        ans = (ans + T[c][0]) % MOD;
    }
    printf("%lld\n", ans);
    return 0;
}
""";

    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
