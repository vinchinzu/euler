"""Project Euler Problem 466: Distinct products."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef int64_t i64;

#define N 10000000000000000LL  // 10^16
#define K 64

int ff[K + 1];  // Largest prime factor

void preff() {
    for (int i = 0; i <= K; i++) ff[i] = 0;
    for (int i = 2; i <= K; i++) {
        if (ff[i] == 0) {
            for (int j = i; j <= K; j += i) {
                ff[j] = i;
            }
        }
    }
}

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

i64 numNotDivisibleBy(i64 n, int *factors, int nf) {
    // Check for factor 1 (impossible to not be divisible)
    for (int i = 0; i < nf; i++) {
        if (factors[i] == 1) return 0;
    }

    // Find two factors with common prime factor
    for (int i = 0; i < nf; i++) {
        for (int j = i + 1; j < nf; j++) {
            int p = ff[gcd(factors[i], factors[j])];
            if (p > 1) {
                // Split: numbers divisible by p, numbers not divisible by p
                int newFactors1[K], newFactors2[K];
                int nf1 = 0, nf2 = 0;
                newFactors2[nf2++] = p;
                for (int k = 0; k < nf; k++) {
                    if (factors[k] % p == 0) {
                        newFactors1[nf1++] = factors[k] / p;
                    } else {
                        newFactors1[nf1++] = factors[k];
                        newFactors2[nf2++] = factors[k];
                    }
                }
                return numNotDivisibleBy(n / p, newFactors1, nf1) +
                       numNotDivisibleBy(n, newFactors2, nf2);
            }
        }
    }

    // Inclusion-exclusion: all remaining factors are coprime
    i64 result = 0;
    for (int subset = 0; subset < (1 << nf); subset++) {
        i64 count = n;
        for (int i = 0; i < nf; i++) {
            if (subset & (1 << i)) {
                count /= -factors[i];  // Alternating sign via negative division
            }
        }
        result += count;
    }
    return result;
}

int main() {
    preff();

    i64 ans = 0;
    for (int m = 1; m <= K; m++) {
        int factors[K];
        int nf = 0;
        for (int i = m + 1; i <= K; i++) {
            factors[nf++] = i / gcd(i, m);
        }
        ans += numNotDivisibleBy(N, factors, nf);
    }

    printf("%lld\n", (long long)ans);
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
