#!/usr/bin/env python3
"""
Project Euler Problem 379: Least common multiple count

Find g(N) = sum_{n=1}^{N} f(n) for N = 10^12, where
f(n) = number of pairs (x, y) with 1 <= x <= y and lcm(x, y) <= n.

By Mobius inversion:
g(N) = sum_d mu(d) * T(N/d^2)
where T(m) = number of ordered triples (a,b,c) with a*b*c <= m.
Then g(N) = (sum + N) / 2 to account for x <= y ordering.

Uses embedded C for performance since the T(m) computation for large m
involves O(m^{2/3}) arithmetic operations.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* Sieve Mobius function up to L using linear sieve */
static int *mobius_arr;
static char *is_prime;
static int *primes;
static int nprimes;

void sieve_mobius(int L) {
    mobius_arr = (int *)calloc(L + 1, sizeof(int));
    is_prime = (char *)malloc(L + 1);
    primes = (int *)malloc((L / 2 + 1) * sizeof(int));
    memset(is_prime, 1, L + 1);
    nprimes = 0;
    mobius_arr[1] = 1;

    for (int i = 2; i <= L; i++) {
        if (is_prime[i]) {
            primes[nprimes++] = i;
            mobius_arr[i] = -1;
        }
        for (int j = 0; j < nprimes; j++) {
            long long ip = (long long)i * primes[j];
            if (ip > L) break;
            is_prime[ip] = 0;
            if (i % primes[j] == 0) {
                mobius_arr[ip] = 0;
                break;
            }
            mobius_arr[ip] = -mobius_arr[i];
        }
    }
}

/* isqrt for long long */
long long isqrt_ll(long long n) {
    if (n <= 0) return 0;
    long long x = (long long)sqrt((double)n);
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

/* D(n) = sum_{k=1}^{n} floor(n/k) in O(sqrt(n)) time */
long long D(long long n) {
    if (n <= 0) return 0;
    long long sq = isqrt_ll(n);
    long long s = 0;
    for (long long k = 1; k <= sq; k++) {
        s += n / k;
    }
    return 2 * s - sq * sq;
}

/* icbrt for long long */
long long icbrt(long long n) {
    if (n <= 0) return 0;
    long long x = (long long)cbrt((double)n);
    /* Adjust for floating-point errors */
    while (x > 0 && x * x * x > n) x--;
    while ((x + 1) * (x + 1) * (x + 1) <= n) x++;
    return x;
}

/* T(m) = number of ordered triples (a,b,c) with a*b*c <= m */
long long T(long long m) {
    if (m <= 0) return 0;

    long long cbrt_m = icbrt(m);
    long long total = 0;

    /* Part 1: a = 1..cbrt_m, compute D(m/a) directly */
    for (long long a = 1; a <= cbrt_m; a++) {
        total += D(m / a);
    }

    /* Part 2: a > cbrt_m, group by v = m/a */
    long long a = cbrt_m + 1;
    while (a <= m) {
        long long v = m / a;
        long long a_max = m / v;
        total += D(v) * (a_max - a + 1);
        a = a_max + 1;
    }

    return total;
}

int main() {
    long long N = 1000000000000LL;
    long long L = isqrt_ll(N);

    sieve_mobius((int)L);

    long long ans = 0;
    for (long long d = 1; d <= L; d++) {
        if (mobius_arr[d] != 0) {
            ans += mobius_arr[d] * T(N / (d * d));
        }
    }
    ans += N;
    ans /= 2;

    printf("%lld\n", ans);

    free(mobius_arr);
    free(is_prime);
    free(primes);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol379.c")
    exe = os.path.join(tmpdir, "sol379")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
