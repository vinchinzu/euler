/*
 * Project Euler Problem 418: Factorisation triples.
 *
 * Find the minimum possible value of a+b+c if a*b*c = 43!.
 *
 * 43! = product of primes^exponents. We enumerate factors of 43! near
 * the cube root, then brute force over pairs to find minimum sum.
 *
 * Since 43! is huge (~5.1e52), we use GMP-like multi-precision or just
 * represent factors by their prime exponent vectors and compare.
 *
 * Strategy: represent each factor by its exponent vector over primes up to 43.
 * Use Python-like arbitrary precision via __int128 pairs, or more practically,
 * represent the "value" as a pair of doubles (for comparison) + exact exponent
 * vectors (for exact division).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

/* Primes up to 43 */
static const int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43};
#define NPRIMES 14

/* Exponents of each prime in 43! */
static int fact_exp[NPRIMES];

static int num_factors_in_factorial(int n, int p) {
    int count = 0;
    int power = p;
    while (power <= n) {
        count += n / power;
        if (power > n / p) break;  /* avoid overflow */
        power *= p;
    }
    return count;
}

/* A factor of 43! represented by its exponent vector */
typedef struct {
    int exp[NPRIMES];
    double logval;  /* log of the factor value */
} Factor;

static Factor *factors;
static int nfactors;
static int factors_cap;

static double log_primes[NPRIMES];
static double cube_root_log;

static void compute_logval(Factor *f) {
    f->logval = 0.0;
    for (int i = 0; i < NPRIMES; i++)
        f->logval += f->exp[i] * log_primes[i];
}

/* Recursively enumerate factors with log in [lo, hi] */
static void enumerate(int idx, int exp[], double logv, double lo, double hi) {
    if (logv > hi) return;
    if (idx == NPRIMES) {
        if (logv >= lo) {
            if (nfactors >= factors_cap) {
                factors_cap *= 2;
                factors = (Factor *)realloc(factors, factors_cap * sizeof(Factor));
            }
            memcpy(factors[nfactors].exp, exp, NPRIMES * sizeof(int));
            factors[nfactors].logval = logv;
            nfactors++;
        }
        return;
    }

    for (int e = 0; e <= fact_exp[idx]; e++) {
        exp[idx] = e;
        enumerate(idx + 1, exp, logv + e * log_primes[idx], lo, hi);
    }
    exp[idx] = 0;
}

/* Compare factors by logval */
static int cmp_factor(const void *a, const void *b) {
    double d = ((const Factor *)a)->logval - ((const Factor *)b)->logval;
    if (d < 0) return -1;
    if (d > 0) return 1;
    return 0;
}

/* Check if exp1[i] + exp2[i] <= fact_exp[i] for all i */
static int compatible(const Factor *f1, const Factor *f2) {
    for (int i = 0; i < NPRIMES; i++)
        if (f1->exp[i] + f2->exp[i] > fact_exp[i]) return 0;
    return 1;
}

/* Compute the sum f1 + f2 + (43!/f1/f2) as a double (for comparison) */
static double compute_sum_log(const Factor *f1, const Factor *f2) {
    /* The three values are exp(f1->logval), exp(f2->logval), exp(log43! - f1->logval - f2->logval) */
    /* We want to minimize a + b + c. The sum is hard to compare via logs directly. */
    /* But since all three are near cube_root, we can use double. */
    /* Actually, for exact comparison we need big integers. Let's use a different approach. */
    double l1 = f1->logval;
    double l2 = f2->logval;
    double total_log = 0;
    for (int i = 0; i < NPRIMES; i++)
        total_log += fact_exp[i] * log_primes[i];
    double l3 = total_log - l1 - l2;
    return exp(l1) + exp(l2) + exp(l3);
}

/*
 * Big number as array of digits (base 10^9) for exact sum comparison.
 * Each big number has at most 18 limbs (54 digits is enough for 43!^(1/3) ~ 17 digits,
 * but the factors themselves can be up to 52 digits).
 */
#define MAXLIMBS 20
#define LIMB_BASE 1000000000ULL

typedef struct {
    ull limbs[MAXLIMBS];
    int nlimbs;
} BigNum;

static void bn_from_one(BigNum *b) {
    memset(b, 0, sizeof(*b));
    b->limbs[0] = 1;
    b->nlimbs = 1;
}

static void bn_mul_int(BigNum *b, ll x) {
    ull carry = 0;
    for (int i = 0; i < b->nlimbs; i++) {
        ull v = b->limbs[i] * (ull)x + carry;
        b->limbs[i] = v % LIMB_BASE;
        carry = v / LIMB_BASE;
    }
    while (carry) {
        b->limbs[b->nlimbs++] = carry % LIMB_BASE;
        carry /= LIMB_BASE;
    }
}

