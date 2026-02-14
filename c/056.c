#include <stdio.h>
#include <string.h>

#define MAXDIGITS 256

/* Big number: array of digits, digits[0] is most significant */
/* Multiply big number string by an integer */
int big_multiply(const char *a, int alen, int b, char *result) {
    int carry = 0;
    char tmp[MAXDIGITS];
    int ri = 0;

    for (int i = alen - 1; i >= 0; i--) {
        int product = (a[i] - '0') * b + carry;
        carry = product / 10;
        tmp[ri++] = product % 10 + '0';
    }

    while (carry > 0) {
        tmp[ri++] = carry % 10 + '0';
        carry /= 10;
    }

    /* Reverse into result */
    for (int i = 0; i < ri; i++) {
        result[i] = tmp[ri - 1 - i];
    }
    result[ri] = '\0';
    return ri;
}

int big_power(int base, int exp, char *result) {
    result[0] = '1';
    result[1] = '\0';
    int len = 1;

    char tmp[MAXDIGITS];
    for (int i = 0; i < exp; i++) {
        len = big_multiply(result, len, base, tmp);
        memcpy(result, tmp, len + 1);
    }
    return len;
}

int digit_sum(const char *num, int len) {
    int sum = 0;
    for (int i = 0; i < len; i++) {
        sum += num[i] - '0';
    }
    return sum;
}

int main(void) {
    int max_sum = 0;
    char power_str[MAXDIGITS];

    for (int a = 1; a < 100; a++) {
        for (int b = 1; b < 100; b++) {
            int len = big_power(a, b, power_str);
            int sum = digit_sum(power_str, len);
            if (sum > max_sum) {
                max_sum = sum;
            }
        }
    }

    printf("%d\n", max_sum);
    return 0;
}
