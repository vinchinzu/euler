/*
 * Project Euler 768 - Chandelier Balance
 *
 * Find number of ways to place K=20 candles on N=360 evenly spaced positions
 * such that the chandelier is balanced (complex sum = 0).
 *
 * Uses meet-in-the-middle on a ring of rad(N)=30 positions,
 * then raises a generating polynomial to the (N/rad(N))=12 power.
 *
 * Works in Z_p for a prime p = 1 (mod 30) with 30th roots of unity.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef __int128 i128;

#define KK 20
#define NN 360

static int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Find a generator mod p */
static int64_t find_generator(int64_t p) {
    int64_t phi = p - 1;
    /* Factor phi */
    int64_t factors[64];
    int nfactors = 0;
    int64_t temp = phi;
    for (int64_t i = 2; i * i <= temp; i++) {
        if (temp % i == 0) {
            factors[nfactors++] = i;
            while (temp % i == 0) temp /= i;
        }
    }
    if (temp > 1) factors[nfactors++] = temp;

    for (int64_t g = 2; g < p; g++) {
        int ok = 1;
        for (int i = 0; i < nfactors; i++) {
            if (pow_mod(g, phi / factors[i], p) == 1) {
                ok = 0;
                break;
            }
        }
        if (ok) return g;
    }
    return -1;
}

/* rad(n) = product of distinct prime factors */
static int rad(int n) {
    int result = 1;
    for (int p = 2; p * p <= n; p++) {
        if (n % p == 0) {
            result *= p;
            while (n % p == 0) n /= p;
        }
    }
    if (n > 1) result *= n;
    return result;
}

/* Find prime p = 1 (mod L) near 2^31 */
static int64_t find_prime(int L) {
    /* Start from a large number and search */
    for (int64_t p = 2000000000LL + 1; ; p += 2) {
        if (p % L != 1) continue;
        /* Check primality */
        int is_prime = 1;
        for (int64_t i = 2; i * i <= p; i++) {
            if (p % i == 0) { is_prime = 0; break; }
        }
        if (is_prime) return p;
    }
}

/* Polynomial multiplication mod x^(KK+1) using int64_t coefficients */
static void poly_mul(int64_t *a, int64_t *b, int64_t *result, int deg) {
    int64_t temp[KK + 1];
    memset(temp, 0, sizeof(temp));
    for (int i = 0; i <= deg; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j <= deg && i + j <= deg; j++) {
            temp[i + j] += a[i] * b[j];
        }
    }
    memcpy(result, temp, (deg + 1) * sizeof(int64_t));
}

/* Polynomial exponentiation mod x^(KK+1) */
static void poly_pow(int64_t *base_poly, int exp, int64_t *result, int deg) {
    /* result = 1 */
    memset(result, 0, (deg + 1) * sizeof(int64_t));
    result[0] = 1;

    int64_t b[KK + 1];
    memcpy(b, base_poly, (deg + 1) * sizeof(int64_t));

    while (exp > 0) {
        if (exp & 1)
            poly_mul(result, b, result, deg);
        poly_mul(b, b, b, deg);
        exp >>= 1;
    }
}

int main(void) {
    int L = rad(NN);  /* 30 */
    int half = L / 2; /* 15 */

    /* Find suitable prime */
    int64_t p = find_prime(L);
    int64_t g = find_generator(p);
    int64_t w = pow_mod(g, (p - 1) / L, p); /* L-th root of unity */

    /* Compute powers of w: ws[i] = w^i mod p */
    int64_t ws[31];
    ws[0] = 1;
    for (int i = 1; i < L; i++)
        ws[i] = (i128)ws[i - 1] * w % p;

    /* Meet in the middle: enumerate subsets of first half (positions 0..half-1) */
    /* For each subset, compute sum of ws[position] and count of positions */
    /* Store in hash map: key = weight, value = array of counts by bit count */

    /* Hash map: weight -> counts[0..L] */
    #define MITM_HASH_SIZE (1 << 18)
    #define MITM_HASH_MASK (MITM_HASH_SIZE - 1)

    typedef struct {
        int64_t weight;
        int64_t counts[KK + 1];
        int next;
    } MitmEntry;

    int mitm_cap = 1 << half;
    if (mitm_cap > 40000) mitm_cap = 40000;
    MitmEntry *entries = (MitmEntry *)calloc(mitm_cap, sizeof(MitmEntry));
    int *mitm_buckets = (int *)malloc(MITM_HASH_SIZE * sizeof(int));
    memset(mitm_buckets, -1, MITM_HASH_SIZE * sizeof(int));
    int entry_count = 0;

    for (int subset = 0; subset < (1 << half); subset++) {
        int64_t weight = 0;
        int bit_count = 0;
        for (int i = 0; i < half; i++) {
            if (subset & (1 << i)) {
                weight = (weight + ws[i]) % p;
                bit_count++;
            }
        }
        if (bit_count > KK) continue;

        /* Insert/find in hash map */
        int bucket = (int)(weight & MITM_HASH_MASK);
        int found = -1;
        for (int idx = mitm_buckets[bucket]; idx != -1; idx = entries[idx].next) {
            if (entries[idx].weight == weight) {
                found = idx;
                break;
            }
        }
        if (found == -1) {
            found = entry_count++;
            entries[found].weight = weight;
            memset(entries[found].counts, 0, sizeof(entries[found].counts));
            entries[found].next = mitm_buckets[bucket];
            mitm_buckets[bucket] = found;
        }
        entries[found].counts[bit_count]++;
    }

    /* Now enumerate subsets of second half (positions half..L-1) */
    /* For each, find matching first-half with same weight (so combined = 0 mod p) */
    int64_t num_balanced[KK + 1];
    memset(num_balanced, 0, sizeof(num_balanced));

    int second_half = L - half;
    for (int subset = 0; subset < (1 << second_half); subset++) {
        int64_t weight = 0;
        int bit_count = 0;
        for (int i = 0; i < second_half; i++) {
            if (subset & (1 << i)) {
                weight = (weight + ws[half + i]) % p;
                bit_count++;
            }
        }
        if (bit_count > KK) continue;

        /* Need first-half weight such that first_weight + weight = 0 (mod p) */
        int64_t target = (p - weight) % p;
        int bucket = (int)(target & MITM_HASH_MASK);
        for (int idx = mitm_buckets[bucket]; idx != -1; idx = entries[idx].next) {
            if (entries[idx].weight == target) {
                for (int bc1 = 0; bc1 <= KK - bit_count; bc1++) {
                    if (entries[idx].counts[bc1] > 0) {
                        num_balanced[bc1 + bit_count] += entries[idx].counts[bc1];
                    }
                }
                break;
            }
        }
    }

    /* Now raise the generating polynomial to the (N/L) power */
    int exponent = NN / L;  /* 12 */

    int64_t result[KK + 1];
    poly_pow(num_balanced, exponent, result, KK);

    printf("%lld\n", result[KK]);

    free(entries);
    free(mitm_buckets);
    return 0;
}
