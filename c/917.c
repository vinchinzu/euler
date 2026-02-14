/* Project Euler Problem 917 - Grid DP with interesting indices
 * Minimum cost path on N x N grid. Uses compressed DP with
 * interesting row/column indices (prefix/suffix minima + top-K smallest).
 * N = 10^7, MOD = 998388889.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <float.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD 998388889ULL
#define S1  102022661ULL

int N_VAL = 10000000;

static ull *s_arr; /* s[1..2N] */
static ull *a_arr; /* a[1..N] */
static ull *b_arr; /* b[1..N] */
static ll *pa_arr; /* prefix sums of a */
static ll *pb_arr; /* prefix sums of b */

void generate_s(void) {
    s_arr = (ull *)malloc((size_t)(2 * N_VAL + 1) * sizeof(ull));
    s_arr[1] = S1;
    ull curr = S1;
    for (int i = 2; i <= 2 * N_VAL; i++) {
        curr = (unsigned __int128)curr * curr % MOD;
        s_arr[i] = curr;
    }
}

void fill_ab(void) {
    a_arr = (ull *)malloc((size_t)(N_VAL + 1) * sizeof(ull));
    b_arr = (ull *)malloc((size_t)(N_VAL + 1) * sizeof(ull));
    pa_arr = (ll *)malloc((size_t)(N_VAL + 1) * sizeof(ll));
    pb_arr = (ll *)malloc((size_t)(N_VAL + 1) * sizeof(ll));

    a_arr[0] = 0; b_arr[0] = 0;
    pa_arr[0] = 0; pb_arr[0] = 0;

    ll cpa = 0, cpb = 0;
    for (int i = 1; i <= N_VAL; i++) {
        a_arr[i] = s_arr[2 * i - 1];
        b_arr[i] = s_arr[2 * i];
        cpa += a_arr[i]; pa_arr[i] = cpa;
        cpb += b_arr[i]; pb_arr[i] = cpb;
    }
    free(s_arr);
}

/* Compare for qsort */
int cmp_int(const void *a, const void *b) {
    int ia = *(const int *)a, ib = *(const int *)b;
    return (ia > ib) - (ia < ib);
}

int cmp_by_val_a(const void *a, const void *b) {
    int ia = *(const int *)a, ib = *(const int *)b;
    if (a_arr[ia] < a_arr[ib]) return -1;
    if (a_arr[ia] > a_arr[ib]) return 1;
    return 0;
}

int cmp_by_val_b(const void *a, const void *b) {
    int ia = *(const int *)a, ib = *(const int *)b;
    if (b_arr[ia] < b_arr[ib]) return -1;
    if (b_arr[ia] > b_arr[ib]) return 1;
    return 0;
}

/* Remove duplicates from sorted array, return new length */
int unique_sorted(int *arr, int n) {
    if (n <= 1) return n;
    int j = 0;
    for (int i = 1; i < n; i++) {
        if (arr[i] != arr[j]) arr[++j] = arr[i];
    }
    return j + 1;
}

int *get_interesting_indices(ull *arr, int N, int K, int *out_count) {
    int *indices = (int *)malloc((size_t)(N + K + 10) * sizeof(int));
    int cnt = 0;

    /* Always include 1 and N */
    indices[cnt++] = 1;
    indices[cnt++] = N;

    /* Prefix minima */
    ull curr_min = (ull)(-1);
    for (int i = 1; i <= N; i++) {
        if (arr[i] < curr_min) {
            curr_min = arr[i];
            indices[cnt++] = i;
        }
    }

    /* Suffix minima */
    curr_min = (ull)(-1);
    for (int i = N; i >= 1; i--) {
        if (arr[i] < curr_min) {
            curr_min = arr[i];
            indices[cnt++] = i;
        }
    }

    /* Top K smallest values by threshold */
    ull threshold = 1000000;
    int *candidates = (int *)malloc((size_t)(N + 1) * sizeof(int));
    int ncand = 0;
    for (int i = 1; i <= N; i++) {
        if (arr[i] < threshold) {
            candidates[ncand++] = i;
        }
    }

    /* If too many, take K smallest */
    if (ncand > K) {
        /* Partial sort: pick K smallest by value */
        /* Simple approach: sort by value, take first K */
        /* We already have them, just sort */
        if (arr == a_arr)
            qsort(candidates, ncand, sizeof(int), cmp_by_val_a);
        else
            qsort(candidates, ncand, sizeof(int), cmp_by_val_b);
        ncand = K;
    }

    for (int i = 0; i < ncand; i++)
        indices[cnt++] = candidates[i];
    free(candidates);

    /* Sort and deduplicate */
    qsort(indices, cnt, sizeof(int), cmp_int);
    cnt = unique_sorted(indices, cnt);

    *out_count = cnt;
    return indices;
}

int main(void) {
    generate_s();
    fill_ab();

    int K = 2000;
    int NR, NC;
    int *rows = get_interesting_indices(a_arr, N_VAL, K, &NR);
    int *cols = get_interesting_indices(b_arr, N_VAL, K, &NC);

    /* DP on compressed grid */
    /* dp[r][c] = min cost to reach (rows[r], cols[c]) */
    double *dp = (double *)malloc((size_t)NR * NC * sizeof(double));
    for (int i = 0; i < NR * NC; i++) dp[i] = 1e30;

    dp[0] = (double)a_arr[rows[0]] + (double)b_arr[cols[0]];

    for (int r = 0; r < NR; r++) {
        int real_r = rows[r];
        double val_a_r = (double)a_arr[real_r];

        int has_down = (r + 1 < NR);
        int next_r = 0;
        ll diff_r = 0, cost_a_segment = 0;
        if (has_down) {
            next_r = rows[r + 1];
            diff_r = next_r - real_r;
            cost_a_segment = pa_arr[next_r] - pa_arr[real_r];
        }

        for (int c = 0; c < NC; c++) {
            double curr = dp[r * NC + c];
            if (curr >= 1e29) continue;

            int real_c = cols[c];

            /* Move Right */
            if (c + 1 < NC) {
                int next_c = cols[c + 1];
                double cost_move = (double)(next_c - real_c) * val_a_r + (double)(pb_arr[next_c] - pb_arr[real_c]);
                double new_val = curr + cost_move;
                if (new_val < dp[r * NC + c + 1])
                    dp[r * NC + c + 1] = new_val;
            }

            /* Move Down */
            if (has_down) {
                double cost_move = (double)cost_a_segment + (double)diff_r * (double)b_arr[real_c];
                double new_val = curr + cost_move;
                if (new_val < dp[(r + 1) * NC + c])
                    dp[(r + 1) * NC + c] = new_val;
            }
        }
    }

    ll result = (ll)dp[(NR - 1) * NC + NC - 1];
    printf("%lld\n", result);

    free(dp);
    free(rows);
    free(cols);
    free(a_arr);
    free(b_arr);
    free(pa_arr);
    free(pb_arr);
    return 0;
}
