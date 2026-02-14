"""Project Euler Problem 820: Nth digit of reciprocal.

Let d_n(x) be the nth decimal digit of the fractional part of x. Find
sum_{k=1}^N d_n(1/k).

Given x, the fractional part of 10^{N-1} x has our desired digit right
after the decimal point. To find it, we multiply by 10 and take the floor.

As a quick optimization, instead of computing 10^{N-1} (mod k) for all k,
we can just take 10^{N-1} (mod 2k) (mod k) if k is small enough.

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base = base % mod;
    while (exp > 0) {
        if (exp & 1)
            result = (result * base) % mod;
        base = (base * base) % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    long long N = 10000000LL;
    long long B = 10;

    long long *pows = (long long *)calloc(N + 1, sizeof(long long));
    if (!pows) { fprintf(stderr, "alloc fail\n"); return 1; }

    long long k;
    for (k = N; k >= 1; k--) {
        if (2 * k <= N) {
            pows[k] = pows[2 * k] % k;
        } else {
            pows[k] = pow_mod(B, N - 1, k);
        }
    }

    long long ans = 0;
    for (k = 1; k <= N; k++) {
        ans += (pows[k] * B) / k;
    }

    printf("%lld\n", ans);
    free(pows);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol820.c")
    exe = os.path.join(tmpdir, "sol820")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
