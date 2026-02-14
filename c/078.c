/* Project Euler 078 - Coin partitions (pentagonal number theorem) */
#include <stdio.h>
#include <stdlib.h>

#define TARGET_DIVISOR 1000000
#define MAX_N 100000

static int partitions[MAX_N + 1];

int main(void) {
    partitions[0] = 1;

    for (int n = 1; n <= MAX_N; n++) {
        int current = 0;
        int k = 1;

        while (1) {
            int pent1 = k * (3 * k - 1) / 2;
            int pent2 = k * (3 * k + 1) / 2;
            int sign = (k % 2 == 1) ? 1 : -1;

            if (n - pent1 >= 0) {
                current += sign * partitions[n - pent1];
            } else {
                break;
            }

            if (n - pent2 >= 0) {
                current += sign * partitions[n - pent2];
            }

            k++;
        }

        partitions[n] = ((current % TARGET_DIVISOR) + TARGET_DIVISOR) % TARGET_DIVISOR;

        if (partitions[n] == 0) {
            printf("%d\n", n);
            return 0;
        }
    }

    return 1;
}
