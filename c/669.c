/*
 * Project Euler 669 - The King's Banquet
 *
 * Arrangement of 1..N where consecutive numbers sum to Fibonacci.
 * Uses __int128 for modular multiplication since N is ~10^17.
 */
#include <stdio.h>
#include <stdint.h>

typedef unsigned long long ull;
typedef __int128 u128;

int main() {
    ull N = 99194853094755497ULL;
    ull K = 10000000000000000ULL; /* 10^16 */

    /* Generate Fibonacci numbers up to N */
    ull fib[200];
    int nfib = 0;
    fib[0] = 1; fib[1] = 1;
    nfib = 2;
    while (fib[nfib - 1] < N) {
        fib[nfib] = fib[nfib - 1] + fib[nfib - 2];
        nfib++;
    }

    /* Find largest Fibonacci number < N */
    ull a;
    if (fib[nfib - 1] >= N)
        a = fib[nfib - 2];
    else
        a = fib[nfib - 1];

    ull ans;
    if ((N - K) % 2 == 0) {
        ull d = (N - K) / 2;
        /* ans = (-d * a) % N using __int128 */
        u128 da = (u128)d * (u128)a;
        u128 rem = da % (u128)N;
        if (rem == 0)
            ans = 0;
        else
            ans = (ull)((u128)N - rem);
    } else {
        ull d = (N + 1 - K) / 2;
        u128 da = (u128)d * (u128)a;
        ans = (ull)(da % (u128)N);
    }

    printf("%llu\n", ans);
    return 0;
}
