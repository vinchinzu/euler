/*
 * Project Euler 777 - Lissajous Curves
 *
 * Sum of x^2+y^2 at self-crossings of Lissajous curves C_{a,b} summed over
 * coprime pairs 2<=a,b<=N. Uses Mobius inversion with correction for 10|ab case.
 *
 * Works with __int128 to avoid overflow in intermediate products.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef __int128 i128;

#define N_LIMIT 1000000

static int *mobius;

static void pre_mobius(int limit) {
    mobius = (int *)calloc(limit + 1, sizeof(int));
    char *is_prime = (char *)calloc(limit + 1, 1);
    for (int i = 0; i <= limit; i++) {
        mobius[i] = 1;
        is_prime[i] = 1;
    }
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((j / i) % i == 0)
                    mobius[j] = 0;
                else
                    mobius[j] = -mobius[j];
            }
        }
    }
    free(is_prime);
}

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* Number of divisors */
static int num_divisors(int n) {
    int count = 0;
    for (int i = 1; (long long)i * i <= n; i++) {
        if (n % i == 0) {
            count += 2;
            if (i * i == n) count--;
        }
    }
    return count;
}

/* Get all divisors of n */
static int get_divisors(int n, int *divs) {
    int count = 0;
    for (int i = 1; (long long)i * i <= n; i++) {
        if (n % i == 0) {
            divs[count++] = i;
            if (i != n / i)
                divs[count++] = n / i;
        }
    }
    return count;
}

int main(void) {
    int N = N_LIMIT;
    pre_mobius(N);

    /* Precompute num_divisors for divisors of 10 */
    int nd[11];
    for (int d = 1; d <= 10; d++)
        nd[d] = num_divisors(d);

    /* Accumulate 4*ans as i128 to keep everything integral */
    i128 ans_times_4 = 0;

    for (int g = 1; g <= N; g++) {
        if (mobius[g] == 0) continue;
        int n = N / g;
        if (n < 1) continue;

        int t = gcd(10, g);
        i128 trn = (i128)n * (n + 1) / 2;

        /* Main formula multiplied by 4: 8*g^2*trn^2 - 12*n*g*trn */
        i128 g2 = (i128)g * g;
        i128 res4 = 8 * g2 * trn * trn - 12 * (i128)n * g * trn;

        /* Correction for 10|ab case */
        int val_10_t = 10 / t;
        int divs[16];
        int ndivs = get_divisors(val_10_t, divs);
        int nd_t = nd[t];

        for (int di = 0; di < ndivs; di++) {
            int d = divs[di];
            int e = val_10_t / d;
            i128 tnd = (i128)(n / d) * (n / d + 1) / 2;
            i128 tne = (i128)(n / e) * (n / e + 1) / 2;
            i128 nd_val = n / d;
            i128 ne_val = n / e;

            res4 += nd_t * (
                -6 * g2 * d * tnd * e * tne
                + 3 * nd_val * g * e * tne
                + 3 * ne_val * g * d * tnd
                + 4 * nd_val * ne_val
            );
        }

        ans_times_4 += mobius[g] * res4;
    }

    /* ans = ans_times_4 / 4 */
    /* The result should be a large number in scientific notation */
    /* Convert i128 to double for output */
    double ans = (double)ans_times_4 / 4.0;

    /* Format: expected 2.533018434e23 */
    char buf[100];
    sprintf(buf, "%.9e", ans);
    /* Remove '+' from exponent */
    char out[100];
    int j = 0;
    for (int i = 0; buf[i]; i++) {
        if (buf[i] == '+') continue;
        out[j++] = buf[i];
    }
    out[j] = '\0';
    printf("%s\n", out);

    free(mobius);
    return 0;
}
