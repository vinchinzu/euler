/*
 * Project Euler Problem 813: XOR Power.
 *
 * Compute K^N in XOR multiplication where K=11, N=8^12 * 12^8 = 2^52 * 6561.
 * XOR multiplication = polynomial multiplication over GF(2).
 * K = 11 = x^3 + x + 1 in GF(2)[x].
 *
 * Frobenius: K^(2^52) = x^(3*2^52) + x^(2^52) + 1.
 * All positions are multiples of 2^52, so work with reduced positions.
 * Raise {0, 1, 3} to power 6561 in GF(2) polynomial arithmetic.
 * Max reduced position = 3*6561 = 19683, very manageable.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

static ll pow_mod_ll(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Polynomial over GF(2) represented as a bitset.
 * Bit i is set iff x^i has coefficient 1. */
#define MAX_DEG 20000  /* 3*6561 + slack */
#define NWORDS ((MAX_DEG + 63) / 64)

typedef struct {
    uint64_t bits[NWORDS];
} GF2Poly;

static void poly_zero(GF2Poly *p) {
    memset(p->bits, 0, sizeof(p->bits));
}

static int poly_bit(const GF2Poly *p, int pos) {
    return (p->bits[pos / 64] >> (pos % 64)) & 1;
}

static void poly_flip(GF2Poly *p, int pos) {
    p->bits[pos / 64] ^= (uint64_t)1 << (pos % 64);
}

static void poly_set(GF2Poly *p, int pos) {
    p->bits[pos / 64] |= (uint64_t)1 << (pos % 64);
}

/* Multiply two GF(2) polynomials.
 * Standard approach: for each set bit in a, XOR shifted b into result. */
static void poly_mul(const GF2Poly *a, const GF2Poly *b, GF2Poly *result) {
    poly_zero(result);

    /* Find max degree of a and b */
    int max_a = -1, max_b = -1;
    for (int w = NWORDS - 1; w >= 0; w--) {
        if (a->bits[w] && max_a < 0) max_a = w * 64 + 63 - __builtin_clzll(a->bits[w]);
        if (b->bits[w] && max_b < 0) max_b = w * 64 + 63 - __builtin_clzll(b->bits[w]);
        if (max_a >= 0 && max_b >= 0) break;
    }
    if (max_a < 0 || max_b < 0) return;

    /* For each set bit in a, XOR b shifted by that amount into result */
    for (int i = 0; i <= max_a; i++) {
        if (!poly_bit(a, i)) continue;

        /* XOR b << i into result */
        int word_off = i / 64;
        int bit_off = i % 64;

        if (bit_off == 0) {
            int max_w_b = max_b / 64;
            for (int w = 0; w <= max_w_b; w++) {
                result->bits[w + word_off] ^= b->bits[w];
            }
        } else {
            int max_w_b = max_b / 64;
            for (int w = 0; w <= max_w_b; w++) {
                result->bits[w + word_off] ^= b->bits[w] << bit_off;
                if (w + word_off + 1 < NWORDS)
                    result->bits[w + word_off + 1] ^= b->bits[w] >> (64 - bit_off);
            }
        }
    }
}

int main(void) {
    /* K^(2^52) represented with reduced positions: {0, 1, 3} */
    /* Raise to power 6561 */

    GF2Poly base, result, temp;
    poly_zero(&base);
    poly_set(&base, 0);
    poly_set(&base, 1);
    poly_set(&base, 3);

    /* result = 1 (x^0) */
    poly_zero(&result);
    poly_set(&result, 0);

    int n = 6561;
    int hb = 0;
    { int tmp = n; while (tmp > 0) { hb++; tmp >>= 1; } hb--; }

    for (int bit = hb; bit >= 0; bit--) {
        /* Square result */
        poly_mul(&result, &result, &temp);
        /* Copy temp to result */
        memcpy(&result, &temp, sizeof(GF2Poly));

        if ((n >> bit) & 1) {
            /* Multiply by base */
            poly_mul(&result, &base, &temp);
            memcpy(&result, &temp, sizeof(GF2Poly));
        }
    }

    /* Now result represents K^N with reduced positions.
     * Actual position = reduced_pos * 2^52.
     * Answer = sum of 2^(reduced_pos * 2^52) mod M.
     * = sum of (2^(2^52))^reduced_pos mod M.
     */
    ll M = 1000000007LL;
    ll base_val = pow_mod_ll(2, (ll)1 << 52, M);  /* 2^(2^52) mod M */

    ll ans = 0;
    for (int i = 0; i < MAX_DEG; i++) {
        if (poly_bit(&result, i)) {
            ans = (ans + pow_mod_ll(base_val, i, M)) % M;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
