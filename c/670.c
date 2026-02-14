/*
 * Project Euler 670 - Coloured Tiles
 *
 * Matrix exponentiation approach with states representing tile configs.
 * N=10^16, K=4, T=3, M=1000004321.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000004321LL
#define SZ 10  /* 1 + T*T = 1 + 9 = 10 states */
#define T 3
#define K 4

typedef struct {
    ll m[SZ][SZ];
} Mat;

static Mat mat_mult(Mat *a, Mat *b) {
    Mat result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < SZ; i++) {
        for (int k = 0; k < SZ; k++) {
            if (a->m[i][k] == 0) continue;
            for (int j = 0; j < SZ; j++) {
                result.m[i][j] = (result.m[i][j] + (lll)a->m[i][k] * b->m[k][j]) % MOD;
            }
        }
    }
    return result;
}

static Mat mat_pow(Mat *mat, ll exp) {
    Mat result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < SZ; i++) result.m[i][i] = 1;

    Mat base = *mat;
    while (exp > 0) {
        if (exp & 1) result = mat_mult(&result, &base);
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    return result;
}

/* States: 0 = (-1,-1), then (i,j) for i=0..T-1, j=0..T-1 mapped to 1+i*T+j */
static int point_idx(int i, int j) {
    return 1 + i * T + j;
}

int main() {
    ll N = 10000000000000000LL; /* 10^16 */

    Mat A;
    memset(&A, 0, sizeof(A));

    /* A[0][0] = K-1 = 3 */
    A.m[0][0] = K - 1;
    /* A[0][1] = K-2 = 2 (index of Point(0,0) = 1) */
    A.m[0][point_idx(0, 0)] = K - 2;

    for (int i = 0; i < T; i++) {
        for (int j = 0; j < T; j++) {
            A.m[point_idx(i, j)][0] = (ll)(K - 1) * (K - 2);
        }
    }

    for (int i = 1; i < T; i++) {
        for (int j = 0; j < T; j++) {
            A.m[point_idx(i - 1, j)][point_idx(i, 0)] = K - 2;
        }
    }

    for (int i = 0; i < T; i++) {
        for (int j = 1; j < T; j++) {
            A.m[point_idx(i, j - 1)][point_idx(0, j)] = K - 2;
        }
    }

    for (int i = 1; i < T; i++) {
        for (int j = 1; j < T; j++) {
            A.m[point_idx(i - 1, j - 1)][point_idx(i, j)] = 1;
        }
    }

    /* Compute A^(N-1) */
    Mat Ae = mat_pow(&A, N - 1);

    ll ans = 0;
    for (int t = 0; t < 2; t++) {
        ans = (ans + (ll)K * Ae.m[t][0]) % MOD;
        for (int i = 0; i < T; i++) {
            for (int j = 0; j < T; j++) {
                ans = (ans + (ll)K * (K - 1) % MOD * Ae.m[t][point_idx(i, j)]) % MOD;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
