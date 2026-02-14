/* Project Euler 063 - Powerful digit counts */
#include <stdio.h>

int main(void) {
    int count = 0;

    for (int n = 1; n <= 21; n++) {
        for (int b = 1; b <= 9; b++) {
            /* Compute b^n using digit array (little-endian) */
            int digits[200];
            int len = 1;
            digits[0] = 1;

            for (int i = 0; i < n; i++) {
                int carry = 0;
                for (int j = 0; j < len; j++) {
                    int prod = digits[j] * b + carry;
                    digits[j] = prod % 10;
                    carry = prod / 10;
                }
                while (carry > 0) {
                    digits[len++] = carry % 10;
                    carry /= 10;
                }
            }

            if (len == n) {
                count++;
            }
        }
    }

    printf("%d\n", count);
    return 0;
}
