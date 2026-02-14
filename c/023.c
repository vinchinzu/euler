/*
 * Project Euler 023 - Non-abundant Sums
 * Find the sum of all positive integers which cannot be written
 * as the sum of two abundant numbers.
 */
#include <stdio.h>
#include <stdbool.h>

#define LIMIT 28123

int divisor_sums[LIMIT + 1];
int abundants[LIMIT];
int abundant_count = 0;
bool can_be_sum[LIMIT + 1];

int main(void) {
    /* Precompute divisor sums */
    for (int i = 0; i <= LIMIT; i++) divisor_sums[i] = 0;
    for (int i = 1; i <= LIMIT; i++) {
        for (int j = i * 2; j <= LIMIT; j += i) {
            divisor_sums[j] += i;
        }
    }

    /* Find all abundant numbers */
    for (int i = 12; i <= LIMIT; i++) {
        if (divisor_sums[i] > i) {
            abundants[abundant_count++] = i;
        }
    }

    /* Mark all sums of two abundant numbers */
    for (int i = 0; i <= LIMIT; i++) can_be_sum[i] = false;
    for (int i = 0; i < abundant_count; i++) {
        for (int j = i; j < abundant_count; j++) {
            int sum = abundants[i] + abundants[j];
            if (sum <= LIMIT) {
                can_be_sum[sum] = true;
            } else {
                break;
            }
        }
    }

    /* Sum all numbers that cannot be expressed as sum of two abundants */
    long long total = 0;
    for (int i = 1; i <= LIMIT; i++) {
        if (!can_be_sum[i]) {
            total += i;
        }
    }

    printf("%lld\n", total);
    return 0;
}
