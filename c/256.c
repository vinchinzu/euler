/*
 * Project Euler Problem 256: Tatami-Free Rooms
 *
 * Find the smallest area s such that exactly 200 rooms of area s are
 * tatami-free.
 *
 * For room a*b (a<=b, a>2), it is tatami-free iff there is no integer
 * in the interval ((b-1)/(a+1), (b+1)/(a-1)).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define TARGET 200
#define LIMIT 100000000  /* 10^8 */

/* Use short to save memory - max count should be small */
static short counts[LIMIT + 1];

int main(void) {
    memset(counts, 0, sizeof(counts));

    int sq = (int)sqrt((double)LIMIT);

    for (int a = 3; a <= sq; a++) {
        int max_b = LIMIT / a;
        int max_k = (a - 5) / 2;

        for (int k = 0; k <= max_k; k++) {
            int lo = k * (a + 1) + 2;
            int hi = (k + 1) * (a - 1) - 2;
            if (lo < a) lo = a;
            if (hi > max_b) hi = max_b;
            if (lo > hi) continue;

            /* Increment counts for s = a*lo, a*(lo+1), ..., a*hi */
            for (int b = lo; b <= hi; b++) {
                int s = a * b;
                if (s <= LIMIT)
                    counts[s]++;
            }
        }
    }

    /* Find smallest s with exactly TARGET tatami-free pairs */
    for (int s = 1; s <= LIMIT; s++) {
        if (counts[s] == TARGET) {
            printf("%d\n", s);
            return 0;
        }
    }

    printf("0\n");
    return 0;
}
