/*
 * Project Euler Problem 592: Factorial Trailing Hex Digits.
 *
 * Find the last 12 hexadecimal digits before the trailing zeroes in (20!)!.
 *
 * Uses the recursive formula: odd_part(N!) = PO(N) * odd_part(floor(N/2)!)
 * where PO(N) = product of odd numbers from 1 to N.
 *
 * PO(N) mod 2^48 is computed using baby-step/giant-step with polynomial
 * interpolation, exploiting the fact that the product of 2^47 consecutive
 * odd numbers is 1 mod 2^48.
 *
 * Extracted from embedded C in Python solution.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef unsigned long long u64;
typedef __int128 u128;

#define NBITS 48
#define MOD ((u64)1 << NBITS)        /* 2^48 */
#define HALF_MOD ((u64)1 << (NBITS-1)) /* 2^47 */
#define MASK (MOD - 1)

/* Degree of the polynomial f(a) mod 2^48 */
#define POLY_DEG 26  /* slightly more than n/2 for safety */

/* Block size for BSGS: 2^22 = ~4M. Balances precomp vs eval cost. */
#define BLOCK_BITS 22
#define BLOCK_SIZE ((u64)1 << BLOCK_BITS)

static inline u64 mulmod(u64 a, u64 b) {
    return ((u128)a * b) & MASK;
}

static inline u64 addmod(u64 a, u64 b) {
    return (a + b) & MASK;
}

static inline u64 submod(u64 a, u64 b) {
    return (a - b) & MASK;
}

/*
 * Compute product of r consecutive odd numbers: 1*3*5*...*(2r-1) mod 2^48.
 * r is already reduced mod 2^47.
 *
 * Uses BSGS with polynomial interpolation.
 */
u64 product_of_odds(u64 r) {
    if (r == 0) return 1;
    if (r == 1) return 1;

    u64 B = BLOCK_SIZE;

    /* If r is small enough, compute directly */
    if (r <= B || r <= (u64)(POLY_DEG + 2)) {
        u64 result = 1;
        for (u64 j = 0; j < r; j++) {
            result = mulmod(result, (2*j + 1) & MASK);
        }
        return result;
    }

    /* Step 1: Precompute f(a) for a = 0, 1, ..., POLY_DEG */
    u64 f_vals[POLY_DEG + 1];
    for (int a = 0; a <= POLY_DEG; a++) {
        u64 prod = 1;
        u64 base = (u64)a * B;  /* starting odd number: 2*(a*B) + 1 */
        for (u64 j = 0; j < B; j++) {
            u64 odd_num = (2 * (base + j) + 1) & MASK;
            prod = mulmod(prod, odd_num);
        }
        f_vals[a] = prod;
    }

    /* Step 2: Build forward difference table */
    u64 deltas[POLY_DEG + 1];
    deltas[0] = f_vals[0];
    /* Compute differences in place */
    u64 work[POLY_DEG + 1];
    memcpy(work, f_vals, sizeof(f_vals));
    for (int k = 0; k < POLY_DEG; k++) {
        for (int i = 0; i < POLY_DEG - k; i++) {
            work[i] = submod(work[i+1], work[i]);
        }
        deltas[k+1] = work[0];
    }

    /* Step 3: Evaluate f at a=0,1,...,q-1 and accumulate product */
    u64 q = r / B;
    u64 remainder = r % B;

    u64 d[POLY_DEG + 1];
    memcpy(d, deltas, sizeof(deltas));

    u64 result = 1;
    for (u64 a = 0; a < q; a++) {
        result = mulmod(result, d[0]);
        for (int k = 0; k < POLY_DEG; k++) {
            d[k] = addmod(d[k], d[k+1]);
        }
    }

    /* Step 4: Handle partial last block */
    if (remainder > 0) {
        u64 base = q * B;
        for (u64 j = 0; j < remainder; j++) {
            u64 odd_num = (2 * (base + j) + 1) & MASK;
            result = mulmod(result, odd_num);
        }
    }

    return result;
}

int main() {
    /* N = 20! */
    /* 20! = 2432902008176640000 */
    u64 N = 2432902008176640000ULL;

    /* Compute odd_part(N!) mod 2^48 */
    u64 odd_part = 1;
    u64 cur = N;

    while (cur > 1) {
        /* r = ceil(cur / 2) = number of odd numbers from 1 to cur */
        u64 r = (cur + 1) / 2;
        /* Reduce r mod 2^47 */
        u64 r_red = r & (HALF_MOD - 1);

        u64 po = product_of_odds(r_red);
        odd_part = mulmod(odd_part, po);

        cur = cur / 2;
    }

    /* Compute v2(N!) = sum floor(N/2^k) */
    u64 v2 = 0;
    u64 t = N;
    while (t > 1) {
        t /= 2;
        v2 += t;
    }

    /* Answer = odd_part * 2^(v2 mod 4) mod 2^48 */
    u64 pow2 = 1ULL << (v2 % 4);
    u64 answer = mulmod(odd_part, pow2);

    /* Print as uppercase hex, exactly 12 digits */
    printf("%012llX\n", (unsigned long long)answer);

    return 0;
}
