"""Project Euler Problem 810: XOR-Primes."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define N 5000000
#define L (1 << 27)  // 134217728

int main() {
    // Sieve for XOR-primes
    char *sieve = calloc(L, 1);
    for (int i = 2; i < L; i++) sieve[i] = 1;

    for (int i = 2; i < L; i++) {
        if (sieve[i]) {
            // Mark composites: for j >= i, mark xor_product(i, j)
            for (long long j = i; j < L; j++) {
                // XOR product i ⊗ j
                // Optimized: m = XOR of j*(k & -k) for each set bit k in i
                long long m = 0;
                for (int k = i; k > 0; k -= k & -k)
                    m ^= j * (k & -k);
                if (m >= L) break;
                sieve[(int)m] = 0;
            }
        }
    }

    // Count XOR-primes
    int count = 0;
    int ans = 0;
    for (int i = 2; count < N; i++) {
        if (sieve[i]) {
            count++;
            ans = i;
        }
    }

    printf("%d\n", ans);
    free(sieve);
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
