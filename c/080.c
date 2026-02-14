/* Project Euler 080 - Square root digital expansion
 * For non-perfect-square n in 1..100, sum first 100 decimal digits of sqrt(n).
 * Uses big integer arithmetic with base-10000 for speed. */
#include <stdio.h>
#include <string.h>

/* Big integer in base 10000, little-endian */
#define BASE 10000
#define MAXW 120  /* enough for ~480 decimal digits */

typedef struct {
    int w[MAXW];
    int len;
} Big;

static void big_zero(Big *b) { memset(b->w, 0, sizeof(b->w)); b->len = 1; }

static void big_from_int(Big *b, long long v) {
    big_zero(b);
    b->len = 0;
    if (v == 0) { b->w[0] = 0; b->len = 1; return; }
    while (v > 0) { b->w[b->len++] = (int)(v % BASE); v /= BASE; }
}

static int big_cmp(const Big *a, const Big *b) {
    if (a->len != b->len) return a->len - b->len;
    for (int i = a->len - 1; i >= 0; i--)
        if (a->w[i] != b->w[i]) return a->w[i] - b->w[i];
    return 0;
}

static void big_add(Big *c, const Big *a, const Big *b) {
    int maxl = a->len > b->len ? a->len : b->len;
    int carry = 0;
    c->len = 0;
    for (int i = 0; i < maxl || carry; i++) {
        int s = carry;
        if (i < a->len) s += a->w[i];
        if (i < b->len) s += b->w[i];
        c->w[c->len++] = s % BASE;
        carry = s / BASE;
    }
}

/* c = a - b, assumes a >= b */
static void big_sub(Big *c, const Big *a, const Big *b) {
    int borrow = 0;
    c->len = a->len;
    for (int i = 0; i < a->len; i++) {
        int d = a->w[i] - borrow - (i < b->len ? b->w[i] : 0);
        if (d < 0) { d += BASE; borrow = 1; } else borrow = 0;
        c->w[i] = d;
    }
    while (c->len > 1 && c->w[c->len-1] == 0) c->len--;
}

/* c = a * b (full multiply) */
static void big_mul(Big *c, const Big *a, const Big *b) {
    long long tmp[MAXW * 2];
    int clen = a->len + b->len;
    memset(tmp, 0, sizeof(long long) * clen);
    for (int i = 0; i < a->len; i++) {
        long long carry = 0;
        for (int j = 0; j < b->len; j++) {
            tmp[i+j] += (long long)a->w[i] * b->w[j] + carry;
            carry = tmp[i+j] / BASE;
            tmp[i+j] %= BASE;
        }
        if (carry) tmp[i + b->len] += carry;
    }
    while (clen > 1 && tmp[clen-1] == 0) clen--;
    c->len = clen;
    for (int i = 0; i < clen; i++) c->w[i] = (int)tmp[i];
}

/* c = a / small_div (single-word divisor) */
static void big_div_small(Big *c, const Big *a, int d) {
    long long rem = 0;
    c->len = a->len;
    for (int i = a->len - 1; i >= 0; i--) {
        rem = rem * BASE + a->w[i];
        c->w[i] = (int)(rem / d);
        rem %= d;
    }
    while (c->len > 1 && c->w[c->len-1] == 0) c->len--;
}

