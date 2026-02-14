/*
 * Project Euler Problem 294: Sum of digits - experience #23
 *
 * Find the number of positive integers with up to N=11^12 digits that are
 * divisible by 23 and whose digit sum is 23, modulo 10^9.
 *
 * Uses "convolution" of digit count tables with repeated doubling.
 */
#include <stdio.h>
#include <string.h>

#define K 23
#define M 1000000000
#define B 10

/* Table: f[remainder][digitsum] */
typedef long long Table[K][K + 1];

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static void combine(Table f1, Table f2, long long n, Table out) {
    long long mult = pow_mod(B, n, K);
    memset(out, 0, sizeof(Table));
    for (int r1 = 0; r1 < K; r1++) {
        for (int s1 = 0; s1 <= K; s1++) {
            if (f1[r1][s1] == 0) continue;
            for (int r2 = 0; r2 < K; r2++) {
                int r = (int)((r1 * mult + r2) % K);
                for (int s2 = 0; s1 + s2 <= K; s2++) {
                    if (f2[r2][s2] == 0) continue;
                    out[r][s1 + s2] = (out[r][s1 + s2] + f1[r1][s1] * f2[r2][s2]) % M;
                }
            }
        }
    }
}

/* Recursive helper using repeated squaring */
static Table stack_tables[65]; /* enough for log2(11^12) ~ 41 */
static long long stack_n[65];
static int stack_top;

int main(void) {
    /* N = 11^12 */
    long long N = 1;
    for (int i = 0; i < 12; i++) N *= 11;

    /* Single digit table */
    Table single;
    memset(single, 0, sizeof(single));
    for (int d = 0; d < B; d++) {
        single[d % K][d] = 1;
    }

    /* Build table for N digits using repeated doubling */
    /* Decompose N in binary and use combine */
    /* We need to track the "width" of each partial result for the 10^n multiplier */

    /* Iterative approach with stack */
    /* Build from bits of N */
    int bits[65];
    int nbits = 0;
    {
        long long tmp = N;
        while (tmp > 0) {
            bits[nbits++] = (int)(tmp & 1);
            tmp >>= 1;
        }
    }

    /* Start with table for 0 digits (identity: f[0][0] = 1) */
    Table result;
    memset(result, 0, sizeof(result));
    result[0][0] = 1;
    long long result_width = 0;

    Table power;
    memcpy(power, single, sizeof(Table));
    long long power_width = 1;

    Table temp;

    for (int i = 0; i < nbits; i++) {
        if (bits[i]) {
            combine(result, power, power_width, temp);
            memcpy(result, temp, sizeof(Table));
            result_width += power_width;
        }
        /* Square the power */
        if (i + 1 < nbits) {
            combine(power, power, power_width, temp);
            memcpy(power, temp, sizeof(Table));
            power_width *= 2;
        }
    }

    printf("%lld\n", result[0][K]);
    return 0;
}
