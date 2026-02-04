"""Project Euler Problem 648: Skipping Squares.

Power series expansion of expected number of perfect squares skipped.
Sum of first N+1 coefficients. Uses C for speed.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <string.h>
#include <math.h>

#define NN 1000
#define MOD 1000000000LL

static long long jump1[NN+1], jump2[NN+1], f[NN+1];

static int is_square(int n) {
    int r = (int)sqrt((double)n);
    while (r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r * r == n;
}

static int isqrt_val(int n) {
    int r = (int)sqrt((double)n);
    while (r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r;
}

int main(void) {
    int max_s = (NN / 2) * (NN / 2);

    memset(jump1, 0, sizeof(jump1));
    memset(jump2, 0, sizeof(jump2));

    for (int s = max_s; s >= 0; s--) {
        memset(f, 0, sizeof(f));

        if (s > 0 && is_square(s)) {
            f[0] = isqrt_val(s) - 1;
        } else {
            /* f^s(rho) = rho * f^{s+1}(rho) + (1-rho) * f^{s+2}(rho)
               Coefficient k: f[k] = jump2[k] + jump1[k-1] - jump2[k-1]
               where jump1 = f^{s+1}, jump2 = f^{s+2}
            */
            f[0] = jump2[0];
            for (int k = 1; k <= NN; k++) {
                f[k] = ((jump2[k] + jump1[k-1] - jump2[k-1]) % MOD + MOD) % MOD;
            }
        }

        /* shift: jump2 = jump1, jump1 = f */
        memcpy(jump2, jump1, sizeof(jump1));
        memcpy(jump1, f, sizeof(f));
    }

    long long ans = 0;
    for (int k = 0; k <= NN; k++) {
        ans = (ans + jump1[k]) % MOD;
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
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
