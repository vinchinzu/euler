/*
 * Project Euler Problem 561: Divisor Pairs
 *
 * S((2*3*...*p_K)^n) = tr(n+1)^K - (n+1)^K
 * Sum E(K,n) for n=1..N where K=904961, N=10^12.
 * The answer reduces to (K+1) * sum of floor(N/4 / 2^i) for i=0,1,...
 */
#include <stdio.h>

int main(void) {
    long long N = 1000000000000LL;  /* 10^12 */
    long long K = 904961LL;

    long long ans = 0;
    long long n = N / 4;
    while (n > 0) {
        ans += (K + 1) * n;
        n /= 2;
    }

    printf("%lld\n", ans);
    return 0;
}
