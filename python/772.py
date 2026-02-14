"""Project Euler Problem 772: Balanceable Partitions.

Find the smallest integer S such that all partitions of S consisting of
integers at most N can be split into two sub-partitions with the same sum.

The answer is twice the LCM of the integers from 1 to N.

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Modular exponentiation: base^exp mod mod */
static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1)
            result = (result * base) % mod;
        base = (base * base) % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    long long N = 100000000LL;  /* 10^8 */
    long long M = 1000000007LL; /* 10^9 + 7 */

    /* Sieve of Eratosthenes using char array (100 MB) */
    unsigned char *is_prime = (unsigned char *)malloc((size_t)(N + 1));
    if (!is_prime) {
        fprintf(stderr, "alloc fail\n");
        return 1;
    }
    memset(is_prime, 1, (size_t)(N + 1));
    is_prime[0] = 0;
    is_prime[1] = 0;

    /* isqrt(N) */
    long long sq = 1;
    while ((sq + 1) * (sq + 1) <= N) sq++;

    for (long long i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (long long j = i * i; j <= N; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    /* Compute 2 * LCM(1..N) mod M */
    long long ans = 2;

    for (long long p = 2; p <= N; p++) {
        if (!is_prime[p]) continue;

        /* Compute exponent: largest e such that p^e <= N */
        /* Use integer arithmetic to avoid floating-point errors */
        long long pe = p;  /* p^1 */
        int exp = 1;
        while (pe <= N / p) {  /* pe * p <= N, written to avoid overflow */
            pe *= p;
            exp++;
        }

        ans = (ans * pow_mod(p, exp, M)) % M;
    }

    printf("%lld\n", ans);
    free(is_prime);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol772.c")
    exe = os.path.join(tmpdir, "sol772")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
