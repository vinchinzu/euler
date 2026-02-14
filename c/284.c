/*
 * Project Euler 284 - Steady Squares
 *
 * Find the sum of the digits of all n-digit steady squares in base 14 for
 * 1 <= n <= 10000. A steady square k satisfies k^2 ≡ k (mod 14^n).
 *
 * By CRT, solutions mod 14^n = 2^n * 7^n are: 0, 1, a_n, b_n where
 * a_n ≡ 0 (mod 2^n), ≡ 1 (mod 7^n) and b_n ≡ 1 (mod 2^n), ≡ 0 (mod 7^n).
 *
 * We compute the base-14 digits of a and b incrementally using Hensel lifting,
 * then compute the digit-sum contribution.
 *
 * Uses GMP for big integer arithmetic.
 *
 * Since we need arbitrary precision, we implement our own big integer operations
 * or use a simpler approach: compute digits of a and b one at a time using
 * Hensel's lemma.
 *
 * Actually, we use a different approach: compute digits of a_n mod 14^n
 * incrementally. a_n satisfies a_n^2 ≡ a_n (mod 14^n). Given a_{n-1}, we
 * find a_n = a_{n-1} + d * 14^{n-1} where d is chosen so that a_n^2 ≡ a_n
 * (mod 14^n). This gives d*(2*a_{n-1} - 1) ≡ 0 (mod 14), with the constraint
 * that a_n ≡ 0 (mod 2^n) and ≡ 1 (mod 7^n).
 *
 * Simpler: lift mod 2^n and mod 7^n separately, then CRT each digit.
 *
 * For x^2 ≡ x (mod p^n), solutions are 0 and 1. Lifting:
 * - mod 2^n: solutions are 0 and 1 (the only idempotents)
 * - mod 7^n: solutions are 0 and 1
 * So a_n: x ≡ 0 (mod 2^n), x ≡ 1 (mod 7^n) => x = 7^n * inv(7^n, 2^n) * (something)
 *
 * We compute the base-14 digits directly by maintaining the number mod 14^k
 * for increasing k. We use arrays of base-14 digits.
 *
 * Approach: compute a mod 14^k for k=1,2,...,N by Hensel lifting.
 * a_1 ≡ 8 (mod 14) [since 8^2 = 64 ≡ 8 (mod 14), and 8 ≡ 0 mod 2, 8 ≡ 1 mod 7]
 * b_1 ≡ 7 (mod 14) [since 7^2 = 49 ≡ 7 (mod 14), and 7 ≡ 1 mod 2, 7 ≡ 0 mod 7]
 *
 * To lift: if a_k^2 ≡ a_k (mod 14^k), then a_{k+1} = a_k + d * 14^k where
 * d = a_k * (a_k - 1) / 14^k * inv(1 - 2*a_k, 14) -- but this requires
 * knowing the full number.
 *
 * Better: use the formula a_{k+1} = (3*a_k^2 - 2*a_k^3) mod 14^{k+1}.
 * This is Newton's method for x^2 - x = 0.
 *
 * We'll use big integer arithmetic with arrays of base-14 digits.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 10001

/* Big number in base 14, stored as array of digits (least significant first) */
/* We need numbers up to 14^10000 */

typedef struct {
    int digits[MAXN]; /* base-14 digits, d[0] is least significant */
    int len;          /* number of digits */
} BigNum;

/* Set bn = 0 */
static void bn_zero(BigNum *a) {
    memset(a->digits, 0, sizeof(a->digits));
    a->len = 1;
}

/* Set bn from small int */
static void bn_set(BigNum *a, int v) {
    bn_zero(a);
    a->len = 0;
    if (v == 0) { a->len = 1; return; }
    while (v > 0) {
        a->digits[a->len++] = v % 14;
        v /= 14;
    }
}

/* a = b (copy) */
static void bn_copy(BigNum *a, const BigNum *b) {
    memcpy(a, b, sizeof(BigNum));
}

/* a += b, result in a, both have at most maxlen digits */
static void bn_add(BigNum *a, const BigNum *b, int maxlen) {
    int carry = 0;
    int len = a->len > b->len ? a->len : b->len;
    if (len > maxlen) len = maxlen;
    for (int i = 0; i < len; i++) {
        int s = a->digits[i] + b->digits[i] + carry;
        a->digits[i] = s % 14;
        carry = s / 14;
    }
    if (carry && len < maxlen) {
        a->digits[len] = carry;
        len++;
    }
    a->len = len;
}

