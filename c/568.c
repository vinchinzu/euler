/*
 * Project Euler Problem 568: Reciprocal Games II
 *
 * D(n) = H_n / 2^n. We need 7 most significant digits of D(123456789).
 * log10(D(n)) = log10(H_n) - n*log10(2).
 * H_n ~ ln(n) + gamma + 1/(2n) - 1/(12n^2) + ... (Euler-Maclaurin)
 * So log10(H_n) = log10(ln(n) + gamma + ...).
 * Then extract the fractional part and get 7 significant digits.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    long long N = 123456789LL;
    double n = (double)N;

    /* Euler-Mascheroni constant */
    double gamma = 0.5772156649015328606065120900824024310421;

    /* H_n via Euler-Maclaurin: ln(n) + gamma + 1/(2n) - 1/(12n^2) + 1/(120n^4) - ... */
    double inv_n = 1.0 / n;
    double inv_n2 = inv_n * inv_n;
    double H_n = log(n) + gamma
                 + inv_n / 2.0
                 - inv_n2 / 12.0
                 + inv_n2 * inv_n2 / 120.0
                 - inv_n2 * inv_n2 * inv_n2 / 252.0
                 + inv_n2 * inv_n2 * inv_n2 * inv_n2 / 240.0;

    /* log10(D(N)) = log10(H_N) - N * log10(2) */
    double log10_H = log10(H_n);
    double log10_D = log10_H - (double)N * log10(2.0);

    /* Extract fractional part */
    double frac = log10_D - floor(log10_D);
    double significant = pow(10.0, frac);

    /* Get 7 digits */
    long long digits = (long long)(significant * 1000000.0);
    printf("%lld\n", digits);
    return 0;
}
