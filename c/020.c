#include <stdio.h>

int main(void) {
    /* 100! has 158 digits; use array of digits (LSB first) */
    int digits[256];
    int len = 1;
    digits[0] = 1;

    for (int i = 2; i <= 100; i++) {
        int carry = 0;
        for (int j = 0; j < len; j++) {
            long long temp = (long long)digits[j] * i + carry;
            digits[j] = (int)(temp % 10);
            carry = (int)(temp / 10);
        }
        while (carry > 0) {
            digits[len++] = carry % 10;
            carry /= 10;
        }
    }

    int sum = 0;
    for (int i = 0; i < len; i++) {
        sum += digits[i];
    }
    printf("%d\n", sum);
    return 0;
}
