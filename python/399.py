#!/usr/bin/env python3
"""Project Euler Problem 399 - Square-free Fibonacci

Find the Nth square-free Fibonacci number using Wall's Conjecture.
Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* Bit sieve for primes */
static unsigned char *prime_sieve;

static inline int is_prime(long long n) {
    if (n < 2) return 0;
    return (prime_sieve[n >> 3] >> (n & 7)) & 1;
}

static void sieve_primes(long long limit) {
    long long bytes = (limit >> 3) + 1;
    prime_sieve = (unsigned char *)malloc(bytes);
    if (!prime_sieve) { fprintf(stderr, "alloc fail prime_sieve\n"); exit(1); }
    memset(prime_sieve, 0xFF, bytes);
    prime_sieve[0] &= ~((1 << 0) | (1 << 1)); /* 0 and 1 are not prime */

    for (long long i = 2; i * i <= limit; i++) {
        if (is_prime(i)) {
            for (long long j = i * i; j <= limit; j += i) {
                prime_sieve[j >> 3] &= ~(1 << (j & 7));
            }
        }
    }
}

int main(void) {
    long long N = 100000000LL;        /* 10^8 */
    long long L = 2 * N;              /* 2 * 10^8 */
    long long M = 10000000000000000LL; /* 10^16 */

    /* Allocate is_square_free as byte array */
    unsigned char *sqf = (unsigned char *)malloc(L);
    if (!sqf) { fprintf(stderr, "alloc fail sqf\n"); return 1; }
    memset(sqf, 1, L);

    /* Sieve primes up to L */
    sieve_primes(L);

    /* For each prime p, find rank of apparition alpha(p),
       then mark multiples of p*alpha(p) as not square-free */
    for (long long p = 2; p < L; p++) {
        if (!is_prime(p)) continue;

        long long first_index = 1;
        long long a = 1 % p, b = 1 % p;

        while (p * first_index < L) {
            if (a == 0) {
                /* Mark all multiples of p * first_index */
                long long step = p * first_index;
                for (long long i = step; i < L; i += step) {
                    sqf[i] = 0;
                }
                break;
            }
            long long new_b = (a + b) % p;
            a = b;
            b = new_b;
            first_index++;
        }
    }

    /* Free prime sieve - no longer needed */
    free(prime_sieve);

    /* Find the N-th square-free Fibonacci index */
    long long index = -1;
    long long count = 0;
    while (count < N) {
        index++;
        if (sqf[index]) {
            count++;
        }
    }
    free(sqf);

    /* Compute last 16 digits of F(index+1) via F(0)=0, F(1)=1, F(2)=1, ...
       The Python code starts a=1, b=1 (F1, F2) and does 'index' iterations,
       ending with a = F(index+1). */
    long long fa = 1, fb = 1; /* F(1), F(2) */
    for (long long i = 0; i < index; i++) {
        long long new_fb = (fa + fb) % M;
        fa = fb;
        fb = new_fb;
    }
    /* fa = F(index+1) mod M, but Python returns 'a' which is F(index+1) */

    /* Compute scientific notation:
       F(n) ~ phi^n / sqrt(5)
       log10(F(index+1)) = (index+1) * log10(phi) - 0.5 * log10(5)
       But the Python code uses (index+1) * log10(phi) - log10(sqrt(5))
       which is the same thing. */
    long double phi = (1.0L + sqrtl(5.0L)) / 2.0L;
    long double log_value = (long double)(index + 1) * log10l(phi) - log10l(sqrtl(5.0L));
    long long exponent = (long long)log_value;
    long double mantissa = powl(10.0L, log_value - (long double)exponent);

    /* Format: last16digits,mantissa_with_1_decimalEexponent */
    printf("%lld,%.1Lfe%lld\n", fa, mantissa, exponent);

    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol399.c")
    exe = os.path.join(tmpdir, "sol399")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"],
                   check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True,
                           check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
