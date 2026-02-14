#include <stdio.h>
#include <math.h>

/*
 * Project Euler 855 - Coins in a Box
 *
 * S(a,b) = (a!^b * b!^a) / ((ab)!)^2
 * Compute S(5,8) in scientific notation with 10 sig digits after decimal.
 *
 * We compute log10(S) = b*log10(a!) + a*log10(b!) - 2*log10((ab)!)
 * using lgamma for log-factorial.
 */

int main(void) {
    int a = 5, b = 8;

    /* log10(n!) = lgamma(n+1) / ln(10) */
    double ln10 = log(10.0);

    double log10_fact_a = lgamma(a + 1) / ln10;
    double log10_fact_b = lgamma(b + 1) / ln10;
    double log10_fact_ab = lgamma(a * b + 1) / ln10;

    double log10_val = b * log10_fact_a + a * log10_fact_b - 2.0 * log10_fact_ab;

    /* log10_val is negative. Write as mantissa * 10^exponent */
    /* val = 10^log10_val */
    /* exponent = floor(log10_val) */
    /* mantissa = 10^(log10_val - exponent) */

    double exponent_d = floor(log10_val);
    int exponent = (int)exponent_d;
    double mantissa = pow(10.0, log10_val - exponent_d);

    /* Adjust if mantissa rounds to 10 */
    if (mantissa >= 10.0 - 1e-15) {
        mantissa /= 10.0;
        exponent++;
    }

    printf("%.10e\n", mantissa * pow(10.0, exponent));
    return 0;
}
