/*
 * Project Euler Problem 505: Bidirectional Recurrence.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdint.h>

#define N 1000000000000LL
#define K ((1ULL << 60) - 1)

uint64_t helper(uint64_t k, uint64_t prev_x, uint64_t x, uint64_t alpha, uint64_t beta) {
    if (k >= N)
        return x;
    uint64_t y = helper(2 * k, x, (2 * prev_x + 3 * x) & K, K - beta, K - alpha);
    if (K - y <= alpha)
        return alpha;
    uint64_t y2 = helper(2 * k + 1, x, (3 * prev_x + 2 * x) & K, y, K - alpha);
    return K - (y > y2 ? y : y2);
}

int main() {
    uint64_t result = helper(1, 0, 1, 0, K);
    printf("%llu\n", (unsigned long long)result);
    return 0;
}
