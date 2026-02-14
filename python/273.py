"""Project Euler Problem 273 — Sum of Squares (embedded C)."""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

/*
 * Find primes p < 150 with p = 1 mod 4.
 * Express each as p = a^2 + b^2 (a < b).
 * Recursively form all products of Gaussian integers (x+yi) choosing
 * for each prime: skip, multiply by (a+bi), multiply by (a-bi).
 * Sum min(|x|,|y|) for all products with y > 0.
 * Use __int128 since products of ~16 primes can be large.
 */

typedef __int128 i128;

static int primes[40];
static int num_primes = 0;

/* Gaussian base: (a, b) with a^2 + b^2 = p, a < b */
static int ga[40], gb[40];

static i128 ans = 0;

static i128 abs128(i128 x) { return x < 0 ? -x : x; }

static void helper(int idx, i128 x, i128 y) {
    if (idx == num_primes) {
        if (y > 0) {
            i128 ax = abs128(x), ay = abs128(y);
            ans += (ax < ay) ? ax : ay;
        }
        return;
    }
    /* Skip this prime */
    helper(idx + 1, x, y);
    /* Multiply by (ga[idx] + gb[idx]*i) */
    i128 a = ga[idx], b = gb[idx];
    i128 nx = x * a - y * b;
    i128 ny = x * b + y * a;
    helper(idx + 1, nx, ny);
    /* Multiply by (ga[idx] - gb[idx]*i) */
    nx = x * a + y * b;
    ny = -x * b + y * a;
    helper(idx + 1, nx, ny);
}

int main(void) {
    /* Find primes < 150 with p = 1 mod 4 */
    int sieve[150];
    for (int i = 0; i < 150; i++) sieve[i] = 1;
    sieve[0] = sieve[1] = 0;
    for (int i = 2; i * i < 150; i++)
        if (sieve[i])
            for (int j = i*i; j < 150; j += i)
                sieve[j] = 0;
    for (int i = 2; i < 150; i++)
        if (sieve[i] && i % 4 == 1)
            primes[num_primes++] = i;

    /* Find Gaussian decomposition for each prime */
    for (int k = 0; k < num_primes; k++) {
        int p = primes[k];
        for (int a = 1; a * a < p; a++) {
            int rem = p - a * a;
            int b = (int)sqrt((double)rem);
            if (b * b == rem) {
                ga[k] = a;
                gb[k] = b;
                break;
            }
        }
    }

    helper(0, 1, 0);

    /* Print __int128 result */
    if (ans == 0) {
        printf("0\n");
        return 0;
    }
    char buf[50];
    int pos = 0;
    i128 v = ans;
    while (v > 0) {
        buf[pos++] = '0' + (int)(v % 10);
        v /= 10;
    }
    for (int i = pos - 1; i >= 0; i--)
        putchar(buf[i]);
    putchar('\n');
    return 0;
}
'''.lstrip()

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
