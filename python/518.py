"""Project Euler Problem 518: Prime triples and geometric sequences.

Find sum(a+b+c) for all triples a < b < c < N where a+1, b+1, c+1
form a geometric sequence, with N = 10^8.

Algorithm: For each k, iterate q from 1..sqrt(N/k), set c = k*q^2 - 1.
If c is prime and < N, iterate p from 1..q-1 with gcd(p,q)==1,
set a = k*p^2 - 1, b = k*p*q - 1. If both prime, add a+b+c.

Uses embedded C for performance (bitset sieve, inline gcd).
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 100000000

/* Bit-packed prime sieve: 12.5 MB for 10^8 */
static unsigned char sieve[(N / 8) + 1];

static inline void set_composite(int i) {
    sieve[i >> 3] |= (1 << (i & 7));
}

static inline int is_prime(int i) {
    if (i < 2) return 0;
    return !(sieve[i >> 3] & (1 << (i & 7)));
}

static void init_sieve(void) {
    int sq = (int)sqrt((double)N) + 1;
    int i, j;
    set_composite(0);
    set_composite(1);
    for (i = 2; i <= sq; i++) {
        if (is_prime(i)) {
            for (j = i * i; j < N; j += i) {
                set_composite(j);
            }
        }
    }
}

static inline int gcd(int a, int b) {
    while (b) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

static inline int isqrt_int(long long n) {
    int r = (int)sqrt((double)n);
    while ((long long)r * r > n) r--;
    while ((long long)(r + 1) * (r + 1) <= n) r++;
    return r;
}

int main(void) {
    long long ans = 0;
    int k, q, p;
    long long a, b, c;
    int k_max;

    init_sieve();

    /* We need q >= 2 for the p loop (1..q-1) to have entries.
       c = k*q^2 - 1 < N  =>  k*q^2 <= N.
       For q=2: k*4 <= N => k <= N/4. */
    k_max = N / 4;

    for (k = 1; k <= k_max; k++) {
        /* q_max: largest q with k*q^2 <= N, i.e. q <= sqrt(N/k) */
        int q_max = isqrt_int((long long)N / k);

        for (q = 2; q <= q_max; q++) {
            c = (long long)k * q * q - 1;
            if (c < 2 || c >= N) continue;
            if (!is_prime((int)c)) continue;

            for (p = 1; p < q; p++) {
                if (gcd(q, p) != 1) continue;
                a = (long long)k * p * p - 1;
                if (a < 2) continue;
                if (!is_prime((int)a)) continue;
                b = (long long)k * p * q - 1;
                if (b < 2 || b >= N) continue;
                if (!is_prime((int)b)) continue;
                ans += a + b + c;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol518.c")
    exe = os.path.join(tmpdir, "sol518")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"],
                   check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True,
                           check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
