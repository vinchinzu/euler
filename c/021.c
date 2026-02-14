/*
 * Project Euler 021 - Amicable Numbers
 * Find the sum of all amicable numbers under 10000.
 */
#include <stdio.h>

int main(void) {
    const int LIMIT = 10000;
    const int EXTENDED_LIMIT = 20000;

    int divisor_sums[20001] = {0};

    /* Precompute divisor sums using sieve method */
    for (int i = 1; i <= EXTENDED_LIMIT; i++) {
        for (int j = 2 * i; j <= EXTENDED_LIMIT; j += i) {
            divisor_sums[j] += i;
        }
    }

    int sum = 0;
    for (int a = 1; a < LIMIT; a++) {
        int b = divisor_sums[a];
        if (b < LIMIT && b != a && divisor_sums[b] == a) {
            sum += a;
        }
    }

    printf("%d\n", sum);
    return 0;
}
