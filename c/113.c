/* Project Euler Problem 113: Non-bouncy Numbers Below 10^100 */
#include <stdio.h>

/* Need big integer arithmetic since C(110,10) is huge.
   Use __int128 which can hold up to ~1.7 * 10^38.
   C(110,10) ~ 5 * 10^13, C(109,9) ~ 5 * 10^12, so the result fits in long long actually.
   Let's verify: C(110,10) = 110!/(10! * 100!)
   Actually for N=100 the answer is ~5.1 * 10^13 which fits in long long. */

static long long comb(int n, int k) {
    if (k > n - k) k = n - k;
    long long result = 1;
    for (int i = 0; i < k; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

int main(void) {
    int N = 100;
    long long term1 = comb(N + 10, 10);
    long long term2 = comb(N + 9, 9);
    long long count = term1 + term2 - 10 * N - 2;
    printf("%lld\n", count);
    return 0;
}
