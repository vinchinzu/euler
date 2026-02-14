/*
 * Project Euler Problem 176: Right-angled triangles sharing a cathetus.
 *
 * Find the smallest n such that the number of right triangles with
 * n as a cathetus is exactly 47547.
 *
 * The count of right triangles with cathetus n is:
 *   For n = 2^a * m (m odd), the count is:
 *     (prod (2*e_i + 1) - 1) / 2      if a <= 1
 *     ((2a-1) * prod(2*e_i+1) - 1) / 2 if a >= 2
 *   where m = prod p_i^e_i.
 *
 * We need count = 47547, so 2*47547+1 = 95095 = product of odd factors.
 * 95095 = 5 * 7 * 11 * 13 * 19
 *
 * For a <= 1: we need prod(2*e_i + 1) = 95095
 * For a >= 2: we need (2a-1) * prod(2*e_i + 1) = 95095
 *
 * Factor 95095 as product of odd numbers >= 3 (or 1 for the trivial factor).
 * The exponents e_i = (f_i - 1) / 2 where f_i are the factors.
 * To minimize n, assign largest exponents to smallest primes.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long i64;
typedef unsigned long long u64;

#define VALUE 95095  /* 2 * 47547 + 1 */

static int odd_primes[] = {3, 5, 7, 11, 13, 17, 19, 23, 29, 31};
static int n_odd_primes = 10;

/* Prime factorization */
typedef struct { int prime; int exp; } Factor;

static int factorize(int n, Factor *factors) {
    int nf = 0;
    for (int d = 2; (i64)d * d <= n; d++) {
        if (n % d == 0) {
            factors[nf].prime = d;
            factors[nf].exp = 0;
            while (n % d == 0) { n /= d; factors[nf].exp++; }
            nf++;
        }
    }
    if (n > 1) { factors[nf].prime = n; factors[nf].exp = 1; nf++; }
    return nf;
}

/* Get all divisors of n */
static int get_divisors(int n, int *divs) {
    Factor factors[20];
    int nf = factorize(n, factors);
    divs[0] = 1;
    int nd = 1;
    for (int i = 0; i < nf; i++) {
        int prev_nd = nd;
        int pk = 1;
        for (int e = 1; e <= factors[i].exp; e++) {
            pk *= factors[i].prime;
            for (int j = 0; j < prev_nd; j++)
                divs[nd++] = divs[j] * pk;
        }
    }
    return nd;
}

/* Compute minimal m for a given f = product of (2e_i + 1) factors.
 * f's prime factorization gives us the exponent structure.
 * Each prime factor p with multiplicity k contributes k exponents of (p-1)/2.
 * Assign largest exponents to smallest odd primes to minimize m. */
static u64 minimal_m_for(int f) {
    Factor factors[20];
    int nf = factorize(f, factors);

    int exponents[50];
    int ne = 0;
    for (int i = 0; i < nf; i++)
        for (int j = 0; j < factors[i].exp; j++)
            exponents[ne++] = (factors[i].prime - 1) / 2;

    /* Sort descending */
    for (int i = 0; i < ne - 1; i++)
        for (int j = i + 1; j < ne; j++)
            if (exponents[i] < exponents[j]) {
                int tmp = exponents[i]; exponents[i] = exponents[j]; exponents[j] = tmp;
            }

    u64 m = 1;
    for (int i = 0; i < ne; i++) {
        u64 p = odd_primes[i];
        for (int e = 0; e < exponents[i]; e++)
            m *= p;
    }
    return m;
}

int main(void) {
    u64 best = (u64)-1;

    /* Case a = 0 or a = 1: need product = VALUE */
    for (int a = 0; a <= 1; a++) {
        u64 m = minimal_m_for(VALUE);
        u64 n = (1ULL << a) * m;
        if (n < best) best = n;
    }

    /* Case a >= 2: need (2a-1) * prod = VALUE, so (2a-1) | VALUE and (2a-1) >= 3 */
    int divs[500];
    int nd = get_divisors(VALUE, divs);

    for (int di = 0; di < nd; di++) {
        int d = divs[di];
        if (d < 3) continue;
        if ((d + 1) % 2 != 0) continue;
        int a = (d + 1) / 2;
        if (a < 2) continue;
        if (a >= 64) continue;  /* would overflow u64 */
        int f = VALUE / d;
        u64 m = minimal_m_for(f);
        u64 power = 1ULL << a;
        if (m > 0 && power > (u64)-1 / m) continue;  /* mul overflow */
        u64 n = power * m;
        if (n < best) best = n;
    }

    printf("%llu\n", best);
    return 0;
}
