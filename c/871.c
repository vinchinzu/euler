/*
 * Project Euler 871 - Functional Graph D(f_n)
 *
 * For n = 10^5+1 to 10^5+100, compute D(f_n) where f(x) = (x^3 + x + 1) % n.
 * D(f) = max independent antecedent set size.
 *
 * Algorithm: functional graph decomposition into trees + cycles.
 * For trees: DP with P[u] (include u) and S[u] (exclude u).
 * For cycles: DP with adjacency constraint.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 100200

static int f_map[MAXN];
static int in_degree[MAXN];
static int queue_buf[MAXN];
static int sum_S[MAXN];
static int max_diff[MAXN];
static int visited[MAXN];

/* Cycle DP arrays */
static int cycle_nodes[MAXN];
static int cycle_P[MAXN];
static int cycle_S[MAXN];

static int solve_cycle(int *P, int *S, int k) {
    if (k == 0) return 0;

    /* Case 1: c0 is NOT in A */
    int dp0 = S[0];
    int dp1 = -1000000000;

    for (int i = 1; i < k; i++) {
        int next_dp0 = dp0 + S[i];
        int t = dp1 + P[i] - 1;
        if (t > next_dp0) next_dp0 = t;
        int next_dp1 = dp0 + P[i];
        dp0 = next_dp0;
        dp1 = next_dp1;
    }

    int res0 = dp0;
    int res1 = dp1 - S[0] + (P[0] - 1);
    int ans_case0 = res0 > res1 ? res0 : res1;

    /* Case 2: c0 IS in A */
    dp0 = -1000000000;
    dp1 = P[0];

    for (int i = 1; i < k; i++) {
        int next_dp0 = dp0 + S[i];
        int t = dp1 + P[i] - 1;
        if (t > next_dp0) next_dp0 = t;
        int next_dp1 = dp0 + P[i];
        dp0 = next_dp0;
        dp1 = next_dp1;
    }

    int ans_case1 = dp0;

    return ans_case0 > ans_case1 ? ans_case0 : ans_case1;
}

static int compute_D(int n) {
    /* Build functional graph f(x) = (x^3 + x + 1) % n */
    for (int x = 0; x < n; x++) {
        long long xx = (long long)x;
        long long val = ((xx * xx % n) * xx % n + xx + 1) % n;
        f_map[x] = (int)val;
    }

    memset(in_degree, 0, n * sizeof(int));
    for (int x = 0; x < n; x++)
        in_degree[f_map[x]]++;

    memset(sum_S, 0, n * sizeof(int));
    memset(max_diff, 0, n * sizeof(int));

    /* Topological sort (peel off trees) */
    int head = 0, tail = 0;
    for (int x = 0; x < n; x++)
        if (in_degree[x] == 0)
            queue_buf[tail++] = x;

    while (head < tail) {
        int u = queue_buf[head++];
        int p_u = 1 + sum_S[u];
        int s_u = sum_S[u] + max_diff[u];
        int v = f_map[u];
        sum_S[v] += s_u;
        int diff = p_u - s_u;
        if (diff > max_diff[v])
            max_diff[v] = diff;
        in_degree[v]--;
        if (in_degree[v] == 0)
            queue_buf[tail++] = v;
    }

    /* Process cycles */
    memset(visited, 0, n * sizeof(int));
    int total_max = 0;

    for (int i = 0; i < n; i++) {
        if (in_degree[i] > 0 && !visited[i]) {
            int k = 0;
            int curr = i;
            while (!visited[curr]) {
                visited[curr] = 1;
                cycle_nodes[k++] = curr;
                curr = f_map[curr];
            }

            for (int j = 0; j < k; j++) {
                int node = cycle_nodes[j];
                cycle_P[j] = 1 + sum_S[node];
                cycle_S[j] = sum_S[node] + max_diff[node];
            }

            total_max += solve_cycle(cycle_P, cycle_S, k);
        }
    }

    return total_max;
}

int main(void) {
    long long total_D = 0;
    int start_n = 100001;
    int end_n = 100100;

    for (int n = start_n; n <= end_n; n++)
        total_D += compute_D(n);

    printf("%lld\n", total_D);
    return 0;
}
