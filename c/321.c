/*
 * Project Euler Problem 321 - Swapping Counters
 *
 * m(n) = n*(n+2) = minimum moves to swap n red and n blue counters.
 * Find sum of first 40 values of n where m(n) is a triangular number.
 * Uses recurrence: a[i] = 6*a[i-2] - a[i-4] + 4
 */
#include <stdio.h>

int main(void) {
    long long seq[40];
    seq[0] = 1;
    seq[1] = 3;
    seq[2] = 10;
    seq[3] = 22;

    for (int i = 4; i < 40; i++) {
        seq[i] = 6 * seq[i - 2] - seq[i - 4] + 4;
    }

    long long total = 0;
    for (int i = 0; i < 40; i++) {
        total += seq[i];
    }

    printf("%lld\n", total);
    return 0;
}
