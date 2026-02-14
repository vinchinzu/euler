/* Project Euler Problem 104: Pandigital Fibonacci Ends */
#include <stdio.h>
#include <stdbool.h>
#include <math.h>
#include <string.h>

static bool is_pandigital_1_9(const char *s) {
    if (strlen(s) != 9) return false;
    int cnt[10] = {0};
    for (int i = 0; i < 9; i++) {
        int d = s[i] - '0';
        if (d < 1 || d > 9) return false;
        cnt[d]++;
    }
    for (int d = 1; d <= 9; d++)
        if (cnt[d] != 1) return false;
    return true;
}

int main(void) {
    long long MOD = 1000000000LL;
    long long a_last = 1, b_last = 1;

    /* For first 9 digits, use Binet's formula via log10 */
    /* log10(F_k) ~ k*log10(phi) - log10(sqrt(5)) for large k */
    double log10_phi = log10((1.0 + sqrt(5.0)) / 2.0);
    double log10_sqrt5 = log10(sqrt(5.0));

    int k = 2;
    while (1) {
        k++;
        long long c_last = (a_last + b_last) % MOD;
        a_last = b_last;
        b_last = c_last;

        /* Check last 9 digits */
        char last9[16];
        sprintf(last9, "%09lld", b_last);
        /* last9 may be shorter than 9 if b_last < 10^8, but %09lld pads with zeros.
           However pandigital requires digits 1-9, so leading zeros would fail.
           Actually we need the actual last 9 digits. If the Fibonacci number has
           fewer than 9 digits, the leading positions are 0 which aren't 1-9 pandigital. */
        if (!is_pandigital_1_9(last9)) continue;

        /* Check first 9 digits using log10 approximation (valid for large k) */
        double log10_fk = k * log10_phi - log10_sqrt5;
        double frac = log10_fk - floor(log10_fk);
        double first9d = pow(10.0, frac + 8.0);
        char first9[16];
        sprintf(first9, "%.0f", first9d);
        /* Take first 9 chars */
        first9[9] = '\0';

        if (is_pandigital_1_9(first9)) {
            printf("%d\n", k);
            return 0;
        }
    }
}
