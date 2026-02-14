"""Project Euler Problem 263 — An engineers' dream come true.

Find the sum of the first 4 engineers' paradise numbers n where:
- n-9, n-3, n+3, n+9 are consecutive primes
- n-8, n-4, n, n+4, n+8 are practical numbers

Ported to embedded C for speed.
"""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

/* Deterministic Miller-Rabin for n < 3,215,031,751 */
static int64_t mulmod(int64_t a, int64_t b, int64_t m) {
    return (__int128)a * b % m;
}

static int64_t powmod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, mod);
        base = mulmod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

static int miller_rabin(int64_t n, int64_t a) {
    if (n % a == 0) return n == a;
    int64_t d = n - 1;
    int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }
    int64_t x = powmod(a, d, n);
    if (x == 1 || x == n - 1) return 1;
    int i;
    for (i = 0; i < r - 1; i++) {
        x = mulmod(x, x, n);
        if (x == n - 1) return 1;
    }
    return 0;
}

static int isprime(int64_t n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    if (n % 5 == 0) return n == 5;
    if (n % 7 == 0) return n == 7;
    /* Deterministic for n < 3,215,031,751 using witnesses 2, 3, 5, 7 */
    return miller_rabin(n, 2) && miller_rabin(n, 3) &&
           miller_rabin(n, 5) && miller_rabin(n, 7);
}

/* Check if n is a practical number */
static int is_practical(int64_t n) {
    if (n <= 1) return 1;
    if (n % 2 != 0) return 0;

    /* Factor n and check Sierpinski's criterion:
       For each prime factor p_i^a_i (in increasing order),
       p_i <= 1 + sigma(p_1^a_1 * ... * p_{i-1}^{a_{i-1}})
       where sigma is the sum-of-divisors function */
    int64_t sigma = 1; /* sigma of product so far */
    int64_t tmp = n;

    /* Factor of 2 */
    int64_t pw = 1;
    while (tmp % 2 == 0) { tmp /= 2; pw *= 2; }
    sigma = 2 * pw - 1; /* sigma(2^a) = 2^(a+1) - 1 */

    int64_t p;
    for (p = 3; p * p <= tmp; p += 2) {
        if (tmp % p == 0) {
            if (p > sigma + 1) return 0;
            pw = 1;
            while (tmp % p == 0) { tmp /= p; pw *= p; }
            /* sigma(p^a) = (p^(a+1) - 1) / (p - 1) */
            sigma *= (pw * p - 1) / (p - 1);
        }
    }
    if (tmp > 1) {
        if (tmp > sigma + 1) return 0;
        sigma *= (tmp + 1);
    }
    return 1;
}

int main(void) {
    int64_t L = 2000000000LL;
    int found = 0;
    int64_t total = 0;
    int target = 4;

    int64_t i;
    for (i = 1; found < target; i++) {
        int sign;
        for (sign = -1; sign <= 1 && found < target; sign += 2) {
            int64_t n = 840 * i + sign * 20;
            if (n < 20 || n > L) continue;

            /* Check n-9, n-3, n+3, n+9 are prime */
            if (!isprime(n - 9) || !isprime(n - 3) ||
                !isprime(n + 3) || !isprime(n + 9))
                continue;

            /* Check they are CONSECUTIVE primes:
               no primes in (n-9, n-3), (n-3, n+3), (n+3, n+9) */
            int consecutive = 1;
            int64_t k;

            /* Check no prime in (n-9, n-3) */
            for (k = n - 7; k < n - 3; k += 2) {
                if (isprime(k)) { consecutive = 0; break; }
            }
            if (!consecutive) continue;

            /* Check no prime in (n-3, n+3) */
            for (k = n - 1; k < n + 3; k += 2) {
                if (isprime(k)) { consecutive = 0; break; }
            }
            if (!consecutive) continue;

            /* Check no prime in (n+3, n+9) */
            for (k = n + 5; k < n + 9; k += 2) {
                if (isprime(k)) { consecutive = 0; break; }
            }
            if (!consecutive) continue;

            /* Check practical numbers */
            if (!is_practical(n - 8) || !is_practical(n - 4) ||
                !is_practical(n) || !is_practical(n + 4) ||
                !is_practical(n + 8))
                continue;

            total += n;
            found++;
        }
    }

    printf("%lld\n", total);
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
