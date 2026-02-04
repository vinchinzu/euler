"""Project Euler Problem 552: Chinese Remainder Theorem / Garner's algorithm.

For each prime p_i, check if any partial CRT reconstruction A_n (n < i) is divisible by p_i.
Uses C extension for the O(L^2) inner loop to meet the time constraint.
"""

import subprocess, tempfile, os, ctypes, struct

def sieve(limit):
    is_prime = bytearray(b'\x01') * (limit + 1)
    is_prime[0] = is_prime[1] = 0
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = 0
    return [i for i in range(limit + 1) if is_prime[i]]

def solve():
    N = 300000
    primes = sieve(N)
    L = len(primes)

    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    int L;
    if (scanf("%d", &L) != 1) return 1;

    long long *primes = (long long *)malloc(L * sizeof(long long));
    for (int i = 0; i < L; i++) {
        scanf("%lld", &primes[i]);
    }

    long long *garner = (long long *)calloc(L, sizeof(long long));
    long long ans = 0;

    for (int i = 0; i < L; i++) {
        long long p = primes[i];
        long long prod = 1;
        long long A = 0;
        int good = 0;

        for (int j = 0; j < i; j++) {
            A = (A + prod % p * (garner[j] % p)) % p;
            prod = prod % p * (primes[j] % p) % p;
            if (A == 0 && j > 0) {  /* j>0: A_1=1, skip trivial */
                good = 1;
            }
        }

        /* Compute garner[i] */
        if (prod % p != 0) {
            long long need = ((i + 1 - A) % p + p) % p;
            /* modular inverse of prod mod p using Fermat's little theorem */
            /* p is prime, so inv = prod^(p-2) mod p */
            long long base = prod % p;
            long long exp = p - 2;
            long long inv = 1;
            while (exp > 0) {
                if (exp & 1) inv = inv * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            garner[i] = need * inv % p;
        } else {
            garner[i] = 0;
        }

        if (good) {
            ans += p;
        }
    }

    printf("%lld\n", ans);
    free(primes);
    free(garner);
    return 0;
}
"""
    # Write, compile, and run C code
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        subprocess.run(["gcc", "-O2", "-o", exe, src], check=True, capture_output=True)

        input_data = f"{L}\n" + " ".join(str(p) for p in primes) + "\n"
        result = subprocess.run([exe], input=input_data, capture_output=True, text=True, check=True)
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
