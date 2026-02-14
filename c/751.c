/* Project Euler Problem 751: Concatenation Coincidence.
 * Translated from python/751.py
 *
 * Binary search on theta using high-precision arithmetic.
 * We use a simple custom decimal type with 60-digit precision.
 * Since the answer has 24 decimal digits, we use 128-bit fixed-point.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/*
 * We use strings of digits with a simple big-decimal approach.
 * Since we only need ~30 digits of precision, we can use
 * a pair of long longs to represent a 30-digit number.
 *
 * Actually, the simplest approach: use the __float128 type if available,
 * or implement with careful integer arithmetic.
 *
 * We'll use a fixed-point representation with 30 decimal digits after
 * the decimal point. Represent theta as an integer T such that
 * theta = T / 10^30.
 */

/* Use __int128 for arithmetic */
typedef __int128 bigint;
typedef unsigned __int128 ubigint;

/* We represent numbers as bigint with 30 decimal places of precision. */
/* theta is between 2 and 3, so we need at most 1 digit before decimal. */
/* T = theta * SCALE, where SCALE = 10^30. */

#define PREC 30

static bigint SCALE;
static bigint TEN;

static void init_scale(void) {
    SCALE = 1;
    TEN = 10;
    for (int i = 0; i < PREC; i++)
        SCALE *= 10;
}

/* Divide bigints: a / b */
static bigint bigdiv(bigint a, bigint b) {
    return a / b;
}

/* Format: given T = theta * SCALE, produce string "D.DDDDDDDDDDDDDDDDDDDDDDD" */
static void format_result(bigint T, char *buf, int decimal_digits) {
    /* Integer part */
    bigint int_part = T / SCALE;
    bigint frac_part = T % SCALE;
    if (frac_part < 0) { frac_part += SCALE; int_part--; }

    /* Convert int_part to string */
    int ip = (int)int_part;
    int pos = 0;
    buf[pos++] = '0' + ip;
    buf[pos++] = '.';

    /* Convert frac_part to string with PREC digits */
    char frac_str[PREC + 1];
    bigint tmp = frac_part;
    for (int i = PREC - 1; i >= 0; i--) {
        frac_str[i] = '0' + (int)(tmp % 10);
        tmp /= 10;
    }
    frac_str[PREC] = '\0';

    for (int i = 0; i < decimal_digits && i < PREC; i++)
        buf[pos++] = frac_str[i];
    buf[pos] = '\0';
}

int main() {
    init_scale();

    int K = 24;
    bigint low = 2 * SCALE;  /* 2.0 */
    bigint high = 3 * SCALE; /* 3.0 */

    char prev_ans[64] = "";
    char ans[64] = "";

    for (int iter = 0; iter < 200; iter++) {
        bigint theta = (low + high) / 2;

        /* Generate sequence */
        bigint b = theta; /* b is theta * SCALE */
        char digits[128];
        int dlen = 0;

        while (dlen < K) {
            /* int_b = floor(b / SCALE) */
            bigint int_b = b / SCALE;
            if (b < 0 && b % SCALE != 0) int_b--;

            /* frac = b - int_b * SCALE (this is the fractional part * SCALE) */
            bigint frac = b - int_b * SCALE;

            /* b = int_b * (frac + SCALE) / SCALE
             *   = int_b * (frac/SCALE + 1)
             * But we need to keep precision:
             * new_b = int_b * (frac + SCALE) -- but this overflows!
             *
             * int_b is at most ~10, frac+SCALE is at most 2*SCALE ~ 2*10^30
             * product is at most ~2*10^31, which fits in 128 bits.
             * But then we don't divide by SCALE yet, because we need to
             * compute floor(new_b / SCALE) for the next iteration.
             *
             * Actually new_b (as a number) = int_b * (fractional + 1)
             * As fixed-point: new_b_fixed = int_b * (frac + SCALE)
             * But this gives new_b in units of SCALE, while our b is in units of SCALE.
             * So: new_b_fixed = int_b * frac + int_b * SCALE
             *                 = int_b * frac/SCALE * SCALE + int_b * SCALE
             * Hmm, let me think more carefully.
             *
             * b represents b_real = b / SCALE
             * int_b = floor(b_real)
             * frac_real = b_real - int_b = frac / SCALE
             * new_b_real = int_b * (frac_real + 1) = int_b * (frac/SCALE + 1)
             *            = int_b * frac / SCALE + int_b
             * new_b (fixed) = new_b_real * SCALE = int_b * frac + int_b * SCALE
             */
            b = int_b * frac + int_b * SCALE;

            /* floor(new_b_real) = b / SCALE */
            bigint new_int_b = b / SCALE;
            if (b < 0 && b % SCALE != 0) new_int_b--;

            /* Convert new_int_b to digits */
            char num_str[20];
            int nlen = 0;
            bigint tmp = new_int_b;
            if (tmp == 0) {
                num_str[nlen++] = '0';
            } else {
                while (tmp > 0) {
                    num_str[nlen++] = '0' + (int)(tmp % 10);
                    tmp /= 10;
                }
            }
            /* Reverse */
            for (int i = nlen - 1; i >= 0 && dlen < K; i--)
                digits[dlen++] = num_str[i];
        }
        digits[K] = '\0';

        /* Build tau string */
        bigint int_theta = theta / SCALE;
        char tau[64];
        int pos = 0;
        tau[pos++] = '0' + (int)int_theta;
        tau[pos++] = '.';
        for (int i = 0; i < K; i++)
            tau[pos++] = digits[i];
        tau[pos] = '\0';

        /* Parse tau back to fixed-point */
        bigint tau_val = (bigint)(tau[0] - '0') * SCALE;
        bigint frac_val = 0;
        for (int i = 0; i < K && i < PREC; i++) {
            frac_val = frac_val * 10 + (tau[2 + i] - '0');
        }
        /* Pad remaining digits */
        for (int i = K; i < PREC; i++)
            frac_val *= 10;
        tau_val += frac_val;

        if (tau_val > theta)
            low = theta;
        else
            high = theta;

        strcpy(prev_ans, ans);
        strcpy(ans, tau);
        if (strcmp(prev_ans, ans) == 0)
            break;
    }

    printf("%s\n", ans);
    return 0;
}
