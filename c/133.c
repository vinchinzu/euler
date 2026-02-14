/* Project Euler Problem 133: Repunit non-factors.
 *
 * Sum of primes < 100000 that never divide R(10^n) for any n.
 * A prime p never divides R(10^n) iff ord_p(10) is not of the form 2^a * 5^b.
 */
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

#define PLIMIT 100000

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1 % mod;
    base %= mod;
    while (exp > 0) {
        if (exp & 1)
            result = (__int128)result * base % mod;
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Compute multiplicative order of 10 mod p (p is prime, p > 5) */
static long long mult_order_10(int p) {
    long long phi = p - 1;

    /* Factor phi */
    long long temp = phi;
    int factors[64];
    int nfactors = 0;
    for (long long f = 2; f * f <= temp; f++) {
        if (temp % f == 0) {
            factors[nfactors++] = (int)f;
            while (temp % f == 0) temp /= f;
        }
    }
    if (temp > 1) factors[nfactors++] = (int)temp;

    long long order = phi;
    for (int i = 0; i < nfactors; i++) {
        while (order % factors[i] == 0 && pow_mod(10, order / factors[i], p) == 1) {
            order /= factors[i];
        }
    }
    return order;
}

static bool order_is_2_5_smooth(long long order) {
    if (order == 0) return false;
    while (order % 2 == 0) order /= 2;
    while (order % 5 == 0) order /= 5;
    return order == 1;
}

int main(void) {
    /* Sieve */
    bool *is_prime = calloc(PLIMIT + 1, sizeof(bool));
    for (int i = 2; i <= PLIMIT; i++) is_prime[i] = true;
    for (int i = 2; (long long)i * i <= PLIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= PLIMIT; j += i)
                is_prime[j] = false;
        }
    }

    /* 2, 3, 5 never divide R(10^n) */
    long long total = 2 + 3 + 5;

    for (int p = 7; p < PLIMIT; p++) {
        if (!is_prime[p]) continue;
        long long order = mult_order_10(p);
        if (!order_is_2_5_smooth(order)) {
            total += p;
        }
    }

    printf("%lld\n", total);
    free(is_prime);
    return 0;
}
