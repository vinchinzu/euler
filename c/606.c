/*
 * Project Euler Problem 606: Gozinta Chains
 *
 * Embedded C extracted from Python wrapper.
 * Uses Lucy's algorithm for sum of p^3 for primes p <= n.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;
typedef __int128 i128;

#define M 1000000000LL
#define L 1000000000000LL

i64 isqrt(i64 n) {
    i64 x = (i64)sqrtl((long double)n);
    while (x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

i64 sqrtL;

i64 *small_S, *large_S;

i64 get_S(i64 v) {
    if (v <= sqrtL) return small_S[v];
    return large_S[L / v];
}

void set_S(i64 v, i64 val) {
    if (v <= sqrtL) small_S[v] = val;
    else large_S[L / v] = val;
}

i64 sum_cubes_mod(i64 n) {
    n %= (2 * M);
    i64 t;
    if (n % 2 == 0) {
        t = (n / 2) % M;
        t = (i128)t * ((n + 1) % M) % M;
    } else {
        t = n % M;
        t = (i128)t * (((n + 1) / 2) % M) % M;
    }
    return (i128)t * t % M;
}

i64 pow_mod(i64 base, int exp, i64 mod) {
    i64 result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

void compute_sum_prime_cubes() {
    sqrtL = isqrt(L);

    small_S = (i64*)calloc(sqrtL + 2, sizeof(i64));
    large_S = (i64*)calloc(sqrtL + 2, sizeof(i64));

    for (i64 v = 1; v <= sqrtL; v++)
        small_S[v] = (sum_cubes_mod(v) - 1 + M) % M;
    for (i64 k = 1; k <= sqrtL; k++) {
        i64 v = L / k;
        large_S[k] = (sum_cubes_mod(v) - 1 + M) % M;
    }

    for (i64 p = 2; p <= sqrtL; p++) {
        if (get_S(p) == get_S(p - 1)) continue;

        i64 p3 = pow_mod(p, 3, M);
        i64 S_pm1 = get_S(p - 1);

        for (i64 k = 1; k <= sqrtL; k++) {
            i64 v = L / k;
            if (v < p * p) break;
            i64 vp = v / p;
            i64 old = get_S(v);
            i64 sub = (i128)p3 * ((get_S(vp) - S_pm1 + M) % M) % M;
            set_S(v, (old - sub + M) % M);
        }
        for (i64 v = sqrtL; v >= p * p; v--) {
            i64 vp = v / p;
            i64 old = get_S(v);
            i64 sub = (i128)p3 * ((get_S(vp) - S_pm1 + M) % M) % M;
            set_S(v, (old - sub + M) % M);
        }
    }
}

int main() {
    compute_sum_prime_cubes();

    char *is_prime = (char*)calloc(sqrtL + 1, sizeof(char));
    for (i64 i = 2; i <= sqrtL; i++) is_prime[i] = 1;
    for (i64 i = 2; i * i <= sqrtL; i++)
        if (is_prime[i])
            for (i64 j = i * i; j <= sqrtL; j += i)
                is_prime[j] = 0;

    i64 ans = 0;
    for (i64 p = 2; p <= sqrtL; p++) {
        if (!is_prime[p]) continue;
        i64 p3 = pow_mod(p, 3, M);
        i64 q_max = L / p;
        if (q_max > p) {
            i64 sum_qmax = get_S(q_max);
            i64 sum_p = get_S(p);
            i64 sum_q = (sum_qmax - sum_p + M) % M;
            ans = (ans + (i128)p3 * sum_q % M) % M;
        }
    }

    printf("%lld\n", ans);

    free(small_S);
    free(large_S);
    free(is_prime);

    return 0;
}
