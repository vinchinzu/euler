/*
 * Project Euler 686 - Powers of Two
 *
 * Find the Nth positive integer j such that 2^j starts with "123".
 * Uses log10: fractional part of j*log10(2) determines leading digits.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 678910;
    double log2 = log10(2.0);
    double lo = log10(1.23);
    double hi = log10(1.24);
    int count = 0;
    long long j = 0;
    while (count < N) {
        j++;
        double val = j * log2;
        double frac = val - (long long)val;
        if (frac >= lo && frac < hi) {
            count++;
        }
    }
    printf("%lld\n", j);
    return 0;
}
