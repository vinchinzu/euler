#include <stdio.h>
#include <stdbool.h>
#include <string.h>

#define MAXDIGITS 256

/* Big number stored as char string of digits */

bool is_palindrome(const char *s, int len) {
    for (int i = 0; i < len / 2; i++) {
        if (s[i] != s[len - 1 - i]) return false;
    }
    return true;
}

void reverse_str(const char *src, int len, char *dst) {
    for (int i = 0; i < len; i++) {
        dst[i] = src[len - 1 - i];
    }
    dst[len] = '\0';
}

int add_strings(const char *a, int alen, const char *b, int blen, char *result) {
    int carry = 0;
    int ai = alen - 1;
    int bi = blen - 1;
    int ri = 0;
    char tmp[MAXDIGITS];

    while (ai >= 0 || bi >= 0 || carry) {
        int sum = carry;
        if (ai >= 0) sum += a[ai--] - '0';
        if (bi >= 0) sum += b[bi--] - '0';
        carry = sum / 10;
        tmp[ri++] = sum % 10 + '0';
    }

    /* Reverse into result */
    for (int i = 0; i < ri; i++) {
        result[i] = tmp[ri - 1 - i];
    }
    result[ri] = '\0';
    return ri;
}

bool is_lychrel(int num) {
    char current[MAXDIGITS];
    sprintf(current, "%d", num);
    int clen = (int)strlen(current);

    for (int iter = 0; iter < 50; iter++) {
        char rev[MAXDIGITS];
        reverse_str(current, clen, rev);

        char next[MAXDIGITS];
        clen = add_strings(current, clen, rev, clen, next);
        memcpy(current, next, clen + 1);

        if (is_palindrome(current, clen)) {
            return false;
        }
    }
    return true;
}

int main(void) {
    int count = 0;
    for (int i = 1; i < 10000; i++) {
        if (is_lychrel(i)) count++;
    }
    printf("%d\n", count);
    return 0;
}
