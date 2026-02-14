/*
 * Project Euler Problem 175: Fractions involving the number of different
 * ways a number can be expressed as a sum of powers of 2.
 *
 * Convert p/q = 123456789/987654321 to a continued fraction,
 * adjust parity, reverse, and output as comma-separated run lengths.
 */
#include <stdio.h>

typedef long long i64;

static i64 gcd(i64 a, i64 b) {
    while (b) { i64 t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    i64 p = 123456789LL;
    i64 q = 987654321LL;

    i64 g = gcd(p, q);
    p /= g;
    q /= g;

    /* Continued fraction of the ratio */
    i64 cf[200];
    int cf_len = 0;

    int swap = 0;
    i64 num, den;
    int desired_parity;

    if (p < q) {
        num = q; den = p;
        desired_parity = 1; /* odd length */
        swap = 1;
    } else {
        num = p; den = q;
        desired_parity = 0; /* even length */
    }

    i64 a_n = num, b_n = den;
    while (b_n != 0) {
        i64 a = a_n / b_n;
        cf[cf_len++] = a;
        i64 tmp = b_n;
        b_n = a_n - a * b_n;
        a_n = tmp;
    }

    /* Adjust parity */
    while (cf_len % 2 != desired_parity) {
        if (cf[cf_len - 1] > 1) {
            cf[cf_len - 1]--;
            cf[cf_len++] = 1;
        } else if (cf_len == 1) {
            cf[0]++;
        } else {
            cf[cf_len - 2]++;
            cf_len--;
        }
    }

    /* Reverse */
    for (int i = 0; i < cf_len / 2; i++) {
        i64 tmp = cf[i];
        cf[i] = cf[cf_len - 1 - i];
        cf[cf_len - 1 - i] = tmp;
    }

    /* Print comma-separated */
    for (int i = 0; i < cf_len; i++) {
        if (i > 0) printf(",");
        printf("%lld", cf[i]);
    }
    printf("\n");

    return 0;
}
