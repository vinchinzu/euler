/*
 * Project Euler Problem 173: Square laminae using up to 1 million tiles.
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    int limit = 1000000;
    long long total = 0;
    int max_k = (int)sqrt(limit / 4.0);

    for (int k = 1; k <= max_k; k++) {
        int max_m = limit / (4 * k) - k;
        if (max_m >= 1)
            total += max_m;
    }

    printf("%lld\n", total);
    return 0;
}
