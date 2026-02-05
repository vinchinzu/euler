"""Project Euler Problem 537: Counting tuples."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef int64_t i64;
typedef __int128 i128;

#define N_MAX 20001
#define MOD 1004535809LL
#define G 3  // primitive root for MOD

// MOD = 1004535809 = 479 * 2^21 + 1, so we can do NTT up to size 2^21

i64 power(i64 base, i64 exp, i64 m) {
    i64 result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % m;
        base = (i128)base * base % m;
        exp >>= 1;
    }
    return result;
}

i64 inv(i64 x) {
    return power(x, MOD - 2, MOD);
}

// NTT size (must be power of 2 and >= 2 * N_MAX)
int ntt_size;
i64 *omega, *omega_inv;

void init_ntt(int n) {
    ntt_size = 1;
    while (ntt_size < n) ntt_size <<= 1;

    omega = (i64*)malloc(ntt_size * sizeof(i64));
    omega_inv = (i64*)malloc(ntt_size * sizeof(i64));

    i64 w = power(G, (MOD - 1) / ntt_size, MOD);
    i64 w_inv = inv(w);

    omega[0] = omega_inv[0] = 1;
    for (int i = 1; i < ntt_size; i++) {
        omega[i] = (i128)omega[i-1] * w % MOD;
        omega_inv[i] = (i128)omega_inv[i-1] * w_inv % MOD;
    }
}

void ntt(i64 *a, int n, int invert) {
    for (int i = 1, j = 0; i < n; i++) {
        int bit = n >> 1;
        for (; j & bit; bit >>= 1) j ^= bit;
        j ^= bit;
        if (i < j) { i64 t = a[i]; a[i] = a[j]; a[j] = t; }
    }

    i64 *w = invert ? omega_inv : omega;

    for (int len = 2; len <= n; len <<= 1) {
        int step = ntt_size / len;
        for (int i = 0; i < n; i += len) {
            for (int j = 0, k = 0; j < len / 2; j++, k += step) {
                i64 u = a[i + j];
                i64 v = (i128)a[i + j + len/2] * w[k] % MOD;
                a[i + j] = (u + v) % MOD;
                a[i + j + len/2] = (u - v + MOD) % MOD;
            }
        }
    }

    if (invert) {
        i64 n_inv = inv(n);
        for (int i = 0; i < n; i++)
            a[i] = (i128)a[i] * n_inv % MOD;
    }
}

// Multiply two polynomials mod x^(N_MAX)
void poly_mul(i64 *a, i64 *b, i64 *result, int n) {
    int m = 1;
    while (m < 2 * n) m <<= 1;

    i64 *fa = (i64*)calloc(m, sizeof(i64));
    i64 *fb = (i64*)calloc(m, sizeof(i64));

    for (int i = 0; i < n; i++) fa[i] = a[i];
    for (int i = 0; i < n; i++) fb[i] = b[i];

    ntt(fa, m, 0);
    ntt(fb, m, 0);

    for (int i = 0; i < m; i++)
        fa[i] = (i128)fa[i] * fb[i] % MOD;

    ntt(fa, m, 1);

    for (int i = 0; i < n; i++) result[i] = fa[i];

    free(fa);
    free(fb);
}

// Polynomial exponentiation: a^k mod x^n
void poly_pow(i64 *a, int k, i64 *result, int n) {
    i64 *base = (i64*)malloc(n * sizeof(i64));
    i64 *temp = (i64*)malloc(n * sizeof(i64));

    memcpy(base, a, n * sizeof(i64));
    memset(result, 0, n * sizeof(i64));
    result[0] = 1;

    while (k > 0) {
        if (k & 1) {
            poly_mul(result, base, temp, n);
            memcpy(result, temp, n * sizeof(i64));
        }
        poly_mul(base, base, temp, n);
        memcpy(base, temp, n * sizeof(i64));
        k >>= 1;
    }

    free(base);
    free(temp);
}

// Sieve primes
int *primes;
int prime_count;

void sieve(int limit) {
    char *is_prime = (char*)calloc(limit + 1, sizeof(char));
    for (int i = 2; i <= limit; i++) is_prime[i] = 1;

    for (int i = 2; i * i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= limit; j += i)
                is_prime[j] = 0;
        }
    }

    primes = (int*)malloc((limit + 1) * sizeof(int));
    prime_count = 0;
    primes[prime_count++] = 0;  // primes[0] = 0 for convenience
    for (int i = 2; i <= limit; i++)
        if (is_prime[i]) primes[prime_count++] = i;

    free(is_prime);
}

int main() {
    int N = 20000;
    int K = 20000;

    // Need primes up to p_{N+1}
    // p_{20001} is about 224737
    sieve(250000);

    // Build f: f[i] = p_{i+1} - p_i for i >= 1, f[0] = 1
    i64 *f = (i64*)calloc(N + 1, sizeof(i64));
    f[0] = 1;
    for (int i = 1; i <= N; i++)
        f[i] = primes[i + 1] - primes[i];

    // Initialize NTT
    init_ntt(2 * (N + 1));

    // Compute f^K mod x^(N+1)
    i64 *result = (i64*)calloc(N + 1, sizeof(i64));
    poly_pow(f, K, result, N + 1);

    printf("%lld\n", result[N]);

    free(f);
    free(result);
    free(primes);
    free(omega);
    free(omega_inv);

    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)

if __name__ == "__main__":
    print(solve())
