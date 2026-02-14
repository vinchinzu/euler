/*
 * Project Euler Problem 174: Counting "hollow" square laminae.
 *
 * Count t values <= 1000000 that have 1..10 representations as square laminae.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 1000000
#define MAX_N 10

int main(void) {
    /* counts[t] = number of representations */
    /* t = 4*m*(k-m), k >= 2m+1, t <= LIMIT */
    /* Max t is LIMIT, so we need an array of size LIMIT+1 */
    int *counts = calloc(LIMIT + 1, sizeof(int));

    int max_m = (int)sqrt(LIMIT / 4.0) + 1;

    for (int m = 1; m <= max_m; m++) {
        int min_k = 2 * m + 1;
        int max_k_val = m + LIMIT / (4 * m);

        for (int k = min_k; k <= max_k_val; k++) {
            long long t = 4LL * m * (k - m);
            if (t > LIMIT) break;
            counts[(int)t]++;
        }
    }

    int result = 0;
    for (int t = 1; t <= LIMIT; t++) {
        if (counts[t] >= 1 && counts[t] <= MAX_N)
            result++;
    }

    printf("%d\n", result);
    free(counts);
    return 0;
}
