/* Project Euler Problem 134: Prime pair connection.
 *
 * For consecutive primes (p1, p2) with 5 <= p1 <= 1,000,000:
 * Find smallest S divisible by p2 that ends with digits of p1.
 * Sum all such S.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define LIMIT_P1 1000000
#define SIEVE_LIMIT (LIMIT_P1 + 200)

/* Extended GCD: returns gcd, sets *x, *y such that a*x + b*y = gcd */
static long long ext_gcd(long long a, long long b, long long *x, long long *y) {
    if (a == 0) {
        *x = 0; *y = 1;
        return b;
    }
    long long x1, y1;
    long long g = ext_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return g;
}

static long long mod_inverse(long long a, long long m) {
    long long x, y;
    ext_gcd(a, m, &x, &y);
    return ((x % m) + m) % m;
}

/* Count digits */
static int num_digits(int n) {
    int d = 0;
    while (n > 0) { d++; n /= 10; }
    return d;
}

int main(void) {
    /* Sieve */
    bool *is_prime = calloc(SIEVE_LIMIT + 1, sizeof(bool));
    for (int i = 2; i <= SIEVE_LIMIT; i++) is_prime[i] = true;
    for (int i = 2; (long long)i * i <= SIEVE_LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= SIEVE_LIMIT; j += i)
                is_prime[j] = false;
        }
    }

    /* Collect primes */
    int *primes = malloc(100000 * sizeof(int));
    int nprimes = 0;
    for (int i = 2; i <= SIEVE_LIMIT; i++) {
        if (is_prime[i]) primes[nprimes++] = i;
    }
    free(is_prime);

    long long total = 0;

    for (int i = 0; i < nprimes - 1; i++) {
        int p1 = primes[i];
        if (p1 < 5) continue;
        if (p1 > LIMIT_P1) break;

        int p2 = primes[i + 1];
        int k = num_digits(p1);

        long long M = 1;
        for (int j = 0; j < k; j++) M *= 10;

        long long M_inv = mod_inverse(M, p2);
        /* t = (-p1 mod p2) * M_inv mod p2 */
        long long neg_p1_mod = (-(long long)p1 % p2 + p2) % p2;
        long long t = (__int128)neg_p1_mod * M_inv % p2;
        long long S = p1 + M * t;

        total += S;
    }

    printf("%lld\n", total);
    free(primes);
    return 0;
}
