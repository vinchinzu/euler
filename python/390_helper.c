/*
 * Helper for Project Euler 390: find all fundamental solutions
 * of A^2 - (4p^2+1)*r^2 = p^2 for each p, then generate Pell chains.
 *
 * Compile: gcc -O2 -o 390_helper 390_helper.c -lm
 * Usage: ./390_helper
 * Output: S(10^10)
 */

#include <stdio.h>
#include <math.h>
#include <stdint.h>

typedef unsigned __int128 uint128;
typedef __int128 int128;

/* Integer square root for 64-bit values */
static inline uint64_t isqrt64(uint64_t n) {
    if (n == 0) return 0;
    uint64_t x = (uint64_t)sqrt((double)n);
    /* Correct for floating point errors */
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

/* Integer square root for 128-bit values */
static inline uint64_t isqrt128(uint128 n) {
    if (n == 0) return 0;
    /* Use double for initial approximation */
    uint64_t x = (uint64_t)sqrt((double)n);
    /* Newton's method refinement */
    for (int i = 0; i < 4; i++) {
        if (x == 0) break;
        uint128 x128 = (uint128)x;
        uint128 next = (x128 + n / x128) / 2;
        if (next >= x128 && next - x128 <= 1) break;
        if (x128 >= next && x128 - next <= 1) break;
        x = (uint64_t)next;
    }
    /* Exact correction */
    while ((uint128)x * x > n) x--;
    while ((uint128)(x + 1) * (x + 1) <= n) x++;
    return x;
}

int main(void) {
    const uint64_t N = 10000000000ULL; /* 10^10 */
    const uint64_t max_p = isqrt64(N / 2) + 1;

    uint128 total = 0;

    for (uint64_t p = 1; p <= max_p; p++) {
        uint64_t p2 = p * p;
        uint64_t D = 4 * p2 + 1;
        uint64_t x0 = 8 * p2 + 1;
        uint64_t y0 = 4 * p;

        /* Find fundamental solutions: A^2 - D*r^2 = p^2, r in [0, p-1] */
        for (uint64_t r = 0; r < p; r++) {
            uint128 val = (uint128)D * r * r + p2;
            uint64_t A = isqrt128(val);
            if ((uint128)A * A != val)
                continue;

            /* Found fundamental (A, r). Generate two chains. */

            /* Chain 1: from (A, r) forward */
            {
                uint128 cA = A, cq = r;
                while (1) {
                    if (cq >= p) {
                        if (cA <= N) {
                            total += cA;
                        } else {
                            break;
                        }
                    }
                    if (cA > N) break;
                    uint128 nA = cA * x0 + cq * y0 * D;
                    uint128 nq = cA * y0 + cq * x0;
                    cA = nA;
                    cq = nq;
                }
            }

            /* Chain 2: from (A, -r) forward */
            if (r > 0) {
                int128 sA = (int128)((int128)A * x0 - (int128)r * y0 * D);
                int128 sq = -(int128)A * y0 + (int128)r * x0;
                if (sq < 0) sq = -sq;
                if (sA < 0) sA = -sA;
                uint128 cA = (uint128)sA, cq = (uint128)sq;
                while (1) {
                    if (cq >= p) {
                        if (cA <= N) {
                            total += cA;
                        } else {
                            break;
                        }
                    }
                    if (cA > N) break;
                    uint128 nA = cA * x0 + cq * y0 * D;
                    uint128 nq = cA * y0 + cq * x0;
                    cA = nA;
                    cq = nq;
                }
            }
        }
    }

    /* Print result (128-bit to string) */
    uint64_t hi = (uint64_t)(total / 1000000000000ULL);
    uint64_t lo = (uint64_t)(total % 1000000000000ULL);
    if (hi > 0) {
        printf("%lu%012lu\n", hi, lo);
    } else {
        printf("%lu\n", lo);
    }

    return 0;
}
