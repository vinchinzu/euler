"""Project Euler Problem 784: Reciprocal Pairs.

A pair (p, q) is a reciprocal pair if there exists an integer r < p such that
r = p^(-1) (mod q) and r = q^(-1) (mod p). Find the sum of p+q for all reciprocal
pairs (p,q) with p <= N.

Key insight: For valid (p,q), we need p + q | p^2 - 1.
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define N 2000000

int *spf;  // smallest prime factor

void compute_spf() {
    spf = (int*)calloc(N + 2, sizeof(int));
    for (int i = 2; i <= N + 1; i++) {
        if (spf[i] == 0) {
            spf[i] = i;
            if ((long long)i * i <= N + 1) {
                for (int j = i * i; j <= N + 1; j += i) {
                    if (spf[j] == 0) spf[j] = i;
                }
            }
        }
    }
}

// Generate all divisors of n using SPF
int get_divisors(int n, long long *divs) {
    if (n <= 1) {
        divs[0] = 1;
        return 1;
    }

    // Factor n
    int primes[32], exps[32], nf = 0;
    int temp = n;
    while (temp > 1) {
        int p = spf[temp];
        primes[nf] = p;
        exps[nf] = 0;
        while (temp % p == 0) {
            exps[nf]++;
            temp /= p;
        }
        nf++;
    }

    // Generate divisors recursively
    int count = 0;
    int idx[32] = {0};

    while (1) {
        long long d = 1;
        for (int i = 0; i < nf; i++) {
            for (int j = 0; j < idx[i]; j++) {
                d *= primes[i];
            }
        }
        divs[count++] = d;

        int i = 0;
        while (i < nf) {
            idx[i]++;
            if (idx[i] <= exps[i]) break;
            idx[i] = 0;
            i++;
        }
        if (i == nf) break;
    }

    return count;
}

long long gcd(long long a, long long b) {
    while (b) {
        long long t = b;
        b = a % b;
        a = t;
    }
    return a;
}

// Sort and deduplicate
int cmp_ll(const void *a, const void *b) {
    long long x = *(long long*)a;
    long long y = *(long long*)b;
    return (x > y) - (x < y);
}

int main() {
    compute_spf();

    long long ans = 0;

    long long *divs = (long long*)malloc(100000 * sizeof(long long));
    long long *small_divs = (long long*)malloc(100000 * sizeof(long long));

    for (int p = 2; p <= N; p++) {
        long long m = (long long)(p - 1) * (p + 1);  // p^2 - 1

        long long *d1 = divs;
        long long *d2 = divs + 5000;

        int nd1 = get_divisors(p - 1, d1);
        int nd2 = get_divisors(p + 1, d2);

        // Collect all small divisors d of m with d < p
        int nsmall = 0;
        for (int i = 0; i < nd1; i++) {
            for (int j = 0; j < nd2; j++) {
                long long d = d1[i] * d2[j];
                if (d < p && m % d == 0) {  // d divides m
                    small_divs[nsmall++] = d;
                }
            }
        }

        // Sort and deduplicate
        qsort(small_divs, nsmall, sizeof(long long), cmp_ll);
        int nunique = 0;
        for (int i = 0; i < nsmall; i++) {
            if (i == 0 || small_divs[i] != small_divs[i-1]) {
                small_divs[nunique++] = small_divs[i];
            }
        }

        // For each unique small divisor d, check if D = m/d > 2p
        for (int i = 0; i < nunique; i++) {
            long long d = small_divs[i];
            long long D = m / d;
            if (D > 2 * (long long)p) {
                long long q = D - p;
                if (gcd(p, q) == 1) {
                    ans += p + q;
                }
            }
        }
    }

    printf("%lld\n", ans);

    free(spf);
    free(divs);
    free(small_divs);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)


if __name__ == "__main__":
    print(solve())
