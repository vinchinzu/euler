"""Project Euler Problem 511: Sequences with Divisibility Constraints."""

import subprocess
import tempfile
import os

def solve():
    # The key insight is that we're doing cyclic convolution mod K
    # p1 and p2 are both arrays of size K representing polynomials
    # The cyclic convolution is: result[i] = sum over j of p1[j] * p2[(i-j) mod K]
    # By duplicating, we handle wrap-around: if i < j, (i-j) mod K = i - j + K
    # p[i+K] handles i-j+K case and p[i] handles i-j case (which is invalid for i < j)
    # So p[i+K] - p[i] correctly gives cyclic convolution... but wait
    # Actually for cyclic conv: result[i] = sum_j p1[j] * p2[(i-j) mod K]
    # After multiply: p[m] = sum_{j+l=m} p1[j]*p2[l] for m in [0, 4K-2]
    # For cyclic: we want sum over j of p1[j]*p2[(i-j) mod K] for each i in [0,K-1]
    # (i-j) mod K means: if j <= i, we want p2[i-j], if j > i, we want p2[K + i - j]
    # So result[i] = sum_{j=0}^{i} p1[j]*p2[i-j] + sum_{j=i+1}^{K-1} p1[j]*p2[K+i-j]
    # The first sum contributes to p[i], second to p[K+i]
    # So result[i] = (contribution to i from first half) + (contribution to K+i from wrap)
    # Actually both p1 and p2 have copies so:
    # p[i] includes all ways to sum to i, p[K+i] includes sums to K+i
    # For p1[j]*p2[l] where j,l each in [0,2K-1]:
    # For j in [0,K-1] and l in [0,K-1]: contributes to p[j+l] for j+l in [0, 2K-2]
    # For j in [K,2K-1] and l in [0,K-1]: contributes to p[j+l] for j+l in [K, 3K-2]
    # etc...
    # The subtraction p[i+K] - p[i] removes double counting when both indices use the "original" part

    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define K 4321
#define MOD 1000000000LL

// Using simple O(K^2) convolution - K=4321 is small enough
void poly_multiply_cyclic(int64_t *p1, int64_t *p2, int64_t *result) {
    memset(result, 0, K * sizeof(int64_t));
    for (int i = 0; i < K; i++) {
        for (int j = 0; j < K; j++) {
            int idx = (i + j) % K;
            result[idx] = (result[idx] + (__int128)p1[i] * p2[j]) % MOD;
        }
    }
}

// Global divisors array
int64_t divisors[1000];
int num_divisors;

void find_divisors(int64_t n) {
    num_divisors = 0;
    for (int64_t i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            divisors[num_divisors++] = i;
            if (i * i != n) {
                divisors[num_divisors++] = n / i;
            }
        }
    }
}

int64_t imod(int64_t a, int64_t m) {
    return ((a % m) + m) % m;
}

void num_transitions(int64_t n, int64_t *result) {
    if (n == 1) {
        memset(result, 0, K * sizeof(int64_t));
        for (int i = 0; i < num_divisors; i++) {
            int idx = imod(divisors[i], K);
            result[idx] = (result[idx] + 1) % MOD;
        }
        return;
    }

    int64_t *half = (int64_t*)calloc(K, sizeof(int64_t));
    num_transitions(n / 2, half);
    poly_multiply_cyclic(half, half, result);

    if (n % 2 == 1) {
        int64_t *one = (int64_t*)calloc(K, sizeof(int64_t));
        num_transitions(1, one);
        int64_t *temp = (int64_t*)calloc(K, sizeof(int64_t));
        memcpy(temp, result, K * sizeof(int64_t));
        poly_multiply_cyclic(temp, one, result);
        free(one);
        free(temp);
    }

    free(half);
}

int main() {
    int64_t N = 1234567898765LL;

    find_divisors(N);

    int64_t *transitions = (int64_t*)calloc(K, sizeof(int64_t));
    num_transitions(N, transitions);

    int64_t ans = transitions[imod(-N, K)];

    free(transitions);

    printf("%lld\n", ans);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
