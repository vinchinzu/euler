/*
 * Project Euler Problem 391: Hopping Game
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdint.h>

#define NN 1010

static short F[2][NN][NN];

static long long compute(int n) {
    n += 1;
    short (*f)[NN] = F[0], (*g)[NN] = F[1];

    for (int s = 0; s <= n + 1 && s < NN; ++s)
        for (int x = 0; x <= n && x < NN; ++x)
            if (s + x >= n)
                f[s][x] = 0;
            else
                f[s][x] = s + x;

    for (int h = 1; h <= n; ++h) {
        for (int s = 0; s <= n - h; ++s) {
            for (int x = 0; x <= n && x < NN; ++x) {
                int y = f[s + 1][x];
                g[s][x] = f[s][y];
            }
        }
        short (*tmp)[NN] = f;
        f = g;
        g = tmp;
    }
    return f[0][0];
}

int main(void) {
    long long ans = 0;
    for (int i = 1; i <= 1000; ++i) {
        long long x = compute(i);
        ans += x * x * x;
    }
    printf("%lld\n", ans);
    return 0;
}
