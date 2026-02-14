/* Project Euler Problem 132: Large repunit factors.
 *
 * Sum the first 40 prime factors of R(10^9).
 * A prime p divides R(10^9) iff 10^(10^9) = 1 (mod 9p).
 */
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#define EXPONENT 1000000000
#define TARGET_COUNT 40
#define SIEVE_LIMIT 200000  /* Should be enough to find 40 prime factors */

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

int main(void) {
    /* Sieve of Eratosthenes */
    bool *is_prime = calloc(SIEVE_LIMIT + 1, sizeof(bool));
    for (int i = 2; i <= SIEVE_LIMIT; i++) is_prime[i] = true;
    for (int i = 2; (long long)i * i <= SIEVE_LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= SIEVE_LIMIT; j += i)
                is_prime[j] = false;
        }
    }

    long long total = 0;
    int found = 0;

    for (int p = 2; p <= SIEVE_LIMIT && found < TARGET_COUNT; p++) {
        if (!is_prime[p]) continue;
        if (p == 2 || p == 5) continue;

        long long mod = 9LL * p;
        if (pow_mod(10, EXPONENT, mod) == 1) {
            total += p;
            found++;
        }
    }

    printf("%lld\n", total);
    free(is_prime);
    return 0;
}
