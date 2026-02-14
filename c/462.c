/*
 * Project Euler Problem 462: Permutation of 3-smooth numbers
 *
 * Count arrangements of 3-smooth numbers up to N=10^18 such that
 * each element comes after all proper divisors. Uses hook length
 * formula on the Young diagram of exponents.
 *
 * Output format: mantissa with 10 decimal digits followed by "e" and exponent.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    long long N = 1000000000000000000LL; /* 10^18 */

    /* Generate 3-smooth numbers as (e2, e3) exponent pairs */
    int e2_arr[10000], e3_arr[10000];
    int npoints = 0;

    long long pow2 = 1;
    for (int e2 = 0; e2 < 100; e2++) {
        if (pow2 > N) break;
        long long pow3 = 1;
        for (int e3 = 0; e3 < 100; e3++) {
            if (pow2 > N / (pow3 > 0 ? pow3 : 1) || pow2 * pow3 > N) {
                /* Overflow-safe check */
                if (pow3 > N / pow2) break;
            }
            if (pow2 * pow3 > N) break;
            e2_arr[npoints] = e2;
            e3_arr[npoints] = e3;
            npoints++;
            if (pow3 > N / 3) break;
            pow3 *= 3;
        }
        if (pow2 > N / 2) break;
        pow2 *= 2;
    }

    /* ans = log10(n! / prod(hook_lengths)) */
    double ans = 0.0;

    /* Add log(n!) */
    for (int i = 1; i <= npoints; i++) {
        ans += log10((double)i);
    }

    /* Subtract log(hook_length) for each cell */
    for (int p = 0; p < npoints; p++) {
        int hook = 0;
        for (int q = 0; q < npoints; q++) {
            if ((e2_arr[q] == e2_arr[p] && e3_arr[q] >= e3_arr[p]) ||
                (e3_arr[q] == e3_arr[p] && e2_arr[q] >= e2_arr[p])) {
                hook++;
            }
        }
        ans -= log10((double)hook);
    }

    int exp_part = (int)ans;
    double mantissa = pow(10.0, ans - exp_part);

    /* Adjust if mantissa < 1 */
    if (mantissa < 1.0) {
        mantissa *= 10.0;
        exp_part--;
    }

    printf("%.10fe%d\n", mantissa, exp_part);
    return 0;
}
