/* Project Euler 485 - Maximum number of divisors
 * Extracted from python/485_helper.c
 * Uses divisor sieve + sliding window maximum.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define N 100000000
#define K 100000

int main(void) {
    uint16_t *divs = calloc(N + 1, sizeof(uint16_t));
    if (!divs) {
        fprintf(stderr, "Allocation failed for divs\n");
        return 1;
    }

    for (int i = 1; i <= N; i++) {
        for (int j = i; j <= N; j += i) {
            divs[j]++;
        }
    }

    int *deque = malloc((N + 1) * sizeof(int));
    if (!deque) {
        fprintf(stderr, "Allocation failed for deque\n");
        free(divs);
        return 1;
    }

    int head = 0;
    int tail = 0;
    uint64_t sum = 0;

    for (int i = 1; i <= N; i++) {
        while (tail > head && divs[deque[tail - 1]] <= divs[i]) {
            tail--;
        }
        deque[tail++] = i;

        int left = i - K + 1;
        if (left >= 1) {
            while (deque[head] < left) {
                head++;
            }
            sum += divs[deque[head]];
        }
    }

    printf("%llu\n", (unsigned long long)sum);

    free(deque);
    free(divs);
    return 0;
}
