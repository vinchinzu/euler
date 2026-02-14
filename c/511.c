/*
 * Project Euler Problem 511: Sequences with Divisibility Constraints.
 * Cyclic convolution mod K using O(K^2) approach with binary exponentiation.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define K 4321
#define MOD 1000000000LL

void poly_multiply_cyclic(int64_t *p1, int64_t *p2, int64_t *result) {
    memset(result, 0, K * sizeof(int64_t));
    for (int i = 0; i < K; i++) {
        for (int j = 0; j < K; j++) {
            int idx = (i + j) % K;
            result[idx] = (result[idx] + (__int128)p1[i] * p2[j]) % MOD;
        }
    }
}

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
