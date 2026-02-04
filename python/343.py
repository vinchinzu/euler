#!/usr/bin/env python3
"""Project Euler Problem 343 - Fractional Sequences

f(k) = largest_prime_factor(k+1) - 1
Sum f(k^3) for k=1..2000000.
k^3+1 = (k+1)(k^2-k+1), so lpf(k^3+1) = max(lpf(k+1), lpf(k^2-k+1)).

Uses C extension with sieve for speed.
"""
import subprocess, os, tempfile

C_CODE = r"""
#include <stdio.h>
#include <string.h>

#define N 2000000
#define SZ (N + 2)

static int lpf_kp1[SZ];
static int primes[200000];
static int nprimes = 0;

/* Remaining cofactor after dividing out small primes */
static long long cofactor[SZ];
/* Largest prime factor of k^2-k+1 found during sieve */
static int best_small[SZ];

void build_sieve(void) {
    memset(lpf_kp1, 0, sizeof(lpf_kp1));
    for (int i = 2; i < SZ; i++) {
        if (lpf_kp1[i] == 0) {
            primes[nprimes++] = i;
            for (int j = i; j < SZ; j += i)
                lpf_kp1[j] = i;
        }
    }
}

/* modpow using __int128 */
static long long mpow(long long base, long long exp, long long mod) {
    long long r = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) r = (__int128)r * base % mod;
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return r;
}

static long long modsqrt(long long n, long long p) {
    n %= p; if (n < 0) n += p;
    if (n == 0) return 0;
    if (p == 2) return n & 1;
    if (mpow(n, (p-1)/2, p) != 1) return -1;
    long long s = 0, q = p - 1;
    while (q % 2 == 0) { s++; q /= 2; }
    if (s == 1) return mpow(n, (p+1)/4, p);
    long long z = 2;
    while (mpow(z, (p-1)/2, p) != p - 1) z++;
    long long M = s;
    long long c = mpow(z, q, p);
    long long t = mpow(n, q, p);
    long long R = mpow(n, (q+1)/2, p);
    while (1) {
        if (t == 1) return R;
        long long i = 1, tmp = (__int128)t * t % p;
        while (tmp != 1) { tmp = (__int128)tmp * tmp % p; i++; }
        long long b = c;
        for (long long j = 0; j < M - i - 1; j++) b = (__int128)b * b % p;
        M = i;
        c = (__int128)b * b % p;
        t = (__int128)t * c % p;
        R = (__int128)R * b % p;
    }
}

void init_cofactors(void) {
    for (int k = 1; k <= N; k++) {
        cofactor[k] = (long long)k * k - k + 1;
        best_small[k] = 0;
    }
}

void sieve_poly(void) {
    /* For each prime p, find roots of x^2-x+1 = 0 mod p
       discriminant = -3, so need sqrt(-3) mod p */
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        long long pp = p;

        if (p == 2) {
            /* x^2-x+1 mod 2: x=0:1, x=1:1. No roots. */
            continue;
        }
        if (p == 3) {
            /* x=2 mod 3 is the only root (double) */
            for (long long k = 2; k <= N; k += 3) {
                while (cofactor[k] % 3 == 0) {
                    cofactor[k] /= 3;
                    best_small[k] = 3;
                }
            }
            continue;
        }

        long long sq = modsqrt(pp - 3, pp);
        if (sq < 0) continue;

        long long inv2 = (pp + 1) / 2;
        long long r1 = (__int128)(1 + sq) * inv2 % pp;
        long long r2 = (__int128)(1 + pp - sq) * inv2 % pp;

        if (r1 == 0) r1 = pp;
        if (r2 == 0) r2 = pp;

        for (long long k = r1; k <= N; k += pp) {
            while (cofactor[k] % pp == 0) {
                cofactor[k] /= pp;
                if (p > best_small[k]) best_small[k] = p;
            }
        }
        if (r2 != r1) {
            for (long long k = r2; k <= N; k += pp) {
                while (cofactor[k] % pp == 0) {
                    cofactor[k] /= pp;
                    if (p > best_small[k]) best_small[k] = p;
                }
            }
        }
    }
}

int main(void) {
    build_sieve();
    init_cofactors();
    sieve_poly();

    long long total = 0;
    for (int k = 1; k <= N; k++) {
        /* Largest prime factor of k^2-k+1 */
        long long lpf_q;
        if (cofactor[k] > 1) {
            /* cofactor is a prime > 2M (can't be composite since k^2-k+1 < 4e12
               and we sieved up to 2M, so at most one prime factor > 2M) */
            lpf_q = cofactor[k];
        } else {
            lpf_q = best_small[k];
        }
        if (lpf_q == 0) lpf_q = 1; /* k=1: k^2-k+1=1 */

        long long lpf1 = lpf_kp1[k + 1];
        long long lpf = lpf1 > lpf_q ? lpf1 : lpf_q;
        total += lpf - 1;
    }
    printf("%lld\n", total);
    return 0;
}
"""

def solve():
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "p343.c")
    exe = os.path.join(tmpdir, "p343")
    with open(src, "w") as f:
        f.write(C_CODE)
    r = subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], capture_output=True, text=True)
    if r.returncode != 0:
        import sys
        print(r.stderr, file=sys.stderr)
        return None
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=60)
    return result.stdout.strip()

if __name__ == "__main__":
    print(solve())
