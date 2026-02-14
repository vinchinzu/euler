/* Project Euler Problem 101: Optimum Polynomial */
#include <stdio.h>

/* u(n) = 1 - n + n^2 - n^3 + ... + n^10 */
static long long u(int n) {
    long long result = 0;
    long long power = 1; /* (-n)^0 = 1 */
    int neg_n = -n;
    for (int i = 0; i <= 10; i++) {
        result += power;
        power *= neg_n;
    }
    return result;
}

/* Lagrange interpolation: given k points (1,seq[0]), (2,seq[1]), ..., (k,seq[k-1])
   evaluate the interpolating polynomial at x = n_val.
   Use exact integer arithmetic via long long (works here because values stay moderate). */
static long long op(int k, int n_val, long long *seq) {
    /* Use rational arithmetic: accumulate numerator/denominator separately */
    /* Actually, Lagrange interpolation with integer x-values and integer y-values
       evaluated at integer x gives a rational number. For this problem the result is integer.
       We can compute using double for small k (k <= 10) safely. */
    double total = 0.0;
    for (int j = 0; j < k; j++) {
        double num = 1.0, den = 1.0;
        int xj = j + 1;
        for (int i = 0; i < k; i++) {
            if (i == j) continue;
            int xi = i + 1;
            num *= (n_val - xi);
            den *= (xj - xi);
        }
        total += seq[j] * (num / den);
    }
    /* Round to nearest integer */
    return (long long)(total + (total > 0 ? 0.5 : -0.5));
}

int main(void) {
    long long seq[11];
    long long sum_fits = 0;

    for (int k = 1; k <= 10; k++) {
        seq[k - 1] = u(k);
        long long predicted = op(k, k + 1, seq);
        long long actual = u(k + 1);
        if (predicted != actual) {
            sum_fits += predicted;
        }
    }
    printf("%lld\n", sum_fits);
    return 0;
}
