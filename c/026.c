/*
 * Project Euler 026 - Reciprocal Cycles
 * Find d < 1000 for which 1/d has the longest recurring cycle.
 */
#include <stdio.h>

int cycle_length(int d) {
    if (d % 2 == 0 || d % 5 == 0) return 0;

    /* Track remainders and their positions */
    int remainders[1001];
    for (int i = 0; i < 1001; i++) remainders[i] = -1;

    int remainder = 1;
    int position = 0;

    while (remainder != 0) {
        if (remainders[remainder] != -1) {
            return position - remainders[remainder];
        }
        remainders[remainder] = position;
        remainder = (remainder * 10) % d;
        position++;
    }

    return 0;
}

int main(void) {
    const int LIMIT = 1000;
    int max_cycle = 0;
    int best_d = 0;

    for (int d = 1; d < LIMIT; d++) {
        if (d % 2 == 0 || d % 5 == 0) continue;

        int cycle = cycle_length(d);
        if (cycle > max_cycle) {
            max_cycle = cycle;
            best_d = d;
        }
    }

    printf("%d\n", best_d);
    return 0;
}
