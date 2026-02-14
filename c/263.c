/*
 * Project Euler 263: An engineers' dream come true
 *
 * Find the sum of the first 4 engineers' paradise numbers n where:
 * - n-9, n-3, n+3, n+9 are consecutive primes (sexy prime quadruplet)
 * - n-8, n-4, n, n+4, n+8 are practical numbers
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

static int64_t mulmod(int64_t a, int64_t b, int64_t m) {
    return (__int128)a * b % m;
}

static int64_t powmod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, mod);
        base = mulmod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

static int miller_rabin(int64_t n, int64_t a) {
    if (n % a == 0) return n == a;
    int64_t d = n - 1;
    int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }
    int64_t x = powmod(a, d, n);
    if (x == 1 || x == n - 1) return 1;
    for (int i = 0; i < r - 1; i++) {
        x = mulmod(x, x, n);
        if (x == n - 1) return 1;
    }
    return 0;
}

static int isprime(int64_t n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    if (n % 5 == 0) return n == 5;
    if (n % 7 == 0) return n == 7;
    return miller_rabin(n, 2) && miller_rabin(n, 3) &&
           miller_rabin(n, 5) && miller_rabin(n, 7);
}

static int is_practical(int64_t n) {
    if (n <= 1) return 1;
    if (n % 2 != 0) return 0;

    int64_t sigma = 1;
    int64_t tmp = n;

    int64_t pw = 1;
    while (tmp % 2 == 0) { tmp /= 2; pw *= 2; }
    sigma = 2 * pw - 1;

    for (int64_t p = 3; p * p <= tmp; p += 2) {
        if (tmp % p == 0) {
            if (p > sigma + 1) return 0;
            pw = 1;
            while (tmp % p == 0) { tmp /= p; pw *= p; }
            sigma *= (pw * p - 1) / (p - 1);
        }
    }
    if (tmp > 1) {
        if (tmp > sigma + 1) return 0;
        sigma *= (tmp + 1);
    }
    return 1;
}

int main(void) {
    int found = 0;
    int64_t total = 0;
    int target = 4;

    for (int64_t i = 1; found < target; i++) {
        for (int sign = -1; sign <= 1 && found < target; sign += 2) {
            int64_t n = 840 * i + sign * 20;
            if (n < 20) continue;

            if (!isprime(n - 9) || !isprime(n - 3) ||
                !isprime(n + 3) || !isprime(n + 9))
                continue;

            int consecutive = 1;
            int64_t k;

            for (k = n - 7; k < n - 3; k += 2) {
                if (isprime(k)) { consecutive = 0; break; }
            }
            if (!consecutive) continue;

            for (k = n - 1; k < n + 3; k += 2) {
                if (isprime(k)) { consecutive = 0; break; }
            }
            if (!consecutive) continue;

            for (k = n + 5; k < n + 9; k += 2) {
                if (isprime(k)) { consecutive = 0; break; }
            }
            if (!consecutive) continue;

            if (!is_practical(n - 8) || !is_practical(n - 4) ||
                !is_practical(n) || !is_practical(n + 4) ||
                !is_practical(n + 8))
                continue;

            total += n;
            found++;
        }
    }

    printf("%lld\n", (long long)total);
    return 0;
}
