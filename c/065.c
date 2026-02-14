/* Project Euler 065 - Convergents of e
 * Big integer arithmetic using digit arrays (big-endian) */
#include <stdio.h>
#include <string.h>

#define MAXDIG 200

typedef struct {
    int d[MAXDIG]; /* digits, big-endian, d[0] is most significant */
    int len;
} BigNum;

static void bn_from_int(BigNum *b, int val) {
    b->len = 0;
    if (val == 0) {
        b->d[0] = 0;
        b->len = 1;
        return;
    }
    int tmp[20];
    int tlen = 0;
    while (val > 0) {
        tmp[tlen++] = val % 10;
        val /= 10;
    }
    b->len = tlen;
    for (int i = 0; i < tlen; i++) {
        b->d[i] = tmp[tlen - 1 - i];
    }
}

/* result = a * scalar + b */
static void bn_mul_add(BigNum *result, const BigNum *a, int scalar, const BigNum *b) {
    /* First multiply a by scalar */
    int prod[MAXDIG];
    int plen = a->len;
    int carry = 0;
    for (int i = plen - 1; i >= 0; i--) {
        int val = a->d[i] * scalar + carry;
        prod[i] = val % 10;
        carry = val / 10;
    }
    /* Handle carry overflow */
    int extra[20];
    int elen = 0;
    while (carry > 0) {
        extra[elen++] = carry % 10;
        carry /= 10;
    }
    /* Build full product in a temporary array */
    int full[MAXDIG];
    int flen = elen + plen;
    for (int i = 0; i < elen; i++) {
        full[i] = extra[elen - 1 - i];
    }
    for (int i = 0; i < plen; i++) {
        full[elen + i] = prod[i];
    }

    /* Now add b to full */
    int blen = b->len;
    /* Determine max length */
    int maxlen = flen > blen ? flen : blen;
    int res[MAXDIG + 1];
    carry = 0;
    int ri = 0;
    int fi = flen - 1;
    int bi = blen - 1;
    while (fi >= 0 || bi >= 0 || carry) {
        int sum = carry;
        if (fi >= 0) sum += full[fi--];
        if (bi >= 0) sum += b->d[bi--];
        res[ri++] = sum % 10;
        carry = sum / 10;
    }
    /* Reverse into result */
    result->len = ri;
    for (int i = 0; i < ri; i++) {
        result->d[i] = res[ri - 1 - i];
    }
    (void)maxlen;
}

int main(void) {
    int num_terms = 100;
    int a[100];

    /* Generate continued fraction terms for e */
    a[0] = 2;
    for (int k = 1; k < num_terms; k++) {
        if ((k + 1) % 3 == 0) {
            a[k] = 2 * (k + 1) / 3;
        } else {
            a[k] = 1;
        }
    }

    /* Compute numerator using recurrence: p_k = a_k * p_{k-1} + p_{k-2} */
    BigNum p_km2, p_km1, current_p;
    bn_from_int(&p_km2, 0); /* p_{-2} = 0 */
    bn_from_int(&p_km1, 1); /* p_{-1} = 1 */

    for (int k = 0; k < num_terms; k++) {
        bn_mul_add(&current_p, &p_km1, a[k], &p_km2);
        p_km2 = p_km1;
        p_km1 = current_p;
    }

    /* Sum digits of the numerator */
    int sum = 0;
    for (int i = 0; i < p_km1.len; i++) {
        sum += p_km1.d[i];
    }

    printf("%d\n", sum);
    return 0;
}
