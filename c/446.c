/*
 * Project Euler 446 - Retractions B
 *
 * Extracted from embedded C in python/446.py.
 * F(N) = sum_{n=1}^N R(n^4 + 4)
 * R(n) = prod(1 + p^e) - n for factorization n = prod(p^e)
 * Key: n^4 + 4 = ((n-1)^2 + 1) * ((n+1)^2 + 1)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define N 10000000
#define MOD 1000000007LL

static char is_prime[N + 2];
static int64_t factors[N + 2];
static int64_t res[N + 3];

static int64_t tonelli_shanks(int64_t a, int64_t p) {
    a %= p;
    if (a < 0) a += p;
    if (a == 0) return 0;

    int64_t test = 1;
    int64_t base = a;
    int64_t exp = (p - 1) / 2;
    int64_t e2 = exp;
    while (e2 > 0) {
        if (e2 & 1) test = (__int128)test * base % p;
        base = (__int128)base * base % p;
        e2 >>= 1;
    }
    if (test != 1) return -1;

    int64_t q = p - 1;
    int s = 0;
    while (q % 2 == 0) { q /= 2; s++; }

    int64_t z = 2;
    while (1) {
        base = z; exp = (p - 1) / 2; test = 1;
        int64_t e3 = exp;
        while (e3 > 0) {
            if (e3 & 1) test = (__int128)test * base % p;
            base = (__int128)base * base % p;
            e3 >>= 1;
        }
        if (test == p - 1) break;
        z++;
    }

    int m = s;
    int64_t c = 1; base = z; e2 = q;
    while (e2 > 0) {
        if (e2 & 1) c = (__int128)c * base % p;
        base = (__int128)base * base % p;
        e2 >>= 1;
    }
    int64_t t = 1; base = a; e2 = q;
    while (e2 > 0) {
        if (e2 & 1) t = (__int128)t * base % p;
        base = (__int128)base * base % p;
        e2 >>= 1;
    }
    int64_t r = 1; base = a; e2 = (q + 1) / 2;
    while (e2 > 0) {
        if (e2 & 1) r = (__int128)r * base % p;
        base = (__int128)base * base % p;
        e2 >>= 1;
    }

    while (1) {
        if (t == 1) return r;
        int i = 1;
        int64_t temp = (__int128)t * t % p;
        while (temp != 1) {
            temp = (__int128)temp * temp % p;
            i++;
        }
        int64_t b = c;
        int j;
        for (j = 0; j < m - i - 1; j++)
            b = (__int128)b * b % p;
        m = i;
        c = (__int128)b * b % p;
        t = (__int128)t * c % p;
        r = (__int128)r * b % p;
    }
}

int main(void) {
    int i, k;

    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (i = 2; (int64_t)i * i <= N + 1; i++) {
        if (is_prime[i]) {
            int j;
            for (j = i * i; j <= N + 1; j += i)
                is_prime[j] = 0;
        }
    }

    for (k = 0; k <= N + 1; k++)
        factors[k] = (int64_t)k * k + 1;

    for (i = 0; i <= N + 2; i++)
        res[i] = 1;

    for (k = 1; k <= N + 1; k += 2)
        factors[k] /= 2;

    for (i = 2; i <= N; i += 2)
        res[i] = 5;

    for (int p = 5; p <= N + 1; p++) {
        if (!is_prime[p] || p % 4 != 1) continue;

        int64_t sqrt_neg1 = tonelli_shanks(p - 1, (int64_t)p);
        if (sqrt_neg1 < 0) continue;

        int64_t roots[2];
        roots[0] = sqrt_neg1 % p;
        roots[1] = (p - sqrt_neg1) % p;
        int nroots = (roots[0] == roots[1]) ? 1 : 2;

        int ri;
        for (ri = 0; ri < nroots; ri++) {
            int64_t start = roots[ri];
            for (k = (int)start; k <= N + 1; k += p) {
                int64_t pw = 1;
                while (factors[k] % p == 0) {
                    factors[k] /= p;
                    pw *= p;
                }
                int64_t term = (1 + pw) % MOD;
                if (k >= 1)
                    res[k - 1] = res[k - 1] * term % MOD;
                if (k + 1 <= N + 2)
                    res[k + 1] = res[k + 1] * term % MOD;
            }
        }
    }

    for (k = 0; k <= N + 1; k++) {
        if (factors[k] > 1) {
            int64_t term = (1 + factors[k]) % MOD;
            if (k >= 1)
                res[k - 1] = res[k - 1] * term % MOD;
            if (k + 1 <= N + 2)
                res[k + 1] = res[k + 1] * term % MOD;
        }
    }

    int64_t ans = 0;
    for (i = 1; i <= N; i++) {
        int64_t n = i;
        int64_t n2 = n % MOD;
        n2 = n2 * n2 % MOD;
        int64_t n4 = n2 * n2 % MOD;
        int64_t n4p4 = (n4 + 4) % MOD;
        ans = (ans + res[i] - n4p4 + MOD) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
