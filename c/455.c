/*
 * Project Euler 455 - Powers with trailing digits
 *
 * Extracted from embedded C in python/455.py.
 * For each n from 2 to N=10^6, find largest x<=K=10^9 with n^x = x (mod K).
 */
#include <stdio.h>
#include <stdint.h>

typedef unsigned __int128 uint128;

static int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (int64_t)((uint128)result * base % mod);
        base = (int64_t)((uint128)base * base % mod);
        exp >>= 1;
    }
    return result;
}

int main(void) {
    const int N = 1000000;
    const int64_t K = 1000000000LL;
    int64_t ans = 0;

    for (int n = 2; n <= N; n++) {
        if (n % 10 == 0) continue;
        int64_t f = 2;
        for (;;) {
            int64_t nf = pow_mod(n, f, K);
            if (nf == f) break;
            f = nf;
        }
        ans += f;
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
