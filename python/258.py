"""Project Euler Problem 258 — A lagged Fibonacci sequence.

Find g_N (mod M), where g_k = 1 for 0 <= k < K and
g_k = g_(k-K) + g_(k-K+1) for k >= K.
K=2000, N=10^18, M=20092010.

Uses polynomial exponentiation: x^N mod (x^K - x - 1), evaluated at x=1.
Ported to embedded C for speed.
"""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define K 2000
#define MOD 20092010LL

/* Polynomial represented as array of K coefficients (degree < K) */
/* After reduction, all polys have degree < K */

static long long tmp[2*K]; /* temp for multiplication */

/* Multiply a[0..K-1] * b[0..K-1] into tmp[0..2K-2], then reduce mod (x^K - x - 1) */
/* The modular polynomial is x^K = x + 1 (mod MOD) */
/* So x^{K+i} = x^{i+1} + x^i */
static void poly_mul_mod(long long *a, long long *b, long long *out) {
    int i, j;
    memset(tmp, 0, sizeof(long long) * (2*K - 1));

    for (i = 0; i < K; i++) {
        if (a[i] == 0) continue;
        for (j = 0; j < K; j++) {
            tmp[i+j] = (tmp[i+j] + a[i] * b[j]) % MOD;
        }
    }

    /* Reduce: for degree 2K-2 down to K, replace x^d with x^{d-K+1} + x^{d-K} */
    for (i = 2*K - 2; i >= K; i--) {
        if (tmp[i] == 0) continue;
        long long c = tmp[i];
        tmp[i] = 0;
        /* x^i = x^{i-K} * x^K = x^{i-K} * (x + 1) = x^{i-K+1} + x^{i-K} */
        tmp[i - K + 1] = (tmp[i - K + 1] + c) % MOD;
        tmp[i - K]     = (tmp[i - K]     + c) % MOD;
    }

    memcpy(out, tmp, sizeof(long long) * K);
}

int main(void) {
    long long *base = (long long *)calloc(K, sizeof(long long));
    long long *result = (long long *)calloc(K, sizeof(long long));
    long long *temp = (long long *)calloc(K, sizeof(long long));

    if (!base || !result || !temp) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    /* base = x (polynomial) */
    base[1] = 1;

    /* result = 1 (polynomial) */
    result[0] = 1;

    /* Compute x^N mod (x^K - x - 1) mod MOD */
    /* N = 10^18 */
    long long N = 1000000000000000000LL;

    while (N > 0) {
        if (N & 1) {
            poly_mul_mod(result, base, temp);
            memcpy(result, temp, sizeof(long long) * K);
        }
        poly_mul_mod(base, base, temp);
        memcpy(base, temp, sizeof(long long) * K);
        N >>= 1;
    }

    /* Evaluate at x=1: sum of all coefficients */
    long long ans = 0;
    int i;
    for (i = 0; i < K; i++) {
        ans = (ans + result[i]) % MOD;
    }

    printf("%lld\n", ans);

    free(base);
    free(result);
    free(temp);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