static int bn_cmp(const BigNum *a, const BigNum *b) {
    if (a->nlimbs != b->nlimbs) return a->nlimbs - b->nlimbs;
    for (int i = a->nlimbs - 1; i >= 0; i--) {
        if (a->limbs[i] != b->limbs[i])
            return a->limbs[i] < b->limbs[i] ? -1 : 1;
    }
    return 0;
}

static void bn_add(BigNum *res, const BigNum *a, const BigNum *b) {
    int n = a->nlimbs > b->nlimbs ? a->nlimbs : b->nlimbs;
    ull carry = 0;
    for (int i = 0; i < n; i++) {
        ull va = i < a->nlimbs ? a->limbs[i] : 0;
        ull vb = i < b->nlimbs ? b->limbs[i] : 0;
        ull s = va + vb + carry;
        res->limbs[i] = s % LIMB_BASE;
        carry = s / LIMB_BASE;
    }
    if (carry) {
        res->limbs[n] = carry;
        n++;
    }
    res->nlimbs = n;
}

static void bn_from_exponents(BigNum *b, const int exp[]) {
    bn_from_one(b);
    for (int i = 0; i < NPRIMES; i++) {
        for (int j = 0; j < exp[i]; j++) {
            bn_mul_int(b, primes[i]);
        }
    }
}

static void bn_print(const BigNum *b) {
    printf("%llu", b->limbs[b->nlimbs - 1]);
    for (int i = b->nlimbs - 2; i >= 0; i--)
        printf("%09llu", b->limbs[i]);
}

/* Divide 43! by f1 and f2 to get f3 exponents */
static void get_f3_exp(int f3_exp[], const Factor *f1, const Factor *f2) {
    for (int i = 0; i < NPRIMES; i++)
        f3_exp[i] = fact_exp[i] - f1->exp[i] - f2->exp[i];
}

int main(void) {
    /* Compute exponents of primes in 43! */
    for (int i = 0; i < NPRIMES; i++) {
        fact_exp[i] = num_factors_in_factorial(43, primes[i]);
        log_primes[i] = log((double)primes[i]);
    }

    double total_log = 0;
    for (int i = 0; i < NPRIMES; i++)
        total_log += fact_exp[i] * log_primes[i];
    cube_root_log = total_log / 3.0;

    /* Enumerate factors in a window around the cube root */
    /* R = 0.99999, window: [cube_root * R, cube_root / R] in log space */
    double margin = 15.0;  /* generous margin in log space */
    double lo = cube_root_log - margin;
    double hi = cube_root_log + margin;

    factors_cap = 1000000;
    factors = (Factor *)malloc(factors_cap * sizeof(Factor));
    nfactors = 0;

    int exp[NPRIMES];
    memset(exp, 0, sizeof(exp));
    enumerate(0, exp, 0.0, lo, hi);

    /* Sort by logval */
    qsort(factors, nfactors, sizeof(Factor), cmp_factor);

    /* Find minimum sum */
    BigNum best_sum;
    memset(&best_sum, 0, sizeof(best_sum));
    best_sum.nlimbs = MAXLIMBS;
    for (int i = 0; i < MAXLIMBS; i++) best_sum.limbs[i] = LIMB_BASE - 1;  /* infinity */

    for (int i = 0; i < nfactors; i++) {
        if (factors[i].logval > cube_root_log + 0.01) break;  /* f1 <= cube_root */
        for (int j = i; j < nfactors; j++) {
            double l2 = factors[j].logval;
            double l3 = total_log - factors[i].logval - l2;
            if (l2 > l3 + 0.01) break;  /* f2 <= f3 */
            if (!compatible(&factors[i], &factors[j])) continue;

            /* Compute sum */
            int f3_exp[NPRIMES];
            get_f3_exp(f3_exp, &factors[i], &factors[j]);

            /* Check f3 exponents are non-negative */
            int ok = 1;
            for (int k = 0; k < NPRIMES; k++)
                if (f3_exp[k] < 0) { ok = 0; break; }
            if (!ok) continue;

            BigNum b1, b2, b3, sum12, sum123;
            bn_from_exponents(&b1, factors[i].exp);
            bn_from_exponents(&b2, factors[j].exp);
            bn_from_exponents(&b3, f3_exp);
            bn_add(&sum12, &b1, &b2);
            bn_add(&sum123, &sum12, &b3);

            if (bn_cmp(&sum123, &best_sum) < 0) {
                best_sum = sum123;
            }
        }
    }

    bn_print(&best_sum);
    printf("\n");

    free(factors);
    return 0;
}
