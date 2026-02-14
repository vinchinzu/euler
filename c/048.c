#include <stdio.h>

typedef unsigned long long ull;

ull mulmod(ull a, ull b, ull mod) {
    /* Use __uint128_t for safe multiplication */
    return ((__uint128_t)a * b) % mod;
}

ull mod_pow(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp % 2 == 1) {
            result = mulmod(result, base, mod);
        }
        base = mulmod(base, base, mod);
        exp /= 2;
    }
    return result;
}

int main(void) {
    ull MOD = 10000000000ULL; /* 10^10 */
    ull total = 0;

    for (int n = 1; n <= 1000; n++) {
        ull term = mod_pow(n, n, MOD);
        total = (total + term) % MOD;
    }

    printf("%llu\n", total);
    return 0;
}
