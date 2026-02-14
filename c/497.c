/* Project Euler 497 - Drunken Tower of Hanoi
 * Translated from python/497.py
 *
 * Uses DP for move counts on 3 rods.
 * Memory-optimized: only keep current and previous level.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000000LL
#define NMAX 10000

/* numMoves[start][end][s][e] */
/* We need current (n) and previous (n-1) */
ll cur[3][3][3][3];
ll prev[3][3][3][3];

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    int N = NMAX;
    ll M = MOD;

    /* We'll accumulate answer as we go, computing numMoves level by level */
    /* For n=1: numMoves[1][start][end][s][e] = 1 iff s==start && e==end && start!=end */
    memset(prev, 0, sizeof(prev));
    for (int start = 0; start < 3; start++)
        for (int end = 0; end < 3; end++)
            if (start != end)
                prev[start][end][start][end] = 1;

    /* Accumulate answer for n=1 */
    ll ans = 0;
    {
        ll k = powmod(10, 1, M);
        ll rods[3];
        rods[0] = powmod(3, 1, M);
        rods[1] = powmod(6, 1, M);
        rods[2] = powmod(9, 1, M);

        for (int s = 0; s < 3; s++) {
            for (int e = 0; e < 3; e++) {
                ll dist;
                if (s < e) {
                    dist = ((rods[e] - 1 + M) % M * ((rods[e] - 1 + M) % M) % M
                          - (rods[s] - 1 + M) % M * ((rods[s] - 1 + M) % M) % M + M) % M;
                } else {
                    dist = ((k - rods[e] + M) % M * ((k - rods[e] + M) % M) % M
                          - (k - rods[s] + M) % M * ((k - rods[s] + M) % M) % M + M) % M;
                }
                /* count = numMoves[1][1][0][s][e] + numMoves[1][0][2][s][e] */
                ll count = (prev[1][0][s][e] + prev[0][2][s][e]) % M;
                ans = (ans + dist % M * count % M) % M;
            }
        }
    }

    for (int n = 2; n <= N; n++) {
        memset(cur, 0, sizeof(cur));
        for (int start = 0; start < 3; start++) {
            for (int end = 0; end < 3; end++) {
                if (start == end) continue;
                int other = 3 - start - end;
                for (int s = 0; s < 3; s++) {
                    for (int e = 0; e < 3; e++) {
                        cur[start][end][s][e] = (cur[start][end][s][e] + prev[start][other][s][e]) % M;
                    }
                }
                cur[start][end][other][start] = (cur[start][end][other][start] + 1) % M;
                cur[start][end][start][end] = (cur[start][end][start][end] + 1) % M;
                cur[start][end][end][other] = (cur[start][end][end][other] + 1) % M;
                for (int s = 0; s < 3; s++) {
                    for (int e = 0; e < 3; e++) {
                        cur[start][end][s][e] = (cur[start][end][s][e] + prev[other][end][s][e]) % M;
                    }
                }
            }
        }

        /* Accumulate answer for this n */
        ll k = powmod(10, n, M);
        ll rods[3];
        rods[0] = powmod(3, n, M);
        rods[1] = powmod(6, n, M);
        rods[2] = powmod(9, n, M);

        for (int s = 0; s < 3; s++) {
            for (int e = 0; e < 3; e++) {
                ll dist;
                if (s < e) {
                    dist = ((rods[e] - 1 + M) % M * ((rods[e] - 1 + M) % M) % M
                          + M - (rods[s] - 1 + M) % M * ((rods[s] - 1 + M) % M) % M) % M;
                } else {
                    dist = ((k - rods[e] + M) % M * ((k - rods[e] + M) % M) % M
                          + M - (k - rods[s] + M) % M * ((k - rods[s] + M) % M) % M) % M;
                }
                /* numMoves[1][1][0][s][e] is from the n=1 level (stored in first iteration's prev) */
                /* We need to keep the n=1 level separately */
                ll count = (cur[0][2][s][e]) % M;
                /* Wait - the Python code uses: numMoves[1][1][0][s][e] + numMoves[n][0][2][s][e] */
                /* numMoves[1] is fixed = prev from first iteration. We need to store it. */
                /* Let me restructure... */
                (void)dist;
                (void)count;
            }
        }

        memcpy(prev, cur, sizeof(cur));
    }

    /* Need to restart with proper approach - store numMoves[1] separately */
    /* Reset */
    ll nm1[3][3][3][3];
    memset(nm1, 0, sizeof(nm1));
    for (int start = 0; start < 3; start++)
        for (int end = 0; end < 3; end++)
            if (start != end)
                nm1[start][end][start][end] = 1;

    memcpy(prev, nm1, sizeof(nm1));
    ans = 0;

    for (int n = 1; n <= N; n++) {
        if (n == 1) {
            memcpy(cur, nm1, sizeof(nm1));
        } else {
            memset(cur, 0, sizeof(cur));
            for (int start = 0; start < 3; start++) {
                for (int end = 0; end < 3; end++) {
                    if (start == end) continue;
                    int other = 3 - start - end;
                    for (int s = 0; s < 3; s++)
                        for (int e = 0; e < 3; e++)
                            cur[start][end][s][e] = (cur[start][end][s][e] + prev[start][other][s][e]) % M;
                    cur[start][end][other][start] = (cur[start][end][other][start] + 1) % M;
                    cur[start][end][start][end] = (cur[start][end][start][end] + 1) % M;
                    cur[start][end][end][other] = (cur[start][end][end][other] + 1) % M;
                    for (int s = 0; s < 3; s++)
                        for (int e = 0; e < 3; e++)
                            cur[start][end][s][e] = (cur[start][end][s][e] + prev[other][end][s][e]) % M;
                }
            }
        }

        ll k = powmod(10, n, M);
        ll rods[3];
        rods[0] = powmod(3, n, M);
        rods[1] = powmod(6, n, M);
        rods[2] = powmod(9, n, M);

        for (int s = 0; s < 3; s++) {
            for (int e = 0; e < 3; e++) {
                ll dist;
                if (s < e) {
                    ll re = (rods[e] - 1 + M) % M;
                    ll rs = (rods[s] - 1 + M) % M;
                    dist = (re % M * (re % M) % M + M - rs % M * (rs % M) % M) % M;
                } else {
                    ll re = (k - rods[e] + M) % M;
                    ll rs = (k - rods[s] + M) % M;
                    dist = (re % M * (re % M) % M + M - rs % M * (rs % M) % M) % M;
                }
                ll count = (nm1[1][0][s][e] + cur[0][2][s][e]) % M;
                ans = (ans + dist * count) % M;
            }
        }

        if (n >= 1) memcpy(prev, cur, sizeof(cur));
    }

    printf("%lld\n", ans % M);
    return 0;
}
