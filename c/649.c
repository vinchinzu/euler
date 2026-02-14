/*
 * Project Euler 649 - Low-Prime Chessboard Nim
 * Moves: 2, 3, 5, 7 squares left or up.
 * N = 10000019, C = 100 coins, L = 9 (period), H = 8 (max nimber).
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;

#define N 10000019
#define C_COINS 100
#define M 1000000000LL
#define L 9
#define H 8

static int grid[L][L];
static ll counts[H];
static ll dp[C_COINS + 1][H];

static int ceil_div(int a, int b) {
    return (a + b - 1) / b;
}

int main(void) {
    /* Compute grid nimbers */
    int moves[] = {2, 3, 5, 7};
    for (int i = 0; i < L; i++) {
        for (int j = 0; j < L; j++) {
            int used[H];
            memset(used, 0, sizeof(used));
            for (int mi = 0; mi < 4; mi++) {
                int d = moves[mi];
                if (i >= d && grid[i - d][j] < H)
                    used[grid[i - d][j]] = 1;
                if (j >= d && grid[i][j - d] < H)
                    used[grid[i][j - d]] = 1;
            }
            int nimber = 0;
            while (nimber < H && used[nimber]) nimber++;
            grid[i][j] = nimber;
        }
    }

    /* Count squares with each nimber value */
    memset(counts, 0, sizeof(counts));
    for (int i = 0; i < L; i++) {
        for (int j = 0; j < L; j++) {
            ll count_i = ceil_div(N - i, L);
            ll count_j = ceil_div(N - j, L);
            counts[grid[i][j]] = (counts[grid[i][j]] + count_i * count_j) % M;
        }
    }

    /* DP: dp[c][total] = number of ways with c coins and nimber total */
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    for (int c = 1; c <= C_COINS; c++) {
        for (int total = 0; total < H; total++) {
            for (int curr = 0; curr < H; curr++) {
                dp[c][total] = (dp[c][total] + dp[c - 1][total ^ curr] * counts[curr]) % M;
            }
        }
    }

    ll ans = 0;
    for (int curr = 1; curr < H; curr++) {
        ans = (ans + dp[C_COINS][curr]) % M;
    }

    printf("%lld\n", ans);
    return 0;
}
