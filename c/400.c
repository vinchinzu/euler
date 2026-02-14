/*
 * Project Euler Problem 400: Fibonacci Tree Game
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000
#define L 8192  /* max Nim value is 8191 */
#define MOD 1000000000000000000LL  /* 10^18 */

typedef long long ll;

ll g_prev[L + 1];
ll g_prev2[L + 1];
ll g_cur[L + 1];

int f[N + 1];

int main(void) {
    f[0] = 0;
    f[1] = 1;
    for (int k = 2; k <= N; k++)
        f[k] = (f[k-1] ^ f[k-2]) + 1;

    memset(g_prev2, 0, sizeof(g_prev2));
    memset(g_prev, 0, sizeof(g_prev));
    g_prev[0] = 1;

    for (int k = 2; k <= N; k++) {
        memset(g_cur, 0, sizeof(g_cur));

        int fk2 = f[k-2];
        for (int n = 0; n < L; n++) {
            if (g_prev[n] == 0) continue;
            int target = (n ^ fk2) + 1;
            if (target <= L)
                g_cur[target] = (g_cur[target] + g_prev[n]) % MOD;
        }

        int fk1 = f[k-1];
        for (int n = 0; n < L; n++) {
            if (g_prev2[n] == 0) continue;
            int target = (fk1 ^ n) + 1;
            if (target <= L)
                g_cur[target] = (g_cur[target] + g_prev2[n]) % MOD;
        }

        g_cur[0] = (g_cur[0] + 1) % MOD;

        memcpy(g_prev2, g_prev, sizeof(g_prev));
        memcpy(g_prev, g_cur, sizeof(g_cur));
    }

    printf("%lld\n", g_cur[1]);
    return 0;
}
