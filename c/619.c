/*
 * Project Euler Problem 619: Perfect subsets
 *
 * Count non-empty subsets of {A, A+1, ..., B} whose product is a perfect square.
 * The answer is 2^(B - A + 1 - rank) - 1 mod M, where rank is the number of
 * primes p <= B such that at least one of A..B is divisible by p.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

#define A_VAL 1000000
#define B_VAL 1234567
#define M_VAL 1000000007LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    /* Sieve primes up to B_VAL */
    char *is_prime = (char*)malloc(B_VAL + 1);
    memset(is_prime, 1, B_VAL + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= B_VAL; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= B_VAL; j += i)
                is_prime[j] = 0;
        }
    }

    int rank = 0;
    for (int p = 2; p <= B_VAL; p++) {
        if (is_prime[p]) {
            if ((A_VAL - 1) / p != B_VAL / p) {
                rank++;
            }
        }
    }

    ll ans = (pow_mod(2, B_VAL - A_VAL + 1 - rank, M_VAL) - 1 + M_VAL) % M_VAL;

    printf("%lld\n", ans);

    free(is_prime);
    return 0;
}
