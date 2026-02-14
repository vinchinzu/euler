/*
 * Project Euler Problem 242: Odd Triplets
 *
 * Find the number of pairs (n, k) with n <= N, n, k, f(n,k) all odd,
 * where f(n,k) = number of k-element subsets of {1..n} with odd sum.
 */
#include <stdio.h>
#include <stdlib.h>

typedef unsigned long long ull;

/* 3^e for moderate e */
static ull pow3(int e) {
    ull r = 1;
    for (int i = 0; i < e; i++) r *= 3;
    return r;
}

int main(void) {
    ull N = 1000000000000ULL;  /* 10^12 */
    ull n = (N + 3) / 4;  /* ceil(N/4) */
    int e = 0;
    ull ans = 0;

    while (n > 0) {
        ans = (n % 2 + 1) * ans + (n % 2) * pow3(e);
        n /= 2;
        e++;
    }

    printf("%llu\n", ans);
    return 0;
}
