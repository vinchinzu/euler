#include <stdio.h>

typedef unsigned long long ull;
typedef unsigned __int128 u128;

ull mod_pow(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1)
            result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ull MOD = 10000000000ULL; /* 10^10 */
    ull power = mod_pow(2, 7830457, MOD);
    ull result = ((u128)28433ULL * power + 1) % MOD;
    printf("%llu\n", result);
    return 0;
}
