/*
 * Project Euler Problem 714: Duodigits.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <float.h>

#define NN 50000
#define B 10

typedef long long ll;

long pows_mod[20];
double pows_f[20];

double d_func(int k) {
    for (int numDigits = 1; ; numDigits++) {
        pows_f[0] = 1.0;
        pows_mod[0] = 1;
        for (int i = 1; i < numDigits; i++) {
            pows_f[i] = pows_f[i-1] * B;
            pows_mod[i] = (pows_mod[i-1] * B) % k;
        }

        int n = 1 << numDigits;

        double *nums = calloc((size_t)n * B, sizeof(double));
        long *mods = calloc((size_t)n * B, sizeof(long));

        #define NUMS(bitset, digit) nums[(bitset) * B + (digit)]
        #define MODS(bitset, digit) mods[(bitset) * B + (digit)]

        for (int bitset = 1; bitset < n; bitset++) {
            int i = __builtin_ctz(bitset);
            int prev_bitset = bitset - (bitset & -bitset);
            double num = NUMS(prev_bitset, 1) + pows_f[i];
            long mod = MODS(prev_bitset, 1) + pows_mod[i];
            for (int d = 1; d < B; d++) {
                NUMS(bitset, d) = d * num;
                MODS(bitset, d) = d * mod;
            }
        }

        double best = DBL_MAX;
        for (int bitset = 0; bitset < n / 2; bitset++) {
            for (int d1 = 0; d1 < B; d1++) {
                for (int d2 = 1; d2 < B; d2++) {
                    double num = NUMS(bitset, d1) + NUMS(n - 1 - bitset, d2);
                    long mod = MODS(bitset, d1) + MODS(n - 1 - bitset, d2);
                    if (num < best && mod % k == 0) {
                        best = num;
                    }
                }
            }
        }

        free(nums);
        free(mods);

        #undef NUMS
        #undef MODS

        if (best < DBL_MAX)
            return best;
    }
}

int main() {
    double ans = 0;
    for (int k = 1; k <= NN; k++) {
        ans += d_func(k);
    }
    printf("%.12e\n", ans);
    return 0;
}
