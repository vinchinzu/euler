/*
 * Project Euler Problem 706: 3-Like Numbers.
 *
 * Matrix exponentiation on states tracking prefix digit sum counts (mod 3)
 * and total digit sum (mod 3). States: (num_sums[3], total) where each
 * num_sums[i] is in {0,1,2} and total in {0,1,2}. Total 27 states.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL
#define K 3
#define B 10
#define NSTATES 27  /* 3^3 = 27 states */

typedef long long ll;

typedef struct {
    ll m[NSTATES][NSTATES];
} Matrix;

int num_sums_from_idx(int idx, int *ns) {
    ns[0] = idx % K;
    ns[1] = (idx / K) % K;
    ns[2] = (idx / (K * K)) % K;
    return 0;
}

int idx_from_num_sums_total(int *ns, int total) {
    (void)total;
    return ns[0] + ns[1] * K + ns[2] * K * K;
}

int state_idx(int *ns, int total) {
    /* We encode state as ns[0] + ns[1]*3 + ns[2]*9, ignoring total since
       we'll handle it as a separate dimension. Actually, we need to include total.
       Total states = 3^3 * 3 = 81 */
    return ns[0] + ns[1] * K + ns[2] * K * K;
}

/* Actually we need 81 states: 27 for num_sums * 3 for total */
#undef NSTATES
#define NSTATES 81

typedef struct {
    ll m[NSTATES][NSTATES];
} Mat;

int encode(int n0, int n1, int n2, int total) {
    return n0 + n1 * K + n2 * K * K + total * K * K * K;
}

void decode(int idx, int *n0, int *n1, int *n2, int *total) {
    *n0 = idx % K;
    *n1 = (idx / K) % K;
    *n2 = (idx / (K * K)) % K;
    *total = idx / (K * K * K);
}

Mat mat_mult(Mat *a, Mat *b) {
    Mat c;
    memset(&c, 0, sizeof(c));
    for (int i = 0; i < NSTATES; i++) {
        for (int k = 0; k < NSTATES; k++) {
            if (a->m[i][k] == 0) continue;
            for (int j = 0; j < NSTATES; j++) {
                c.m[i][j] = (c.m[i][j] + a->m[i][k] * b->m[k][j]) % MOD;
            }
        }
    }
    return c;
}

Mat mat_pow(Mat *base, ll exp) {
    Mat result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < NSTATES; i++) result.m[i][i] = 1;

    Mat b = *base;
    while (exp > 0) {
        if (exp & 1) result = mat_mult(&result, &b);
        b = mat_mult(&b, &b);
        exp >>= 1;
    }
    return result;
}

int ncr2(int n) {
    /* C(n, 2) */
    if (n < 2) return 0;
    return n * (n - 1) / 2;
}

int main() {
    ll N = 100000LL;

    /* Build transition matrix */
    Mat A;
    memset(&A, 0, sizeof(A));

    for (int n0 = 0; n0 < K; n0++) {
        for (int n1 = 0; n1 < K; n1++) {
            for (int n2 = 0; n2 < K; n2++) {
                for (int total = 0; total < K; total++) {
                    int old_idx = encode(n0, n1, n2, total);
                    for (int d = 0; d < B; d++) {
                        int new_total = (total + d) % K;
                        int nn[3] = {n0, n1, n2};
                        nn[new_total] = (nn[new_total] + 1) % K;
                        int new_idx = encode(nn[0], nn[1], nn[2], new_total);
                        A.m[new_idx][old_idx] = (A.m[new_idx][old_idx] + 1) % MOD;
                    }
                }
            }
        }
    }

    /* Compute A^(N-1) */
    Mat AN = mat_pow(&A, N - 1);

    /* Count valid final states */
    ll ans = 0;
    for (int n0 = 0; n0 < K; n0++) {
        for (int n1 = 0; n1 < K; n1++) {
            for (int n2 = 0; n2 < K; n2++) {
                int f = ncr2(n0) + ncr2(n1) + ncr2(n2);
                if (f % K != 0) continue;

                for (int d = 1; d < B; d++) {
                    int start_ns[3] = {0, 0, 0};
                    start_ns[0] = (start_ns[0] + 1) % K;  /* prefix of length 0 has sum 0 */
                    start_ns[d % K] = (start_ns[d % K] + 1) % K;
                    int start_idx = encode(start_ns[0], start_ns[1], start_ns[2], d % K);

                    for (int total = 0; total < K; total++) {
                        int final_idx = encode(n0, n1, n2, total);
                        ans = (ans + AN.m[final_idx][start_idx]) % MOD;
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
