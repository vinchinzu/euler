/*
 * Project Euler 281 - Pizza Toppings
 *
 * f(m,n) = number of ways to put m toppings on m*n slices (each topping on
 * exactly n slices), up to rotation. Uses Burnside's lemma:
 * f(m,n) = (1/(m*n)) * sum_{k: gcd(k,m*n) divisible by m} multinomial(g, [g/m]*m)
 * where g = gcd(k, m*n).
 *
 * Sum all f(m,n) <= 10^15 for m>=2, n>=1.
 *
 * Uses __int128 to handle large intermediate values without overflow.
 * Computes multinomial(g, [count_per]*m) = g! / (count_per!)^m carefully
 * by doing the division step-by-step to keep numbers manageable.
 */
#include <stdio.h>

typedef unsigned long long ull;
typedef __int128 i128;
typedef unsigned __int128 u128;

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* Compute multinomial(g; [cp]*m) = g! / (cp!)^m
 * To avoid overflow, compute iteratively:
 * result = product_{i=1}^{g} i, divided by (cp!)^m
 * We interleave multiplication and division.
 */
static u128 multinomial(int g, int cp, int m) {
    if (cp == 0) return 1; /* 0! = 1, so denominator = 1 */

    /* Build list of denominator factors: cp! repeated m times */
    /* Total denom factors = m * cp values */
    /* We'll divide as we go to keep numbers small */

    /* Compute (cp!)^m as a list of prime factors, then divide during multiplication */
    /* Simpler: just compute g! / (cp!)^m step by step */

    /* Approach: compute result = 1, multiply by i for i=1..g, and divide by
     * denominator factors as we go. The denominator is (cp!)^m.
     * Flatten denominator: [1,2,...,cp, 1,2,...,cp, ...] (m copies) */

    /* Total denominator factors: m*cp values from 1 to cp, each appearing m times */
    /* Total numerator factors: g values from 1 to g */
    /* Since g = cp * m, we have g = m*cp numerator factors */

    u128 result = 1;
    int denom_idx = 0; /* index into flattened denominator */
    int denom_copy = 0; /* which copy of cp! we're in */
    int denom_val = 1;  /* current value in the copy */

    for (int i = 1; i <= g; i++) {
        result *= (u128)i;

        /* Divide by as many denominator factors as possible */
        while (denom_copy < m) {
            if (result % (u128)denom_val == 0) {
                result /= (u128)denom_val;
                denom_val++;
                if (denom_val > cp) {
                    denom_val = 1;
                    denom_copy++;
                }
            } else {
                break;
            }
        }
    }

    /* Divide by remaining denominator factors */
    while (denom_copy < m) {
        result /= (u128)denom_val;
        denom_val++;
        if (denom_val > cp) {
            denom_val = 1;
            denom_copy++;
        }
    }

    return result;
}

static u128 f_func(int m, int n) {
    int mn = m * n;
    u128 total = 0;
    for (int k = 0; k < mn; k++) {
        int g = gcd(k, mn);
        if (g % m == 0) {
            int cp = g / m;
            total += multinomial(g, cp, m);
        }
    }
    return total / (u128)mn;
}

int main(void) {
    ull N = 1000000000000000ULL; /* 10^15 */

    ull ans = 0;

    for (int m = 2; ; m++) {
        u128 f1 = f_func(m, 1);
        if (f1 > (u128)N)
            break;
        for (int n = 1; ; n++) {
            u128 fv = f_func(m, n);
            if (fv > (u128)N)
                break;
            ans += (ull)fv;
        }
    }

    printf("%llu\n", ans);
    return 0;
}
