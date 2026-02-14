/*
 * Project Euler Problem 814: Mezzo-forte.
 *
 * Count configurations of 4N people in a circle with exactly N mutual pairs.
 * Dynamic programming approach.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 1002

typedef long long ll;
static const ll MOD = 998244353;

/* dp[a][b][k] for current and previous step */
static ll dp_prev[2][2][MAXN+3];
static ll dp_cur[2][2][MAXN+3];

int main() {
    int N = 1000;
    ll ans = 0;

    for (int sa = 0; sa < 2; sa++) {
        for (int sb = 0; sb < 2; sb++) {
            /* Initialize */
            memset(dp_prev, 0, sizeof(dp_prev));
            dp_prev[sa][sb][0] = 1;

            for (int n = 1; n < 2 * N; n++) {
                memset(dp_cur, 0, sizeof(dp_cur));

                for (int a = 0; a < 2; a++) {
                    for (int b = 0; b < 2; b++) {
                        for (int da = 0; da < 3; da++) {
                            for (int db = 0; db < 3; db++) {
                                int delta = 0;
                                if (da == 1 && db == 1) delta++;
                                if (da == 0 && a == 1) delta++;
                                if (db == 0 && b == 1) delta++;
                                int na = da / 2;
                                int nb = db / 2;

                                int kmax = N - delta;
                                if (kmax < 0) continue;
                                ll *src = dp_prev[a][b];
                                ll *dst = dp_cur[na][nb];
                                for (int k = 0; k <= kmax; k++) {
                                    if (src[k] != 0) {
                                        dst[k + delta] = (dst[k + delta] + src[k]) % MOD;
                                    }
                                }
                            }
                        }
                    }
                }

                memcpy(dp_prev, dp_cur, sizeof(dp_prev));
            }

            /* Final count */
            for (int a = 0; a < 2; a++) {
                for (int b = 0; b < 2; b++) {
                    for (int da = 0; da < 3; da++) {
                        for (int db = 0; db < 3; db++) {
                            if (sa == da / 2 && sb == db / 2) {
                                int k = N;
                                if (da == 1 && db == 1) k--;
                                if (da == 0 && b == 1) k--;
                                if (db == 0 && a == 1) k--;
                                if (k >= 0) {
                                    ans = (ans + dp_prev[a][b][k]) % MOD;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
