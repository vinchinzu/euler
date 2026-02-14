/*
 * Project Euler 148 - Entries in Pascal's triangle not divisible by 7
 *
 * Uses digit DP on base-7 representation.
 * For rows 0 to 10^9-1.
 */
#include <stdio.h>

int main(void) {
    long long N = 999999999LL; /* 10^9 - 1 */

    /* Convert to base 7 */
    int digits[20];
    int ndigits = 0;
    {
        long long tmp = N;
        while (tmp > 0) {
            digits[ndigits++] = (int)(tmp % 7);
            tmp /= 7;
        }
        /* Reverse */
        for (int i = 0; i < ndigits / 2; i++) {
            int t = digits[i];
            digits[i] = digits[ndigits - 1 - i];
            digits[ndigits - 1 - i] = t;
        }
    }

    /* Digit DP: dp(pos, tight) = sum of products of (digit+1) for all valid numbers */
    /* Iterative approach */
    /* result_tight = product considering tight constraint */
    /* result_free = accumulated from freed digits */

    long long result = 0;
    long long prefix_product = 1; /* product of (chosen_digit + 1) for tight path so far */

    for (int pos = 0; pos < ndigits; pos++) {
        int max_d = digits[pos];
        /* Sum contribution from digits 0..max_d-1 (which free the suffix) */
        /* For digit d (0 <= d < max_d), suffix is free: each position can be 0..6 */
        /* Free suffix product = 28^(remaining_positions) since sum_{d=0}^{6}(d+1) = 28 */
        long long free_suffix = 1;
        for (int j = pos + 1; j < ndigits; j++) free_suffix *= 28;

        long long sum_d = 0;
        for (int d = 0; d < max_d; d++) {
            sum_d += (d + 1);
        }
        result += prefix_product * sum_d * free_suffix;

        /* Continue tight path with digit max_d */
        prefix_product *= (max_d + 1);
    }
    /* Add the tight path itself (the number N exactly) */
    result += prefix_product;

    printf("%lld\n", result);
    return 0;
}
