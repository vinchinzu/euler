/*
 * Project Euler 764 - Sum of Solutions to 16x^2+y^4=z^2
 *
 * Find S(N) mod 10^9, the sum of x+y+z for all solutions to 16x^2+y^4=z^2
 * with 1<=x,y,z<=N and GCD(x,y,z)=1.
 *
 * Uses Mobius function with three cases of factorization, and __int128
 * for exact arithmetic before taking mod.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef __int128 i128;

#define MOD 1000000000LL
#define NN 10000000000000000LL  /* 10^16 */

int *mobius_arr;

void pre_mobius(int limit) {
    mobius_arr = (int *)calloc(limit + 1, sizeof(int));
    char *is_prime = (char *)calloc(limit + 1, 1);
    for (int i = 0; i <= limit; i++) {
        mobius_arr[i] = 1;
        is_prime[i] = 1;
    }
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((j / i) % i == 0)
                    mobius_arr[j] = 0;
                else
                    mobius_arr[j] = -mobius_arr[j];
            }
        }
    }
    free(is_prime);
}

/* Precomputed sums of fourth powers */
static i128 *sumFP;    /* sum of i^4 for i=1..n */
static i128 *sumOFP;   /* sum of (2i-1)^4 for i=1..n */

void precompute_sums(int limit) {
    sumFP = (i128 *)calloc(limit + 1, sizeof(i128));
    sumOFP = (i128 *)calloc(limit + 1, sizeof(i128));
    for (int i = 1; i <= limit; i++) {
        i128 iv = i;
        sumFP[i] = sumFP[i - 1] + iv * iv * iv * iv;
        i128 odd = 2 * iv - 1;
        sumOFP[i] = sumOFP[i - 1] + odd * odd * odd * odd;
    }
}

int main(void) {
    double sqrt2 = sqrt(2.0);
    int L = (int)(pow(2.0 * NN, 0.25)) + 10;

    pre_mobius(L);
    precompute_sums(L);

    i128 S = 0;

    for (int g = 1; g < L; g++) {
        if (mobius_arr[g] == 0) continue;

        i128 g4 = (i128)g * g * g * g;
        i128 n = NN / g4;  /* integer division: floor(N / g^4) */
        if (n == 0) break;

        i128 deg4 = 0;
        i128 deg2 = 0;
        /* We'll track 8*deg4 and 1*deg2 to avoid fractions:
         * Actually, use rational approach: multiply everything by 8 to keep integer.
         * But the Python code uses Fraction(5,8), Fraction(3,8), etc.
         * Let's accumulate 8*deg4_frac and 1*deg2 separately, then combine.
         */
        /* Actually, let's use a common denominator of 8 for deg4 terms.
         * deg4_num = numerator of 8*deg4 (so deg4 = deg4_num / 8)
         * deg2 is integer.
         */
        i128 deg4_num8 = 0;  /* 8 * deg4 */

        /* Case 1: only when g is odd */
        if (g % 2 == 1) {
            int64_t b = 1;
            while ((i128)b * b * b * b <= 2 * n) {
                i128 b4 = (i128)b * b * b * b;
                int limit1 = b / 2;
                double val = (2.0 * (double)n - (double)b4);
                int limit2;
                if (val > 0)
                    limit2 = ((int)(pow(val, 0.25)) + 1) / 2;
                else
                    limit2 = 0;
                int num_a = limit1 < limit2 ? limit1 : limit2;

                if (num_a > 0) {
                    /* deg4 += 5/8 * num_a * b4 + 3/8 * sumOFP[num_a] */
                    /* 8*deg4 += 5 * num_a * b4 + 3 * sumOFP[num_a] */
                    deg4_num8 += 5 * (i128)num_a * b4 + 3 * sumOFP[num_a];

                    /* deg2 += num_a^2 * b */
                    deg2 += (i128)num_a * num_a * b;
                }
                b += 2;
            }
        }

        /* Case 2 */
        {
            int64_t b = 1;
            while (4 * (i128)b * b * b * b <= n) {
                i128 b4 = (i128)b * b * b * b;
                int limit1 = (int)(sqrt2 * b);
                i128 remaining = n - 4 * b4;
                int limit2 = remaining > 0 ? (int)(pow((double)remaining, 0.25)) : 0;
                int num_a = limit1 < limit2 ? limit1 : limit2;

                if (g % 2 == 1)
                    num_a = num_a / 2;

                int mult = (g % 2 == 0) ? 1 : 2;

                if (num_a > 0) {
                    /* deg4 += 5 * num_a * b4 + 3/4 * mult^4 * sumFP[num_a] */
                    /* 8*deg4 += 40 * num_a * b4 + 6 * mult^4 * sumFP[num_a] */
                    i128 m4 = (i128)mult * mult * mult * mult;
                    deg4_num8 += 40 * (i128)num_a * b4 + 6 * m4 * sumFP[num_a];

                    /* deg2 += 2 * mult * tr(num_a) * b */
                    i128 tr_num_a = (i128)num_a * (num_a + 1) / 2;
                    deg2 += 2 * mult * tr_num_a * b;
                }
                b++;
            }
        }

        /* Case 3 */
        {
            int mult = (g % 2 == 0) ? 1 : 2;
            int64_t b = mult;
            while ((i128)b * b * b * b <= n) {
                i128 b4 = (i128)b * b * b * b;
                int limit1 = (int)((double)b / sqrt2);
                double remaining = ((double)n - (double)b4) / 4.0;
                int limit2 = remaining > 0 ? (int)(pow(remaining, 0.25)) : 0;
                int num_a = limit1 < limit2 ? limit1 : limit2;

                if (num_a > 0) {
                    /* deg4 += 5/4 * num_a * b4 + 3 * sumFP[num_a] */
                    /* 8*deg4 += 10 * num_a * b4 + 24 * sumFP[num_a] */
                    deg4_num8 += 10 * (i128)num_a * b4 + 24 * sumFP[num_a];

                    /* deg2 += 2 * tr(num_a) * b */
                    i128 tr_num_a = (i128)num_a * (num_a + 1) / 2;
                    deg2 += 2 * tr_num_a * b;
                }
                b += mult;
            }
        }

        /* S += mobius[g] * (deg4 * g^4 + deg2 * g^2) */
        /* deg4 = deg4_num8 / 8, so deg4 * g^4 = deg4_num8 * g^4 / 8 */
        i128 g2 = (i128)g * g;
        S += mobius_arr[g] * (deg4_num8 * g4 / 8 + deg2 * g2);
    }

    /* Take mod */
    long long result = (long long)(S % MOD);
    if (result < 0) result += MOD;
    printf("%lld\n", result);

    free(mobius_arr);
    free(sumFP);
    free(sumOFP);
    return 0;
}
