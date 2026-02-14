/*
 * Project Euler 848 - Guessing Game
 *
 * Compute sum_{i=0}^{20} sum_{j=0}^{20} p(7^i, 5^j)
 * where p(m,n) is the winning probability of first player.
 *
 * Uses rational arithmetic with __int128 for precision.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef __int128 i128;

static i128 abs128(i128 x) { return x < 0 ? -x : x; }

static i128 gcd128(i128 a, i128 b) {
    a = abs128(a); b = abs128(b);
    while (b) { i128 t = b; b = a % b; a = t; }
    return a;
}

typedef struct { i128 num; i128 den; } frac;

static frac frac_new(i128 n, i128 d) {
    if (d < 0) { n = -n; d = -d; }
    i128 g = gcd128(n, d);
    if (g > 1) { n /= g; d /= g; }
    frac f; f.num = n; f.den = d;
    return f;
}

static frac frac_add(frac a, frac b) {
    return frac_new(a.num * b.den + b.num * a.den, a.den * b.den);
}

static frac frac_sub(frac a, frac b) {
    return frac_new(a.num * b.den - b.num * a.den, a.den * b.den);
}

static frac frac_mul(frac a, frac b) {
    return frac_new(a.num * b.num, a.den * b.den);
}

static frac frac_div(frac a, frac b) {
    return frac_new(a.num * b.den, a.den * b.num);
}

/* Get S_n = n * D_n */
static i128 get_Sn(i128 n) {
    if (n <= 0) return 0;
    if (n == 1) return 0;
    if (n == 2) return 1;
    if (n == 3) return 3;

    /* j = floor(log2((n-1)/3)) */
    i128 val = (n - 1) / 3;
    int j = 0;
    i128 tmp = val;
    while (tmp > 1) { j++; tmp >>= 1; }

    /* term1 = 3 * 4^j */
    i128 pow4j = (i128)1 << (2 * j);
    i128 term1 = 3 * pow4j;

    /* start_of_range = 3 * 2^j */
    i128 pow2j = (i128)1 << j;
    i128 start_of_range = 3 * pow2j;
    i128 diff = 3 * pow2j;

    i128 term2 = (n - start_of_range) * diff;

    return term1 + term2;
}

/* Get T_n = transition threshold */
static i128 get_Tn(i128 n) {
    if (n == 1) return 1;
    if (n == 2) return 2;
    if (n == 3) return 3;

    i128 val = (n - 1) / 3;
    int j = 0;
    i128 tmp = val;
    while (tmp > 1) { j++; tmp >>= 1; }

    return 3 * ((i128)1 << (j + 1));
}

static void print_double(double v, int decimals) {
    /* Print with exactly 'decimals' digits after decimal point */
    printf("%.*f\n", decimals, v);
}

int main(void) {
    /* Precompute powers of 7 and 5 */
    i128 pow7[21], pow5[21];
    pow7[0] = 1; pow5[0] = 1;
    for (int i = 1; i <= 20; i++) {
        pow7[i] = pow7[i-1] * 7;
        pow5[i] = pow5[i-1] * 5;
    }

    /* We need to sum fractions. To avoid overflow with __int128 for very large
     * denominators, we'll use double precision. The problem asks for 8 decimal
     * places, so double (15+ significant digits) should be sufficient.
     *
     * But let's try to keep fractions where possible and convert to double at end.
     * Actually, m can be 7^20 ~ 7.98e16, n can be 5^20 ~ 9.54e13.
     * m*n can be ~7.6e30 which fits in __int128 (max ~1.7e38).
     * S_m can be ~ 3 * (2^j)^2 where 2^j ~ m/3, so S_m ~ m^2/3 ~ 6.4e33.
     * S_m * 1 fits in __int128. But S_m / (m * n) as a fraction... the numerator
     * of the running sum could overflow. Let's use long double accumulation.
     */

    long double total_sum = 0.0L;

    for (int i = 0; i <= 20; i++) {
        i128 m = pow7[i];
        i128 S_m = get_Sn(m);

        for (int j = 0; j <= 20; j++) {
            i128 n = pow5[j];
            i128 Tn = get_Tn(n);

            long double term;

            if (m <= Tn) {
                /* Regime 2: p(m,n) = 1 - S_m / (m * n) */
                long double ratio = (long double)S_m / ((long double)m * (long double)n);
                term = 1.0L - ratio;
            } else {
                /* Regime 1: p(m,n) = C_n / m */
                long double Cn;
                if (n == 1) {
                    Cn = 1.0L;
                } else if (n == 2) {
                    Cn = 1.5L;
                } else {
                    i128 S_n = get_Sn(n);
                    Cn = 2.0L * (long double)S_n / (long double)n;
                }
                term = Cn / (long double)m;
            }

            total_sum += term;
        }
    }

    printf("%.8Lf\n", total_sum);
    return 0;
}