/* a = b * c, truncated to maxlen digits */
static void bn_mul(BigNum *a, const BigNum *b, const BigNum *c, int maxlen) {
    /* Use temporary to allow a == b or a == c */
    static int tmp[MAXN];
    memset(tmp, 0, maxlen * sizeof(int));

    for (int i = 0; i < b->len && i < maxlen; i++) {
        if (b->digits[i] == 0) continue;
        int carry = 0;
        for (int j = 0; j < c->len && i + j < maxlen; j++) {
            int s = tmp[i + j] + b->digits[i] * c->digits[j] + carry;
            tmp[i + j] = s % 14;
            carry = s / 14;
        }
        int k = i + c->len;
        while (carry && k < maxlen) {
            int s = tmp[k] + carry;
            tmp[k] = s % 14;
            carry = s / 14;
            k++;
        }
    }
    memset(a->digits, 0, sizeof(a->digits));
    memcpy(a->digits, tmp, maxlen * sizeof(int));
    a->len = maxlen;
    while (a->len > 1 && a->digits[a->len - 1] == 0) a->len--;
}

/* a = (3*b^2 - 2*b^3) mod 14^maxlen
 * = b^2 * (3 - 2*b) mod 14^maxlen
 */
static void bn_hensel_step(BigNum *a, const BigNum *b, int maxlen) {
    static BigNum b2, factor;

    /* b2 = b * b mod 14^maxlen */
    bn_mul(&b2, b, b, maxlen);

    /* factor = (3 - 2*b) mod 14^maxlen */
    /* Compute as: factor = 3, then subtract 2*b with borrow chain */
    bn_zero(&factor);
    factor.digits[0] = 3;
    factor.len = 1;

    int borrow = 0;
    for (int i = 0; i < maxlen; i++) {
        int val = factor.digits[i] - 2 * b->digits[i] - borrow;
        if (val < 0) {
            /* Need to borrow: add enough multiples of 14 to make val non-negative */
            int neg = -val;
            int borrows_needed = (neg + 13) / 14;
            val += borrows_needed * 14;
            borrow = borrows_needed;
        } else {
            borrow = 0;
        }
        factor.digits[i] = val % 14;
    }
    /* Any remaining borrow is absorbed by the mod 14^maxlen */
    factor.len = maxlen;
    while (factor.len > 1 && factor.digits[factor.len - 1] == 0) factor.len--;

    /* a = b2 * factor mod 14^maxlen */
    bn_mul(a, &b2, &factor, maxlen);
}

int main(void) {
    int N = 10000;

    /* Compute a and b using Hensel lifting with Newton's method:
     * x_{k+1} = 3*x_k^2 - 2*x_k^3 (mod 14^{2k})
     * Starting from a_1 = 8 (mod 14), b_1 = 7 (mod 14)
     */

    static BigNum a, b, tmp;

    /* Compute a: start with 8 */
    bn_set(&a, 8);
    /* Doubling: precision goes 1 -> 2 -> 4 -> 8 -> ... -> >= N */
    int prec = 1;
    while (prec < N) {
        int new_prec = prec * 2;
        if (new_prec > N) new_prec = N;
        bn_hensel_step(&tmp, &a, new_prec);
        bn_copy(&a, &tmp);
        prec = new_prec;
    }

    /* Compute b: start with 7 */
    bn_set(&b, 7);
    prec = 1;
    while (prec < N) {
        int new_prec = prec * 2;
        if (new_prec > N) new_prec = N;
        bn_hensel_step(&tmp, &b, new_prec);
        bn_copy(&b, &tmp);
        prec = new_prec;
    }

    /* Now compute digit sum contributions.
     * For each valid n-digit suffix (where d[n-1] != 0), add its digit sum.
     * Rearranging: d[i] contributes d[i] * (count of non-zero d[j] for j >= i).
     */
    long long result = 1; /* For the trivial steady square: 1 */

    /* Process a */
    {
        int *d = a.digits;
        /* nonzero_suffix[i] = count of j >= i with d[j] != 0, for j < N */
        long long suffix = 0;
        long long total = 0;
        for (int i = N - 1; i >= 0; i--) {
            if (d[i] != 0) suffix++;
            total += (long long)d[i] * suffix;
        }
        result += total;
    }

    /* Process b */
    {
        int *d = b.digits;
        long long suffix = 0;
        long long total = 0;
        for (int i = N - 1; i >= 0; i--) {
            if (d[i] != 0) suffix++;
            total += (long long)d[i] * suffix;
        }
        result += total;
    }

    /* Convert result to base 14 and print */
    if (result == 0) {
        printf("0\n");
        return 0;
    }

    char hex_chars[] = "0123456789abcd";
    char buf[64];
    int pos = 0;
    long long v = result;
    while (v > 0) {
        buf[pos++] = hex_chars[v % 14];
        v /= 14;
    }
    /* Reverse */
    for (int i = pos - 1; i >= 0; i--)
        putchar(buf[i]);
    putchar('\n');

    return 0;
}
