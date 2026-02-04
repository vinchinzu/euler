"""Project Euler Problem 752: Powers of 1+sqrt(7).

(1+sqrt(7))^n = alpha(n) + beta(n)*sqrt(7).
g(x) = smallest n>0 with alpha(n)=1 (mod x) and beta(n)=0 (mod x), or 0.
Find G(10^6) = sum of g(x) for x=2..10^6.

Key observations:
- g(x)=0 if gcd(x,6)>1 (since det of matrix [[1,7],[1,1]] is -6)
- For prime p not dividing 6: g(p) = order of matrix [[1,7],[1,1]] mod p
  This order divides p^2-1 (since the matrix is in GL(2,p))
- g(p^e) = g(p) * p^(e-1)
- g(m*n) = lcm(g(m), g(n)) for gcd(m,n)=1

Implemented in C for performance.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve() -> int:
    """Solve Problem 752 using compiled C."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 1000001

static int spf[MAXN];  /* smallest prime factor */
static long long g_val[MAXN]; /* g(x) for each x */

void sieve(void) {
    for (int i = 0; i < MAXN; i++) spf[i] = i;
    for (int i = 2; (long long)i * i < MAXN; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j < MAXN; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }
}

/* 2x2 matrix multiplication mod m */
typedef struct { long long a, b, c, d; } Mat;

Mat mat_mul(Mat A, Mat B, long long m) {
    Mat C;
    C.a = (A.a * B.a + A.b * B.c) % m;
    C.b = (A.a * B.b + A.b * B.d) % m;
    C.c = (A.c * B.a + A.d * B.c) % m;
    C.d = (A.c * B.b + A.d * B.d) % m;
    return C;
}

Mat mat_pow(Mat A, long long e, long long m) {
    Mat R = {1, 0, 0, 1}; /* identity */
    A.a %= m; A.b %= m; A.c %= m; A.d %= m;
    while (e > 0) {
        if (e & 1) R = mat_mul(R, A, m);
        A = mat_mul(A, A, m);
        e >>= 1;
    }
    return R;
}

int is_identity(Mat M, long long m) {
    return M.a % m == 1 && M.b % m == 0 && M.c % m == 0 && M.d % m == 1;
}

/* Find order of matrix A mod p, knowing it divides n = p^2-1 */
/* Try all divisors of n in increasing order */
long long mat_order(long long p) {
    Mat A = {1, 7, 1, 1};
    long long n = (p - 1) * (p + 1);  /* p^2 - 1 */

    /* Factorize n */
    long long temp = n;
    int primes[64], exps[64], np = 0;
    for (long long d = 2; d * d <= temp; d++) {
        if (temp % d == 0) {
            primes[np] = d;
            exps[np] = 0;
            while (temp % d == 0) { exps[np]++; temp /= d; }
            np++;
        }
    }
    if (temp > 1) { primes[np] = temp; exps[np] = 1; np++; }

    /* The order divides n. Start with n and try dividing by each prime factor.
       If the result still gives identity, use the smaller value. */
    long long order = n;
    for (int i = 0; i < np; i++) {
        while (order % primes[i] == 0) {
            long long trial = order / primes[i];
            Mat M = mat_pow(A, trial, p);
            if (is_identity(M, p))
                order = trial;
            else
                break;
        }
    }
    return order;
}

long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

long long lcm(long long a, long long b) {
    if (a == 0 || b == 0) return 0;
    return a / gcd(a, b) * b;
}

int main(void) {
    int N = 1000000;
    sieve();

    /* Compute g for all primes first */
    memset(g_val, 0, sizeof(g_val));

    for (int p = 2; p <= N; p++) {
        if (spf[p] != p) continue; /* not prime */
        if (p == 2 || p == 3) {
            g_val[p] = 0; /* gcd(p, 6) > 1 */
        } else if (p == 7) {
            g_val[p] = 7;
        } else {
            g_val[p] = mat_order((long long)p);
        }
    }

    /* Compute g for prime powers p^e: g(p^e) = g(p) * p^(e-1) */
    /* And for composite x: g(x) = lcm of g over prime power factors */

    /* Process all x from 2 to N */
    long long ans = 0;
    for (int x = 2; x <= N; x++) {
        if (spf[x] == x) {
            /* Prime, already computed */
            ans += g_val[x];
            continue;
        }

        /* Factor x and compute g(x) = lcm of g(p^e) over prime powers */
        int temp = x;
        long long gx = 1;
        int is_zero = 0;
        while (temp > 1) {
            int p = spf[temp];
            int e = 0;
            int pe = 1;
            while (temp % p == 0) { temp /= p; e++; pe *= p; }
            /* g(p^e) = g(p) * p^(e-1) */
            long long gpe = g_val[p];
            if (gpe == 0) { is_zero = 1; break; }
            for (int i = 1; i < e; i++) gpe *= p;
            gx = lcm(gx, gpe);
        }
        if (is_zero) gx = 0;
        g_val[x] = gx;
        ans += gx;
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                check=True, timeout=28)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
