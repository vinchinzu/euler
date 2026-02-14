/*
 * Project Euler 530 - GCD Sum
 *
 * Compute sum_{n=1}^{N} sum_{d|n} gcd(d, n/d) for N = 10^15.
 * Uses Mobius function sieve and floor quotient summation.
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

#define N 1000000000000000LL

int64_t isqrt(int64_t n) {
    if (n <= 0) return 0;
    int64_t x = (int64_t)sqrt((double)n);
    while (x > 0 && x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

int64_t icbrt(int64_t n) {
    if (n <= 0) return 0;
    int64_t x = (int64_t)cbrt((double)n);
    while (x > 0 && x * x * x > n) x--;
    while ((x+1) * (x+1) * (x+1) <= n) x++;
    return x;
}

int64_t sq(int64_t x) { return x * x; }

int64_t sum_floor_quotients(int64_t n) {
    if (n <= 0) return 0;
    int64_t result = 0;
    int64_t i = 1;
    while (i <= n) {
        int64_t q = n / i;
        int64_t j = n / q;
        result += q * (j - i + 1);
        i = j + 1;
    }
    return result;
}

int64_t sum_powers(int64_t n, int k) {
    if (n <= 0) return 0;
    if (k == 1) return n * (n + 1) / 2;
    return 0;
}

int *mobius;

void pre_mobius(int limit) {
    mobius = (int*)calloc(limit + 1, sizeof(int));
    char *is_prime = (char*)malloc((limit + 1) * sizeof(char));
    for (int i = 0; i <= limit; i++) {
        mobius[i] = 1;
        is_prime[i] = 1;
    }
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((j / i) % i == 0)
                    mobius[j] = 0;
                else
                    mobius[j] = -mobius[j];
            }
        }
    }
    free(is_prime);
}

int main() {
    int64_t L = icbrt(N);
    int sqrt_n = (int)isqrt(N);

    pre_mobius(sqrt_n);

    int64_t *big = (int64_t*)calloc(L + 2, sizeof(int64_t));
    int64_t *small = (int64_t*)calloc(L + 2, sizeof(int64_t));

    for (int64_t i = 1; i <= L; i++) {
        small[i] = sum_floor_quotients(i);
    }
    for (int64_t i = 1; i <= L; i++) {
        big[i] = sum_floor_quotients(N / sq(i));
    }

    int64_t ans = 0;

    for (int64_t h = 1; sq(h) <= N; h++) {
        if (mobius[h] == 0) continue;

        int64_t n = N / sq(h);
        int64_t l = icbrt(n) / 10 + 1;

        int64_t sqrt_n_over_l = isqrt(n / l);

        for (int64_t g = 1; g <= sqrt_n_over_l; g++) {
            int64_t gh = g * h;
            int64_t term;
            if (gh <= L) {
                term = big[gh];
            } else {
                int64_t idx = n / sq(g);
                if (idx <= L) {
                    term = small[idx];
                } else {
                    term = sum_floor_quotients(idx);
                }
            }
            ans += mobius[h] * term * g;
        }

        for (int64_t q = 1; q < l; q++) {
            int64_t sqrt_n_q = isqrt(n / q);
            int64_t sqrt_n_q1 = isqrt(n / (q + 1));
            int64_t small_q = (q <= L) ? small[q] : sum_floor_quotients(q);
            ans += mobius[h] * small_q * (sum_powers(sqrt_n_q, 1) - sum_powers(sqrt_n_q1, 1));
        }
    }

    free(big);
    free(small);
    free(mobius);

    printf("%lld\n", ans);
    return 0;
}
