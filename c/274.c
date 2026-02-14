/*
 * Project Euler 274: Divisibility Multipliers
 *
 * For each prime p <= 10^7 with gcd(p, 10) = 1, find the modular inverse
 * of 10 mod p. Sum all such inverses.
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

#define LIMIT 10000000

static char is_prime[LIMIT + 1];

static long long mod_inverse(long long a, long long m) {
    /* Extended Euclidean algorithm: find x such that a*x = 1 (mod m) */
    long long old_r = a, r = m;
    long long old_s = 1, s = 0;
    while (r != 0) {
        long long q = old_r / r;
        long long tmp;
        tmp = r; r = old_r - q * r; old_r = tmp;
        tmp = s; s = old_s - q * s; old_s = tmp;
    }
    /* old_s * a + ??? * m = old_r = gcd */
    return ((old_s % m) + m) % m;
}

int main(void) {
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;

    int sq = (int)sqrt((double)LIMIT);
    for (int i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= LIMIT; j += i)
                is_prime[j] = 0;
        }
    }

    long long ans = 0;
    for (int p = 2; p <= LIMIT; p++) {
        if (is_prime[p] && p != 2 && p != 5) {
            ans += mod_inverse(10, p);
        }
    }

    printf("%lld\n", ans);
    return 0;
}
