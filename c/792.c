/*
 * Project Euler 792 - Too Many Twos
 *
 * u(n) = n + 2 + v2(T(n)) where T(n) = sum_{k=0}^n (-2)^k * C(2n+1, n+k+1).
 * U(N) = sum_{m=1}^N u(m^3).
 *
 * Precompute table of prefix products of odd numbers mod 2^33.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef unsigned long long ull;

#define TABLE_SIZE (1 << 27)
#define B_EFF 33
#define MASK_EFF ((1ULL << B_EFF) - 1)
#define TABLE_MASK (TABLE_SIZE - 1)

static ull *table;

static inline ull mod_inv_ull(ull a) {
    ull x = 1;
    for (int i = 0; i < 40; i++)
        x = (x * (2 - a * x)) & MASK_EFF;
    return x;
}

static void precompute_table(void) {
    table = (ull *)malloc((size_t)TABLE_SIZE * sizeof(ull));
    if (!table) { fprintf(stderr, "malloc failed\n"); exit(1); }
    table[0] = 1;
    for (int n = 1; n < TABLE_SIZE; n++) {
        if (n & 1)
            table[n] = (table[n-1] * (ull)n) & MASK_EFF;
        else
            table[n] = table[n-1];
    }
}

static inline ll v2_factorial(ll n) {
    return n - __builtin_popcountll((ull)n);
}

static ull oddpart_factorial(ll n) {
    ull result = 1;
    while (n > 0) {
        result = (result * table[n & TABLE_MASK]) & MASK_EFF;
        n >>= 1;
    }
    return result;
}

static ull nCr_mod(ll a, ll b) {
    if (b < 0 || b > a) return 0;
    ull num = oddpart_factorial(a);
    ull den = (oddpart_factorial(b) * oddpart_factorial(a - b)) & MASK_EFF;
    ull inv_den = mod_inv_ull(den);
    ull odd_part = (num * inv_den) & MASK_EFF;
    ll exp2 = v2_factorial(a) - v2_factorial(b) - v2_factorial(a - b);
    if (exp2 >= B_EFF || exp2 < 0) return 0;
    return (odd_part << (int)exp2) & MASK_EFF;
}

int main(void) {
    int N = 10000;
    precompute_table();

    ll ans = 0;

    for (int m = 1; m <= N; m++) {
        ll n = (ll)m * m * m;
        ull res = 0;
        int found = 0;
        for (int k = 0; k < B_EFF; k++) {
            ull term = nCr_mod(2*n + 1, n + k + 1);
            if (k % 2 == 0) res = (res + term) & MASK_EFF;
            else res = (res - term + (1ULL << B_EFF)) & MASK_EFF;
            if (res & 1) {
                ans += n + 2 + k;
                found = 1;
                break;
            }
            res >>= 1;
        }
        if (!found) {
            ans += n + 2 + B_EFF;
        }
    }

    printf("%lld\n", ans);
    free(table);
    return 0;
}
