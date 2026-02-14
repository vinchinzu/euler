/* Project Euler 072 - Counting fractions */
#include <stdio.h>

#define LIMIT 1000000

static long long phi[LIMIT + 1];

int main(void) {
    for (int i = 0; i <= LIMIT; i++) {
        phi[i] = i;
    }

    /* Compute phi values using sieve-like approach */
    for (int i = 2; i <= LIMIT; i++) {
        if (phi[i] == i) { /* i is prime */
            for (int j = i; j <= LIMIT; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }

    /* Sum phi values from 2 to LIMIT */
    long long total = 0;
    for (int i = 2; i <= LIMIT; i++) {
        total += phi[i];
    }

    printf("%lld\n", total);
    return 0;
}
