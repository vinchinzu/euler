/*
 * Project Euler 429 - Sum of squares of unitary divisors of N!
 *
 * S(N) = prod_p (1 + p^{2*c(N,p)}) mod M, where c(N,p) counts factors of p in N!.
 * Extracted from embedded C in python/429.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 100000000
#define MOD 1000000009ULL

typedef unsigned long long ull;

/* Bit sieve for odd numbers */
static unsigned char sieve[(N/2)/8 + 1];

#define IS_COMPOSITE(i) (sieve[(i)>>3] & (1 << ((i)&7)))
#define SET_COMPOSITE(i) (sieve[(i)>>3] |= (1 << ((i)&7)))

ull pow_mod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    memset(sieve, 0, sizeof(sieve));

    for (long long i = 3; (long long)i * i <= N; i += 2) {
        if (!IS_COMPOSITE(i/2)) {
            for (long long j = (long long)i * i; j <= N; j += 2*i) {
                SET_COMPOSITE(j/2);
            }
        }
    }

    ull ans = 1;

    /* Handle p=2 */
    {
        ull c = 0;
        long long power = 2;
        while (power <= N) {
            c += N / power;
            power *= 2;
        }
        ans = ans * ((1 + pow_mod(2, 2*c, MOD)) % MOD) % MOD;
    }

    /* Handle odd primes */
    for (long long p = 3; p <= N; p += 2) {
        if (!IS_COMPOSITE(p/2)) {
            ull c = 0;
            long long power = p;
            while (power <= N) {
                c += N / power;
                power *= p;
            }
            ans = ans * ((1 + pow_mod((ull)p, 2*c, MOD)) % MOD) % MOD;
        }
    }

    printf("%llu\n", ans);
    return 0;
}
