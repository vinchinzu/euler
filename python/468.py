"""Project Euler Problem 468: Smooth divisors of binomial coefficients."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

typedef __int128 i128;
typedef int64_t i64;

#define N 11111111
#define M 1000000993LL

int L;  // ilog2(N)
int L2; // Smallest power of 2 >= N/2 + 1

char *is_prime;
i64 *mod_invs;
i64 *mults;
i64 *sums;

void sieve_primes() {
    is_prime = calloc(N + 1, 1);
    for (int i = 2; i <= N; i++) is_prime[i] = 1;
    for (int i = 2; i * i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N; j += i) {
                is_prime[j] = 0;
            }
        }
    }
}

void compute_mod_invs() {
    mod_invs = malloc((N + 1) * sizeof(i64));
    mod_invs[1] = 1;
    for (int i = 2; i <= N; i++) {
        mod_invs[i] = (M - (M / i) * mod_invs[M % i] % M) % M;
    }
}

void multiplyRange(int start, i64 mult) {
    int i = start + L2 / 2;
    while (i % 2 == 0) i /= 2;
    while (1) {
        mults[i] = (i128)mults[i] * mult % M;
        while (i % 2 != 0) {
            i /= 2;
            sums[i] = ((i128)mults[2 * i] * sums[2 * i] +
                       (i128)mults[2 * i + 1] * sums[2 * i + 1]) % M;
            if (i == 0) return;
        }
        i++;
    }
}

int main() {
    sieve_primes();
    compute_mod_invs();

    // Compute L = ilog2(N) = floor(log2(N))
    L = 0;
    int t = N;
    while (t > 0) { L++; t >>= 1; }

    // L2 = smallest power of 2 >= N/2 + 1
    L2 = 1;
    while (L2 < N / 2 + 1) L2 *= 2;
    L2 *= 2;  // For the tree structure

    // S[r] = product of p^e for small primes p <= L dividing C(N, r)
    i64 *S = malloc((N / 2 + 1) * sizeof(i64));
    for (int r = 0; r <= N / 2; r++) S[r] = 1;

    i64 ans = 0;

    // For small B (< L), directly update all terms
    for (int B = 1; B < L; B++) {
        if (is_prime[B]) {
            i64 prod = 1;
            for (int r = 1; r <= N / 2; r++) {
                for (int n = N + 1 - r; n % B == 0; n /= B)
                    prod = (i128)prod * B % M;
                for (int n = r; n % B == 0; n /= B)
                    prod = (i128)prod * mod_invs[B] % M;
                S[r] = (i128)S[r] * prod % M;
            }
        }
        for (int r = 0; r <= N / 2; r++) {
            ans = (ans + 2 * S[r]) % M;
        }
    }

    // Initialize segment tree
    mults = malloc(L2 * sizeof(i64));
    sums = malloc(L2 * sizeof(i64));
    for (int i = 0; i < L2; i++) mults[i] = 1;
    for (int i = 0; i < L2; i++) sums[i] = 0;
    for (int i = 0; i <= N / 2; i++) sums[L2 / 2 + i] = S[i];
    for (int i = L2 / 2 - 1; i > 0; i--) {
        sums[i] = ((i128)mults[2 * i] * sums[2 * i] +
                   (i128)mults[2 * i + 1] * sums[2 * i + 1]) % M;
    }

    // For large B (>= L), use segment tree
    for (int B = L; B <= N; B++) {
        if (is_prime[B]) {
            // Update for numerator: r where (N+1-r) is divisible by B
            // These are r = N % B + 1, N % B + 1 + B, ...
            for (int r = N % B + 1; r <= N / 2; r += B) {
                for (int n = N + 1 - r; n % B == 0; n /= B) {
                    multiplyRange(r, B);
                }
            }
            // Update for denominator: r divisible by B
            for (int r = B; r <= N / 2; r += B) {
                for (int n = r; n % B == 0; n /= B) {
                    multiplyRange(r, mod_invs[B]);
                }
            }
        }
        ans = (ans + 2 * sums[1]) % M;
    }

    printf("%lld\n", (long long)ans);

    free(S);
    free(mults);
    free(sums);
    free(is_prime);
    free(mod_invs);
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
