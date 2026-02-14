#include <stdio.h>

int main(void) {
    /* 2^1000 has about 302 digits; use array of digits (LSB first) */
    int digits[400];
    int len = 1;
    digits[0] = 1;

    for (int e = 0; e < 1000; e++) {
        int carry = 0;
        for (int i = 0; i < len; i++) {
            int temp = digits[i] * 2 + carry;
            digits[i] = temp % 10;
            carry = temp / 10;
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
