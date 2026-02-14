/*
 * Project Euler Problem 418: Factorisation triples.
 *
 * Find the minimum possible value of a+b+c if a*b*c = 43!.
 *
 * Enumerate factors of 43! near the cube root, then brute force over all
 * pairs to find minimum sum. Uses big-number arithmetic for exact comparison.
 *
 * Ported from python/418.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

/* Primes up to 43 */
static const int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43};
#define NPRIMES 14

/* Exponents of each prime in 43! */
static int fact_exp[NPRIMES];

static int num_factors_in_factorial(int n, int p) {
    int count = 0;
    long long power = p;
    while (power <= n) {
        count += n / (int)power;
        power *= p;
    }
    return count;
}

/* Big number as array of limbs (base 10^9) */
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

static void bn_mul_int(BigNum *b, ull x) {
    ull carry = 0;
    for (int i = 0; i < b->nlimbs; i++) {
        __uint128_t v = (__uint128_t)b->limbs[i] * x + carry;
        b->limbs[i] = (ull)(v % LIMB_BASE);
        carry = (ull)(v / LIMB_BASE);
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
    /* Zero out remaining */
    for (int i = n; i < MAXLIMBS; i++) res->limbs[i] = 0;
}

static void bn_from_exponents(BigNum *b, const int exp[]) {
    bn_from_one(b);
    for (int i = 0; i < NPRIMES; i++) {
        for (int j = 0; j < exp[i]; j++) {
            bn_mul_int(b, (ull)primes[i]);
        }
    }
}

/* Check divisibility: is 43!/a divisible by b?
   Equivalently, for all primes, fact_exp[i] - a_exp[i] >= b_exp[i] */
static int divides(const int a_exp[], const int b_exp[]) {
    for (int i = 0; i < NPRIMES; i++)
        if (fact_exp[i] - a_exp[i] < b_exp[i]) return 0;
    return 1;
}

static void bn_print(const BigNum *b) {
    printf("%llu", b->limbs[b->nlimbs - 1]);
    for (int i = b->nlimbs - 2; i >= 0; i--)
        printf("%09llu", b->limbs[i]);
}

/* Factor storage */
typedef struct {
    int exp[NPRIMES];
} Factor;

static Factor *factors;
static int nfactors;
static int factors_cap;

static double log_primes[NPRIMES];
static double L_log;  /* log(cube_root(43!)) */
static double R;

/*
 * Recursively enumerate factors in range [L*R, L/R] (using log-space).
 * This mirrors the Python helper() function exactly:
 *   - Only add factor if n > L*R  (logval > L_log + log(R))
 *   - Prune if n * p > L/R  (logval + log(p) > L_log - log(R))
 */
static void enumerate(int idx, int cur_exp[], double logv, double lo, double hi) {
    if (logv > hi) return;
    if (idx == NPRIMES) {
        if (logv >= lo) {
            if (nfactors >= factors_cap) {
                factors_cap = factors_cap * 2;
                factors = (Factor *)realloc(factors, factors_cap * sizeof(Factor));
            }
            memcpy(factors[nfactors].exp, cur_exp, NPRIMES * sizeof(int));
            nfactors++;
        }
        return;
    }

    for (int e = 0; e <= fact_exp[idx]; e++) {
        cur_exp[idx] = e;
        enumerate(idx + 1, cur_exp, logv + e * log_primes[idx], lo, hi);
    }
    cur_exp[idx] = 0;
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
    L_log = total_log / 3.0;

    /* Match Python: R = 0.99999, range [L*R, L/R] in log space */
    R = 0.99999;
    double lo = L_log + log(R);    /* log(L*R) */
    double hi = L_log - log(R);    /* log(L/R) */

    factors_cap = 100000;
    factors = (Factor *)malloc(factors_cap * sizeof(Factor));
    nfactors = 0;

    int cur_exp[NPRIMES];
    memset(cur_exp, 0, sizeof(cur_exp));
    enumerate(0, cur_exp, 0.0, lo, hi);

    fprintf(stderr, "Found %d factors near cube root\n", nfactors);

    /* Find minimum sum: iterate over all pairs (f1, f2) from factors,
       check if 43!/(f1*f2) is a valid factor (non-negative exponents),
       compute exact sum and track minimum. */
    BigNum best_sum;
    memset(&best_sum, 0, sizeof(best_sum));
    best_sum.nlimbs = MAXLIMBS;
    for (int i = 0; i < MAXLIMBS; i++) best_sum.limbs[i] = LIMB_BASE - 1;

    for (int i = 0; i < nfactors; i++) {
        for (int j = 0; j < nfactors; j++) {
            /* Check f1 * f2 divides 43!, i.e., for each prime,
               f1_exp + f2_exp <= fact_exp */
            if (!divides(factors[i].exp, factors[j].exp)) continue;

            /* f3 = 43! / (f1 * f2) */
            int f3_exp[NPRIMES];
            for (int k = 0; k < NPRIMES; k++)
                f3_exp[k] = fact_exp[k] - factors[i].exp[k] - factors[j].exp[k];

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
