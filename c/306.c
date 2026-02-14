/*
 * Project Euler Problem 306: Paper-strip Game
 *
 * Count winning positions (Grundy != 0) for n in [1, 10^6].
 * Compute Grundy numbers, find period, extrapolate.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INIT_COMPUTE 5000
#define LIMIT 1000000

static int G[INIT_COMPUTE + 1];

/* Compute Grundy numbers up to limit */
static void compute_grundy(int limit) {
    /* G[0] = G[1] = 0 (no moves) */
    /* Need a temporary seen array for mex. Max Grundy value is small. */
    int max_seen = 256;
    char *seen = calloc(max_seen, 1);

    for (int n = 2; n <= limit; n++) {
        memset(seen, 0, max_seen);
        for (int i = 0; i < n - 1; i++) {
            int left = i;
            int right = n - i - 2;
            int xv = G[left] ^ G[right];
            if (xv < max_seen) seen[xv] = 1;
        }
        int mex = 0;
        while (mex < max_seen && seen[mex]) mex++;
        G[n] = mex;
    }
    free(seen);
}

int main(void) {
    compute_grundy(INIT_COMPUTE);

    /* Find period: try periods starting from various offsets */
    int start = -1, period = -1;
    for (int p = 1; p < INIT_COMPUTE / 3; p++) {
        for (int s = 0; s < INIT_COMPUTE / 3; s++) {
            int check_len = 100;
            if (s + p + check_len > INIT_COMPUTE) continue;
            int valid = 1;
            for (int j = 0; j < check_len; j++) {
                if (G[s + j] != G[s + p + j]) { valid = 0; break; }
            }
            if (valid) {
                start = s;
                period = p;
                goto found;
            }
        }
    }
found:
    if (start < 0) {
        /* No period found - shouldn't happen for this problem */
        return 1;
    }

    int count = 0;

    /* Count before periodic part */
    for (int n = 1; n < start; n++) {
        if (G[n] != 0) count++;
    }

    /* Count nonzeros in one period */
    int nonzeros = 0;
    for (int i = 0; i < period; i++) {
        if (G[start + i] != 0) nonzeros++;
    }

    /* Full periods in [start, LIMIT] */
    int remaining = LIMIT - start + 1;
    int full = remaining / period;
    int leftover = remaining % period;

    count += full * nonzeros;

    for (int i = 0; i < leftover; i++) {
        if (G[start + i] != 0) count++;
    }

    printf("%d\n", count);
    return 0;
}
