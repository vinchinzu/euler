"""Project Euler Problem 767: Matrix Counting.

Find the number of 16xN matrices such that every entry is 0 or 1, and every
2xK sub-matrix has exactly K 1s.

Uses FFT with long double precision and carry handling for modular convolution.
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

#define MOD 1000000007LL
#define PI 3.14159265358979323846264338327950288L

typedef __int128 int128;
typedef long double ldouble;

typedef struct {
    ldouble re, im;
} Complex;

Complex cmul(Complex a, Complex b) {
    Complex c;
    c.re = a.re * b.re - a.im * b.im;
    c.im = a.re * b.im + a.im * b.re;
    return c;
}

Complex cadd(Complex a, Complex b) {
    Complex c;
    c.re = a.re + b.re;
    c.im = a.im + b.im;
    return c;
}

Complex csub(Complex a, Complex b) {
    Complex c;
    c.re = a.re - b.re;
    c.im = a.im - b.im;
    return c;
}

void fft(Complex* a, int n, int invert) {
    for (int i = 1, j = 0; i < n; i++) {
        int bit = n >> 1;
        for (; j & bit; bit >>= 1) j ^= bit;
        j ^= bit;
        if (i < j) {
            Complex t = a[i]; a[i] = a[j]; a[j] = t;
        }
    }

    for (int len = 2; len <= n; len <<= 1) {
        ldouble ang = 2 * PI / len * (invert ? -1 : 1);
        Complex wlen = {cosl(ang), sinl(ang)};
        for (int i = 0; i < n; i += len) {
            Complex w = {1, 0};
            for (int j = 0; j < len / 2; j++) {
                Complex u = a[i + j];
                Complex v = cmul(a[i + j + len/2], w);
                a[i + j] = cadd(u, v);
                a[i + j + len/2] = csub(u, v);
                w = cmul(w, wlen);
            }
        }
    }

    if (invert) {
        for (int i = 0; i < n; i++) {
            a[i].re /= n;
            a[i].im /= n;
        }
    }
}

int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (int128)result * base % mod;
        base = (int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

// Split each coefficient into low and high parts for precision
// a[i] = a_lo[i] + a_hi[i] * BASE
#define BASE 32768  // 2^15

void convolve_mod(int64_t* a, int64_t* b, int64_t* result, int n, int64_t mod) {
    int m = 1;
    while (m < 2 * n) m *= 2;

    Complex* fa_lo = (Complex*)calloc(m, sizeof(Complex));
    Complex* fa_hi = (Complex*)calloc(m, sizeof(Complex));
    Complex* fb_lo = (Complex*)calloc(m, sizeof(Complex));
    Complex* fb_hi = (Complex*)calloc(m, sizeof(Complex));

    for (int i = 0; i < n; i++) {
        fa_lo[i].re = a[i] % BASE;
        fa_hi[i].re = a[i] / BASE;
        fb_lo[i].re = b[i] % BASE;
        fb_hi[i].re = b[i] / BASE;
    }

    fft(fa_lo, m, 0);
    fft(fa_hi, m, 0);
    fft(fb_lo, m, 0);
    fft(fb_hi, m, 0);

    // result = (a_lo + a_hi*BASE) * (b_lo + b_hi*BASE)
    //        = a_lo*b_lo + (a_lo*b_hi + a_hi*b_lo)*BASE + a_hi*b_hi*BASE^2
    Complex* r_ll = (Complex*)calloc(m, sizeof(Complex));
    Complex* r_lh = (Complex*)calloc(m, sizeof(Complex));
    Complex* r_hh = (Complex*)calloc(m, sizeof(Complex));

    for (int i = 0; i < m; i++) {
        r_ll[i] = cmul(fa_lo[i], fb_lo[i]);
        r_lh[i] = cadd(cmul(fa_lo[i], fb_hi[i]), cmul(fa_hi[i], fb_lo[i]));
        r_hh[i] = cmul(fa_hi[i], fb_hi[i]);
    }

    fft(r_ll, m, 1);
    fft(r_lh, m, 1);
    fft(r_hh, m, 1);

    for (int i = 0; i < 2*n-1; i++) {
        int64_t ll = (int64_t)roundl(r_ll[i].re) % mod;
        int64_t lh = (int64_t)roundl(r_lh[i].re) % mod;
        int64_t hh = (int64_t)roundl(r_hh[i].re) % mod;
        result[i] = (ll + lh * BASE % mod + (int128)hh * BASE % mod * BASE % mod) % mod;
    }

    free(fa_lo); free(fa_hi);
    free(fb_lo); free(fb_hi);
    free(r_ll); free(r_lh); free(r_hh);
}

int main() {
    int64_t N = 10000000000000000LL;  // 10^16
    int K = 100000;  // 10^5
    int T = 16;

    // Precompute factorials mod MOD
    int64_t* fact = (int64_t*)malloc((K + 1) * sizeof(int64_t));
    int64_t* inv_fact = (int64_t*)malloc((K + 1) * sizeof(int64_t));
    fact[0] = 1;
    for (int i = 1; i <= K; i++)
        fact[i] = (int128)fact[i-1] * i % MOD;
    inv_fact[K] = pow_mod(fact[K], MOD - 2, MOD);
    for (int i = K - 1; i >= 0; i--)
        inv_fact[i] = (int128)inv_fact[i + 1] * (i + 1) % MOD;

    // Compute coefficients: (1/k!)^T
    int64_t* coeffs = (int64_t*)malloc((K + 1) * sizeof(int64_t));
    for (int i = 0; i <= K; i++)
        coeffs[i] = pow_mod(inv_fact[i], T, MOD);

    // Multiply polynomial by itself
    int64_t* p2 = (int64_t*)calloc(2 * K + 1, sizeof(int64_t));
    convolve_mod(coeffs, coeffs, p2, K + 1, MOD);

    // Compute f[i] = i!^T * p2[i]
    int64_t* f = (int64_t*)malloc((K + 1) * sizeof(int64_t));
    for (int i = 0; i <= K; i++)
        f[i] = (int128)pow_mod(fact[i], T, MOD) * p2[i] % MOD;

    // Compute answer
    int64_t base = pow_mod(2, N / K, MOD);
    int64_t term = (base - 2 + MOD) % MOD;

    int64_t ans = 0;
    int64_t term_pow = 1;
    for (int i = 0; i <= K; i++) {
        int64_t ncr = (int128)fact[K] * inv_fact[i] % MOD * inv_fact[K - i] % MOD;
        ans = (ans + (int128)ncr * term_pow % MOD * f[K - i]) % MOD;
        term_pow = (int128)term_pow * term % MOD;
    }

    printf("%lld\n", ans);

    free(fact);
    free(inv_fact);
    free(coeffs);
    free(p2);
    free(f);

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

def main():
    result = solve()
    print(result)
    return result

if __name__ == "__main__":
    main()
