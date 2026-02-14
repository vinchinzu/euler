/*
 * Project Euler Problem 518: Prime triples and geometric sequences.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 100000000

static unsigned char sieve[(N / 8) + 1];

static inline void set_composite(int i) {
    sieve[i >> 3] |= (1 << (i & 7));
}

static inline int is_prime(int i) {
    if (i < 2) return 0;
    return !(sieve[i >> 3] & (1 << (i & 7)));
}

static void init_sieve(void) {
    int sq = (int)sqrt((double)N) + 1;
    set_composite(0);
    set_composite(1);
    for (int i = 2; i <= sq; i++) {
        if (is_prime(i)) {
            for (int j = i * i; j < N; j += i) {
                set_composite(j);
            }
        }
    }
}

static inline int gcd(int a, int b) {
    while (b) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

static inline int isqrt_int(long long n) {
    int r = (int)sqrt((double)n);
    while ((long long)r * r > n) r--;
    while ((long long)(r + 1) * (r + 1) <= n) r++;
    return r;
}

int main(void) {
    long long ans = 0;
    init_sieve();

    int k_max = N / 4;

    for (int k = 1; k <= k_max; k++) {
        int q_max = isqrt_int((long long)N / k);

        for (int q = 2; q <= q_max; q++) {
            long long c = (long long)k * q * q - 1;
            if (c < 2 || c >= N) continue;
            if (!is_prime((int)c)) continue;

            for (int p = 1; p < q; p++) {
                if (gcd(q, p) != 1) continue;
                long long a = (long long)k * p * p - 1;
                if (a < 2) continue;
                if (!is_prime((int)a)) continue;
                long long b = (long long)k * p * q - 1;
                if (b < 2 || b >= N) continue;
                if (!is_prime((int)b)) continue;
                ans += a + b + c;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
