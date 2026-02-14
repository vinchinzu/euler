/*
 * Project Euler 623: Lambda Terms
 *
 * Count closed lambda terms with at most N=2000 characters (mod 10^9+7).
 * DP on (num_chars, num_bound_vars). Iterate b from high to low.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <string.h>

#define NN 2000
#define MOD 1000000007LL
#define MAXB 401

static long long T[NN+1][MAXB];

int main(void) {
    memset(T, 0, sizeof(T));

    for (int b = MAXB - 1; b >= 0; b--) {
        T[1][b] = b % MOD;

        for (int c = 2; c <= NN; c++) {
            long long val = 0;

            /* Abstraction: costs 5 chars, adds 1 bound var */
            if (c >= 6 && b + 1 < MAXB) {
                val = T[c-5][b+1];
            }

            /* Application: costs 2 chars, split c-2 chars into two parts */
            if (c >= 4) {
                int rem = c - 2;
                long long conv = 0;
                for (int l = 1; l < rem; l++) {
                    conv = (conv + T[l][b] % MOD * (T[rem - l][b] % MOD)) % MOD;
                }
                val = (val + conv) % MOD;
            }

            T[c][b] = val;
        }
    }

    long long ans = 0;
    for (int c = 1; c <= NN; c++) {
        ans = (ans + T[c][0]) % MOD;
    }
    printf("%lld\n", ans);
    return 0;
}
