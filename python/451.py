"""Project Euler Problem 451: Modular inverses.

Find sum_{n=3}^N l(n), where l(n) is the largest number smaller than n-1 whose
multiplicative inverse is itself.

l(n) is the largest square root of 1 (mod n), other than n-1.

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 20000001

static char *is_prime_sieve;
static int *primes;
static int num_primes;

static long long ans;

static void sieve(int limit) {
    is_prime_sieve = (char *)malloc(limit + 1);
    if (!is_prime_sieve) { fprintf(stderr, "alloc fail\n"); exit(1); }
    memset(is_prime_sieve, 1, limit + 1);
    is_prime_sieve[0] = is_prime_sieve[1] = 0;
    for (int i = 2; (long long)i * i <= limit; i++) {
        if (is_prime_sieve[i]) {
            for (int j = i * i; j <= limit; j += i) {
                is_prime_sieve[j] = 0;
            }
        }
    }
    /* Count primes >= 3 */
    num_primes = 0;
    for (int i = 3; i <= limit; i++) {
        if (is_prime_sieve[i]) num_primes++;
    }
    primes = (int *)malloc(num_primes * sizeof(int));
    if (!primes) { fprintf(stderr, "alloc fail\n"); exit(1); }
    int idx = 0;
    for (int i = 3; i <= limit; i++) {
        if (is_prime_sieve[i]) primes[idx++] = i;
    }
    free(is_prime_sieve);
}

static long long mod_pos(long long a, long long m) {
    long long r = a % m;
    if (r < 0) r += m;
    return r;
}

static long long mod_inv(long long a, long long m) {
    long long t = 0, new_t = 1;
    long long r = m, new_r = a;
    while (new_r != 0) {
        long long q = r / new_r;
        long long tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

/* Dynamic array for sqrts - preallocate a reasonable max */
#define MAX_SQRTS 2048

static void helper(int min_index, long long n, long long *sqrts, int nsqrts) {
    long long N = 20000000LL;

    /* Find largest sqrt < n-1 */
    long long l = 0;
    for (int i = 0; i < nsqrts; i++) {
        if (sqrts[i] < n - 1 && sqrts[i] > l) {
            l = sqrts[i];
        }
    }
    ans += l;

    /* Stack-allocated new_sqrts for this recursion level */
    long long new_sqrts[MAX_SQRTS];

    for (int index = min_index; index < num_primes; index++) {
        long long p = primes[index];
        if (n * p > N) break;

        long long pe = p;
        while (n * pe <= N) {
            long long pe_inv = mod_inv(pe, n);
            long long n_inv = (1 - pe * pe_inv) / n;
            long long npe = n * pe;

            int new_nsqrts = 0;
            for (int i = 0; i < nsqrts; i++) {
                long long sv = sqrts[i];
                long long a = sv * pe * pe_inv;
                long long b = n * n_inv;
                new_sqrts[new_nsqrts++] = mod_pos(a + b, npe);
                new_sqrts[new_nsqrts++] = mod_pos(a - b, npe);
            }

            helper(index + 1, npe, new_sqrts, new_nsqrts);
            pe *= p;
        }
    }
}

int main(void) {
    long long N = 20000000LL;

    sieve((int)N);

    ans = 0;

    long long sqrts[MAX_SQRTS];

    /* helper(0, 1, [0]) */
    sqrts[0] = 0;
    helper(0, 1, sqrts, 1);

    /* helper(0, 2, [1]) */
    sqrts[0] = 1;
    helper(0, 2, sqrts, 1);

    /* helper(0, 4, [1, 3]) */
    sqrts[0] = 1;
    sqrts[1] = 3;
    helper(0, 4, sqrts, 2);

    /* pow2 = 8, 16, ... <= N */
    long long pow2 = 8;
    while (pow2 <= N) {
        sqrts[0] = 1;
        sqrts[1] = pow2 / 2 - 1;
        sqrts[2] = pow2 / 2 + 1;
        sqrts[3] = pow2 - 1;
        helper(0, pow2, sqrts, 4);
        pow2 *= 2;
    }

    printf("%lld\n", ans);
    free(primes);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol451.c")
    exe = os.path.join(tmpdir, "sol451")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