/* c = a / b (big / big), quotient only */
static void big_div(Big *q, const Big *a, const Big *b) {
    /* Use Newton's method for division: compute 1/b, then multiply.
     * Actually, let's do long division in base 10000. */
    Big rem;
    big_zero(&rem);
    int qw[MAXW];
    int qlen = 0;

    for (int i = a->len - 1; i >= 0; i--) {
        /* Shift remainder left by 1 word */
        for (int j = rem.len; j >= 1; j--) rem.w[j] = rem.w[j-1];
        rem.w[0] = a->w[i];
        rem.len++;
        while (rem.len > 1 && rem.w[rem.len-1] == 0) rem.len--;

        /* Binary search for quotient digit */
        int lo = 0, hi = BASE - 1, qd = 0;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            /* Compute b * mid */
            Big prod;
            prod.len = 0;
            long long carry = 0;
            for (int j = 0; j < b->len; j++) {
                long long v = (long long)b->w[j] * mid + carry;
                prod.w[prod.len++] = (int)(v % BASE);
                carry = v / BASE;
            }
            if (carry) prod.w[prod.len++] = (int)carry;
            if (prod.len == 0) prod.len = 1;

            if (big_cmp(&prod, &rem) <= 0) {
                qd = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        qw[qlen++] = qd;

        /* rem -= b * qd */
        if (qd > 0) {
            Big prod;
            prod.len = 0;
            long long carry = 0;
            for (int j = 0; j < b->len; j++) {
                long long v = (long long)b->w[j] * qd + carry;
                prod.w[prod.len++] = (int)(v % BASE);
                carry = v / BASE;
            }
            if (carry) prod.w[prod.len++] = (int)carry;
            Big tmp;
            big_sub(&tmp, &rem, &prod);
            rem = tmp;
        }
    }

    /* qw is MSB-first, convert to little-endian */
    q->len = qlen;
    for (int i = 0; i < qlen; i++) q->w[i] = qw[qlen - 1 - i];
    while (q->len > 1 && q->w[q->len-1] == 0) q->len--;
}

/* Newton's method sqrt for big integers */
static void big_sqrt(Big *result, const Big *n) {
    if (n->len == 1 && n->w[0] == 0) { big_zero(result); return; }

    /* Initial guess must be >= sqrt(n) so Newton converges from above.
     * Use BASE^(ceil(len/2)) which is always larger. */
    Big x;
    big_zero(&x);
    int half = (n->len + 1) / 2;
    x.len = half + 1;
    x.w[half] = 1;  /* x = BASE^half, guaranteed > sqrt(n) */

    for (int iter = 0; iter < 500; iter++) {
        Big q;
        big_div(&q, n, &x);
        Big sum;
        big_add(&sum, &x, &q);
        Big new_x;
        big_div_small(&new_x, &sum, 2);

        if (big_cmp(&new_x, &x) >= 0) break;
        x = new_x;
    }

    /* Adjust: ensure x*x <= n */
    Big sq;
    big_mul(&sq, &x, &x);
    if (big_cmp(&sq, n) > 0) {
        Big one; big_from_int(&one, 1);
        Big tmp; big_sub(&tmp, &x, &one);
        x = tmp;
    }

    *result = x;
}

/* Convert big number to decimal string (big-endian) */
static int big_to_str(const Big *b, char *s) {
    /* Each word is up to 4 decimal digits */
    int pos = 0;
    /* Print MSB word without leading zeros */
    int msb = b->w[b->len - 1];
    if (msb == 0 && b->len == 1) { s[pos++] = '0'; s[pos] = 0; return pos; }
    char tmp[8];
    int tl = 0;
    if (msb == 0) msb = 0; /* shouldn't happen */
    int m = msb;
    if (m == 0) { tmp[tl++] = '0'; }
    else { while (m > 0) { tmp[tl++] = '0' + m % 10; m /= 10; } }
    for (int i = tl - 1; i >= 0; i--) s[pos++] = tmp[i];

    /* Print remaining words with leading zeros (4 digits each) */
    for (int w = b->len - 2; w >= 0; w--) {
        int v = b->w[w];
        s[pos++] = '0' + (v / 1000) % 10;
        s[pos++] = '0' + (v / 100) % 10;
        s[pos++] = '0' + (v / 10) % 10;
        s[pos++] = '0' + v % 10;
    }
    s[pos] = 0;
    return pos;
}

int main(void) {
    int total = 0;

    for (int n = 1; n <= 100; n++) {
        int rt = 1;
        while (rt * rt < n) rt++;
        if (rt * rt == n) continue;

        /* Compute n * BASE^100 = n * 10000^100 = n * 10^400 */
        /* Then sqrt gives ~200 decimal digits */
        Big big_n;
        big_zero(&big_n);
        big_n.len = 101;
        /* n in the top word(s) */
        big_n.w[100] = n;

        Big root;
        big_sqrt(&root, &big_n);

        /* Convert to decimal string */
        char str[600];
        big_to_str(&root, str);

        /* Sum first 100 digits */
        int sum = 0;
        for (int i = 0; i < 100 && str[i]; i++)
            sum += str[i] - '0';
        total += sum;
    }

    printf("%d\n", total);
    return 0;
}
