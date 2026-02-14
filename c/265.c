/*
 * Project Euler 265: Binary Circles
 *
 * Find all circular arrangements of 2^N bits such that all clockwise
 * subsequences of length N are unique. Sum when interpreted as binary
 * number starting from the all-zeros subsequence. N=5.
 */
#include <stdio.h>
#include <stdint.h>

#define N 5
#define SIZE (1 << N)  /* 32 */
#define MASK ((1 << (N-1)) - 1)  /* 0xF */

static int64_t ans = 0;
static int used[SIZE]; /* tracks which N-bit subsequences are used */

static void helper(int index, int64_t seq) {
    if (index == SIZE - 1) {
        ans += seq >> (N - 1);
        return;
    }

    for (int bit = 0; bit < 2; bit++) {
        int subseq = (int)((seq & MASK) * 2 + bit);
        if (!used[subseq]) {
            used[subseq] = 1;
            helper(index + 1, seq * 2 + bit);
            used[subseq] = 0;
        }
    }
}

int main(void) {
    /* Mark subsequence 0 as used (starting point) */
    used[0] = 1;
    helper(0, 0);
    printf("%lld\n", (long long)ans);
    return 0;
}
