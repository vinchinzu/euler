/* Project Euler 484 - Arithmetic derivative
 * Extracted from embedded C in python/484.py
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef __int128 i128;
typedef int64_t i64;

#define N 5000000000000000LL  /* 5 * 10^15 */

int *primes;
int num_primes;

void sieve_primes(int limit) {
    char *is_prime = calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_prime[i] = 1;
    for (int i = 2; i * i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }
    num_primes = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) num_primes++;
    }
    primes = malloc(num_primes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) primes[idx++] = i;
    }
    free(is_prime);
}

i128 ans = 0;

i64 ipow(i64 base, i64 exp) {
    i64 result = 1;
    while (exp > 0) {
        if (exp & 1) result *= base;
        base *= base;
        exp >>= 1;
    }
    return result;
}

void helper(int minIndex, i64 mult, i64 n) {
    ans += (i128)mult * n;

    for (int index = minIndex; index < num_primes; index++) {
        i64 p = primes[index];
        if ((i128)p * p > n) break;

        i64 new_n = n / (p * p);
        for (i64 e = 2; new_n > 0; e++, new_n /= p) {
            i64 newMult = mult * (p - 1) * ipow(p, e - 2);
            if (e % p == 0) newMult *= (p + 1);
            if (e % p != 1) {
                helper(index + 1, newMult, new_n);
            }
        }
    }
}

int main() {
    int limit = (int)sqrtl((long double)N) + 1;
    sieve_primes(limit);

    helper(0, 1, N);
    ans--;  /* Subtract k=1 term */

    /* Print i128 */
    char buf[50];
    int idx = 0;
    i128 temp = ans;
    if (temp == 0) { buf[idx++] = '0'; }
    else {
        while (temp > 0) {
            buf[idx++] = '0' + (int)(temp % 10);
            temp /= 10;
        }
    }
    for (int i = idx - 1; i >= 0; i--) putchar(buf[i]);
    putchar('\n');

    free(primes);
    return 0;
}
