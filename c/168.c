/*
 * Project Euler Problem 168: Number Rotations
 *
 * Find the last 5 digits of the sum of all positive integers with 2 to 100 digits
 * that are a divisor of their right rotation.
 *
 * We build numbers digit by digit following the Python approach.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* Big number as string multiplication by a single digit */
/* We need to multiply a decimal string by mult (1-9) and check rotation */

#define MAX_DIGITS 105

int main(void) {
    int N = 100;
    int K = 5;
    long long MOD = 100000LL;

    long long total = 0;

    for (int mult = 1; mult <= 9; mult++) {
        for (int last_digit = 1; last_digit <= 9; last_digit++) {
            /* b starts as the string representation of last_digit */
            /* We represent b and multb as big decimal strings */
            char b[MAX_DIGITS + 5];
            b[0] = '0' + last_digit;
            b[1] = '\0';
            int b_len = 1;

            for (int i = 1; i <= N; i++) {
                /* Compute multb = mult * b as a string */
                char multb[MAX_DIGITS + 5];
                int carry = 0;
                int mb_len = 0;
                for (int j = b_len - 1; j >= 0; j--) {
                    int d = (b[j] - '0') * mult + carry;
                    carry = d / 10;
                    multb[mb_len++] = '0' + (d % 10);
                }
                while (carry > 0) {
                    multb[mb_len++] = '0' + (carry % 10);
                    carry /= 10;
                }
                /* Reverse multb */
                for (int j = 0; j < mb_len / 2; j++) {
                    char tmp = multb[j];
                    multb[j] = multb[mb_len - 1 - j];
                    multb[mb_len - 1 - j] = tmp;
                }
                multb[mb_len] = '\0';

                /* Check rotation: rotated = multb[1:] + multb[0] */
                if (mb_len == b_len && b[0] != '0' && i > 1) {
                    /* Build rotated */
                    char rotated[MAX_DIGITS + 5];
                    for (int j = 1; j < mb_len; j++)
                        rotated[j - 1] = multb[j];
                    rotated[mb_len - 1] = multb[0];
                    rotated[mb_len] = '\0';

                    if (strcmp(rotated, b) == 0) {
                        /* Add b mod MOD to total */
                        long long val = 0;
                        int start = b_len > K ? b_len - K : 0;
                        for (int j = start; j < b_len; j++)
                            val = (val * 10 + (b[j] - '0')) % MOD;
                        total = (total + val) % MOD;
                    }
                }

                /* Next b: take last i chars of multb, append last_digit */
                int start_pos = mb_len - i;
                if (start_pos < 0) start_pos = 0;
                int new_len = mb_len - start_pos;
                char new_b[MAX_DIGITS + 5];
                for (int j = 0; j < new_len; j++)
                    new_b[j] = multb[start_pos + j];
                new_b[new_len] = '0' + last_digit;
                new_b[new_len + 1] = '\0';
                b_len = new_len + 1;
                memcpy(b, new_b, b_len + 1);
            }
        }
    }

    printf("%lld\n", total % MOD);
    return 0;
}
