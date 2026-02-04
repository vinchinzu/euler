"""Project Euler Problem 732: Standing on the Shoulders of Trolls.

Maximize remaining IQ after building a troll ladder out of a pit.
Uses C with divide-and-conquer knapsack for memory efficiency.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 1000
#define MOD_VAL 1000000007LL

typedef struct { int h, l, q; } Troll;

static Troll trolls[N];
static int D;
static int total_iq;

static int imod(long long a, long long m) {
    return (int)(((a % m) + m) % m);
}

static void generate_trolls(void) {
    long long r = 1;
    long long M = MOD_VAL;
    for (int i = 0; i < N; i++) {
        trolls[i].h = imod(r, 101) + 50;
        r = (r * 5) % M;
        trolls[i].l = imod(r, 101) + 50;
        r = (r * 5) % M;
        trolls[i].q = imod(r, 101) + 50;
        r = (r * 5) % M;
    }
}

/* Knapsack DP: dp[j] = min IQ to reach height >= j
   After adding a troll with (h, q): for j from D down to h, dp[j] = min(dp[j], dp[j-h]+q)
   Then apply suffix minimum: for j from D-1 down to 0, dp[j] = min(dp[j], dp[j+1])
*/

static const int INF = 1000000000;

/* Store all right DP layers: right_dp[i][j] for i=0..N-1
   right_dp[i] = knapsack over trolls [i+1..N-1], with suffix min applied.
   right_dp[N-1][j] = INF for j>0, 0 for j=0  (no trolls)

   Actually we need right[N-1-i] = knapsack over trolls [i+1..N-1].
   Let's define:
     rdp[k] = knapsack over last k trolls = trolls[N-k..N-1]
   We need rdp[N-1-i] = knapsack over trolls[i+1..N-1].

   We store rdp[0], rdp[1], ..., rdp[N-1] (N layers of size D+1).
*/

/* Use flat array for right DP */
static int *right_all;  /* right_all[k * (D+1) + j] */

static inline int* rdp_row(int k) {
    return right_all + (long long)k * (D + 1);
}

int main(void) {
    generate_trolls();

    int total_h = 0;
    total_iq = 0;
    for (int i = 0; i < N; i++) {
        total_h += trolls[i].h;
        total_iq += trolls[i].q;
    }
    D = (int)ceil((double)total_h / sqrt(2.0));

    /* Allocate right DP: N layers of (D+1) ints */
    right_all = (int *)malloc((long long)N * (D + 1) * sizeof(int));
    if (!right_all) { fprintf(stderr, "malloc fail\n"); return 1; }

    /* rdp[0] = empty knapsack */
    {
        int *row = rdp_row(0);
        row[0] = 0;
        for (int j = 1; j <= D; j++) row[j] = INF;
    }

    /* Build right DP layers: rdp[k] adds troll[N-k] to rdp[k-1] */
    for (int k = 1; k < N; k++) {
        int *prev = rdp_row(k - 1);
        int *curr = rdp_row(k);
        memcpy(curr, prev, (D + 1) * sizeof(int));

        int h = trolls[N - k].h;
        int q = trolls[N - k].q;

        for (int j = D; j >= h; j--) {
            int val = curr[j - h];
            if (val < INF && val + q < curr[j]) {
                curr[j] = val + q;
            }
        }

        /* Suffix minimum: dp[j] = min(dp[j], dp[j+1]) */
        for (int j = D - 1; j >= 0; j--) {
            if (curr[j + 1] < curr[j]) curr[j] = curr[j + 1];
        }
    }

    /* Now compute left DP incrementally and combine */
    int *ldp = (int *)malloc((D + 1) * sizeof(int));
    /* ldp starts as empty knapsack (before troll 0) */
    ldp[0] = 0;
    for (int j = 1; j <= D; j++) ldp[j] = INF;

    int ans = 0;

    for (int i = 0; i < N; i++) {
        /* At this point, ldp = knapsack over trolls [0..i-1] (prefix of i trolls) */
        /* rdp[N-1-i] = knapsack over trolls [i+1..N-1] */

        int dist = D - trolls[i].h - trolls[i].l;
        if (dist >= 0) {
            int *rrow = rdp_row(N - 1 - i);
            for (int j = 0; j <= dist; j++) {
                int lv = ldp[j];
                int rv = rrow[dist - j];
                if (lv < INF && rv < INF) {
                    int iq_used = lv + rv;
                    int remaining = total_iq - iq_used;
                    if (remaining > ans) ans = remaining;
                }
            }
        }

        /* Update ldp: add troll i */
        int h = trolls[i].h;
        int q = trolls[i].q;
        for (int j = D; j >= h; j--) {
            int val = ldp[j - h];
            if (val < INF && val + q < ldp[j]) {
                ldp[j] = val + q;
            }
        }
        /* Suffix minimum */
        for (int j = D - 1; j >= 0; j--) {
            if (ldp[j + 1] < ldp[j]) ldp[j] = ldp[j + 1];
        }
    }

    printf("%d\n", ans);

    free(right_all);
    free(ldp);
    return 0;
}
""";

    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
