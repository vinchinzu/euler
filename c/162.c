/*
 * Project Euler Problem 162: Hexadecimal numbers
 *
 * Count hex numbers up to 16 digits containing at least one 0, 1, and A.
 * Uses inclusion-exclusion.
 */
#include <stdio.h>

typedef unsigned long long u64;
typedef __int128 u128;

static u128 power(u128 base, int exp) {
    u128 result = 1;
    for (int i = 0; i < exp; i++)
        result *= base;
    return result;
}

static u128 sum_series(int first_count, int base, int max_k) {
    if (first_count <= 0 || base < 0 || max_k <= 0) return 0;
    if (base == 1) return (u128)first_count * max_k;
    return (u128)first_count * (power(base, max_k) - 1) / (base - 1);
}

int main(void) {
    int MAX_DIGITS = 16;

    u128 total = sum_series(15, 16, MAX_DIGITS);
    u128 missing_0 = sum_series(15, 15, MAX_DIGITS);
    u128 missing_1 = sum_series(14, 15, MAX_DIGITS);
    u128 missing_a = sum_series(14, 15, MAX_DIGITS);
    u128 missing_0_1 = sum_series(14, 14, MAX_DIGITS);
    u128 missing_0_a = sum_series(14, 14, MAX_DIGITS);
    u128 missing_1_a = sum_series(13, 14, MAX_DIGITS);
    u128 missing_all = sum_series(13, 13, MAX_DIGITS);

    u128 missing_at_least_one = missing_0 + missing_1 + missing_a
        - missing_0_1 - missing_0_a - missing_1_a
        + missing_all;

    u128 result = total - missing_at_least_one;

    /* Print as uppercase hex */
    /* Convert u128 to hex string */
    char hex[40];
    int pos = 0;
    u128 val = result;
    if (val == 0) {
        hex[pos++] = '0';
    } else {
        char tmp[40];
        int tpos = 0;
        while (val > 0) {
            int digit = (int)(val % 16);
            if (digit < 10) tmp[tpos++] = '0' + digit;
            else tmp[tpos++] = 'A' + (digit - 10);
            val /= 16;
        }
        for (int i = tpos - 1; i >= 0; i--)
            hex[pos++] = tmp[i];
    }
    hex[pos] = '\0';
    printf("%s\n", hex);
    return 0;
}
