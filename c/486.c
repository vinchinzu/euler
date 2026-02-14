/* Project Euler 486 - Palindromic substrings
 * Translated from python/486.py
 *
 * Find number of integers 5 <= n <= L such that F_5(n) is divisible by K,
 * where L = 10^18, K = 87654321.
 * Uses multiplicative order + modular arithmetic.
 */
#include <stdio.h>
#include <stdint.h>

typedef unsigned long long ull;
typedef __int128 u128;

ull gcd(ull a, ull b) {
    while (b) { ull t = b; b = a % b; a = t; }
    return a;
}

/* Multiplicative order of base modulo mod */
ull order(ull base, ull mod) {
    if (gcd(base, mod) != 1) return 0;
    ull result = 1;
    ull power = base % mod;
    while (power != 1) {
        power = (u128)power * base % mod;
        result++;
    }
    return result;
}

/* Extended GCD */
void ext_gcd(long long a, long long b, long long *g, long long *x, long long *y) {
    if (a == 0) {
        *g = b; *x = 0; *y = 1;
        return;
    }
    long long g1, x1, y1;
    ext_gcd(b % a, a, &g1, &x1, &y1);
    *g = g1;
    *x = y1 - (b / a) * x1;
    *y = x1;
}

/* Modular inverse */
ull mod_inv(ull a, ull m) {
    long long g, x, y;
    ext_gcd((long long)(a % m), (long long)m, &g, &x, &y);
    if (g != 1) return 0;
    return (ull)((x % (long long)m + (long long)m) % (long long)m);
}

/* Modular exponentiation */
ull pow_mod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    ull N = 1000000000000000000ULL;  /* 10^18 */
    ull K = 87654321ULL;
    ull L = N;

    ull ord_val = order(2, K);
    ull inv = mod_inv(100 * ord_val / 6, K);
    long long C[6] = {57, 41, 25, 9, -8, -26};

    ull ans = 0;
    for (ull k = 0; k < ord_val; k++) {
        long long term = (long long)pow_mod(2, k, K) - (long long)(100 * (k / 6)) + C[k % 6];
        term = ((term % (long long)K) + (long long)K) % (long long)K;
        ull t = (u128)(ull)term * inv % K;
        ull n_mod = (u128)ord_val * t + k;
        if (n_mod >= 5 && n_mod <= L) {
            ull count = (L - n_mod) / ((u128)ord_val * K) + 1;
            ans += count;
        }
    }

    printf("%llu\n", ans);
    return 0;
}
