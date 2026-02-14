/* Project Euler Problem 114: Counting Block Combinations I */
#include <stdio.h>

static long long memo[60];
static int memo_set[60];

static long long ways(int n) {
    if (n < -1) return 0;
    if (n == -1) return 1;
    if (n == 0) return 1;
    if (memo_set[n]) return memo[n];

    long long total = ways(n - 1);
    for (int k = 3; k <= n; k++) {
        total += ways(n - k - 1);
    }
    memo[n] = total;
    memo_set[n] = 1;
    return total;
}

int main(void) {
    for (int i = 0; i < 60; i++) memo_set[i] = 0;
    printf("%lld\n", ways(50));
    return 0;
}
