#include <stdio.h>
#include <string.h>

#define MAXDIGITS 1024

/* Add two big numbers represented as digit strings */
int big_add(const char *a, int alen, const char *b, int blen, char *result) {
    int carry = 0;
    int ai = alen - 1;
    int bi = blen - 1;
    char tmp[MAXDIGITS];
    int ri = 0;

    while (ai >= 0 || bi >= 0 || carry) {
        int sum = carry;
        if (ai >= 0) sum += a[ai--] - '0';
        if (bi >= 0) sum += b[bi--] - '0';
        carry = sum / 10;
        tmp[ri++] = sum % 10 + '0';
    }

    for (int i = 0; i < ri; i++) {
        result[i] = tmp[ri - 1 - i];
    }
    result[ri] = '\0';
    return ri;
}

int main(void) {
    int count = 0;

    /* n = numerator part above 1, d = denominator
       Recurrence: new_n = n + 2*d, new_d = n + d */
    char n[MAXDIGITS] = "1";
    int nlen = 1;
    char d[MAXDIGITS] = "1";
    int dlen = 1;

    char tmp1[MAXDIGITS], tmp2[MAXDIGITS], tmp3[MAXDIGITS];

    for (int i = 1; i <= 1000; i++) {
        /* double_d = d + d */
        int ddlen = big_add(d, dlen, d, dlen, tmp1);

        /* current_n = n + 2*d */
        int cnlen = big_add(n, nlen, tmp1, ddlen, tmp2);

        /* current_d = n + d */
        int cdlen = big_add(n, nlen, d, dlen, tmp3);

        if (cnlen > cdlen) {
            count++;
        }

        /* Update n and d */
        memcpy(n, tmp2, cnlen + 1);
        nlen = cnlen;
        memcpy(d, tmp3, cdlen + 1);
        dlen = cdlen;
    }

    printf("%d\n", count);
    return 0;
}
