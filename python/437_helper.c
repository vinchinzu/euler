#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define N 100000000

static void fib_pair(uint64_t n, uint64_t mod, uint64_t *f, uint64_t *g) {
    if (n == 0) {
        *f = 0;
        *g = 1;
        return;
    }
    uint64_t a, b;
    fib_pair(n >> 1, mod, &a, &b);
    uint64_t two_b = (b << 1) % mod;
    uint64_t c = (a * ((two_b + mod - a) % mod)) % mod;
    uint64_t d = (a * a + b * b) % mod;
    if (n & 1) {
        *f = d;
        *g = (c + d) % mod;
    } else {
        *f = c;
        *g = d;
    }
}

int main(void) {
    int limit = N;
    uint32_t *spf = calloc((size_t)limit + 1, sizeof(uint32_t));
    if (!spf) {
        fprintf(stderr, "Allocation failed for spf\n");
        return 1;
    }

    uint32_t *primes = malloc((size_t)(limit / 10) * sizeof(uint32_t));
    if (!primes) {
        fprintf(stderr, "Allocation failed for primes\n");
        free(spf);
        return 1;
    }
    int prime_count = 0;

    for (uint32_t i = 2; i <= (uint32_t)limit; i++) {
        if (spf[i] == 0) {
            spf[i] = i;
            primes[prime_count++] = i;
        }
        for (int j = 0; j < prime_count; j++) {
            uint64_t p = primes[j];
            uint64_t v = (uint64_t)i * p;
            if (v > (uint64_t)limit || p > spf[i]) {
                break;
            }
            spf[(uint32_t)v] = (uint32_t)p;
        }
    }

    uint64_t sum = 0;
    for (int idx = 0; idx < prime_count; idx++) {
        uint32_t p = primes[idx];
        if (p == 5) {
            sum += 5;
            continue;
        }
        uint32_t mod10 = p % 10;
        if (mod10 != 1 && mod10 != 9) {
            continue;
        }

        uint32_t n = p - 1;
        int good = 1;
        while (n > 1) {
            uint32_t q = spf[n];
            while (n % q == 0) {
                n /= q;
            }
            uint64_t f, g;
            fib_pair((uint64_t)(p - 1) / q, p, &f, &g);
            if (f == 0 && g == 1) {
                good = 0;
                break;
            }
        }
        if (good) {
            sum += p;
        }
    }

    printf("%llu\n", (unsigned long long)sum);
    free(primes);
    free(spf);
    return 0;
}
