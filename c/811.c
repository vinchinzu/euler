/*
 * Project Euler Problem 811: Recursive Binary Function.
 *
 * H(t, r) = A((2^t + 1)^r) where A is computed from the binary representation.
 * The binary representation of (2^t + 1)^r = sum C(r,i) * 2^(t*i).
 * For t >= 64, each binomial coefficient fits in the t-bit gap between terms,
 * so the binary is just the binary of C(r,0), gap, C(r,1), gap, ..., C(r,r).
 *
 * Then A(n) = product of mult_j^(gap_j) where mult_{j+1} = 5*mult_j + 3.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

#define MAX_WORDS 128  /* enough for ~8000 bits */

static ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base = base % mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static ll mod_inv(ll a, ll mod) {
    return pow_mod(a, mod - 2, mod);
}

/* Bignum stored as array of 64-bit words (little-endian) */
typedef struct {
    ull words[MAX_WORDS];
    int nwords;
} BigNum;

static void bn_zero(BigNum *b) {
    memset(b->words, 0, sizeof(b->words));
    b->nwords = 0;
}

/* Add val (up to 64 bits) shifted left by bit_shift bits */
static void bn_add_at(BigNum *b, ull val, int bit_shift) {
    if (val == 0) return;
    int word_off = bit_shift / 64;
    int bit_off = bit_shift % 64;

    /* val shifted by bit_off within word boundaries */
    ull lo, hi;
    if (bit_off == 0) {
        lo = val;
        hi = 0;
    } else {
        lo = val << bit_off;
        hi = val >> (64 - bit_off);
    }

    /* Add lo to word_off with carry propagation */
    ull carry = 0;
    int idx = word_off;

    /* Add lo */
    ull sum = b->words[idx] + lo;
    carry = (sum < b->words[idx]) ? 1 : 0;
    b->words[idx] = sum;
    idx++;

    /* Add hi + carry */
    sum = b->words[idx] + hi + carry;
    if (sum < b->words[idx] || (carry && sum <= b->words[idx])) carry = 1;
    else carry = (hi && sum < hi) ? 1 : 0;
    /* More careful carry detection */
    {
        ull old = b->words[idx];
        ull added = hi + carry;
        /* carry from hi + old_carry */
        int c1 = (added < hi) ? 1 : 0;
        ull s = old + added;
        int c2 = (s < old) ? 1 : 0;
        b->words[idx] = s;
        carry = c1 | c2;
    }
    idx++;

    /* Propagate carry */
    while (carry && idx < MAX_WORDS) {
        ull s = b->words[idx] + carry;
        carry = (s < b->words[idx]) ? 1 : 0;
        b->words[idx] = s;
        idx++;
    }

    /* Update nwords */
    for (int i = MAX_WORDS - 1; i >= b->nwords; i--) {
        if (b->words[i] != 0) { b->nwords = i + 1; break; }
    }
}

static int bn_bit(const BigNum *b, int pos) {
    int word = pos / 64;
    int bit = pos % 64;
    if (word >= MAX_WORDS) return 0;
    return (b->words[word] >> bit) & 1;
}

static int bn_highest_bit(const BigNum *b) {
    for (int w = MAX_WORDS - 1; w >= 0; w--) {
        if (b->words[w]) {
            return w * 64 + 63 - __builtin_clzll(b->words[w]);
        }
    }
    return -1;
}

static ll H(int t, int r, ll M) {
    /* Compute (2^t + 1)^r as a big number using binomial expansion */
    BigNum num;
    bn_zero(&num);

    /* C(r, i) computed iteratively as 64-bit (may overflow for large r) */
    /* C(62, i) max ~ 2^61, fits in ull */
    ull binom = 1;
    for (int i = 0; i <= r; i++) {
        bn_add_at(&num, binom, t * i);
        if (i < r) {
            /* binom = binom * (r - i) / (i + 1) */
            /* Be careful of overflow: binom * (r-i) could overflow 64-bit */
            /* C(62, 31) ~ 9.16e17 which fits in ull. (r-i) <= 62 so product <= ~5.7e19, fits in ull */
            binom = binom * (ull)(r - i) / (ull)(i + 1);
        }
    }

    int hb = bn_highest_bit(&num);
    if (hb < 0) return 1;

    /* Extract gap structure: split binary by '1', for each group of zeros multiply result */
    ll result = 1;
    ll mult = 1;
    int gap_len = 0;

    for (int pos = hb; pos >= 0; pos--) {
        if (bn_bit(&num, pos)) {
            if (gap_len > 0) {
                result = (i128)result * pow_mod(mult, gap_len, M) % M;
            }
            mult = (5 * mult + 3) % M;
            gap_len = 0;
        } else {
            gap_len++;
        }
    }

    return result;
}

int main(void) {
    ll N = 100000000000000LL + 31;  /* 10^14 + 31 */
    int K = 62;
    ll M = 1000062031LL;

    ll v1 = H(K + 1, K, M);
    ll v2 = H(K + 2, K, M);
    ll ratio = (i128)v2 * mod_inv(v1, M) % M;

    ll answer = (i128)v1 * pow_mod(ratio, N - K - 1, M) % M;

    printf("%lld\n", answer);
    return 0;
}
