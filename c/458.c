/*
 * Project Euler 458 - Permutations of Project
 *
 * Count strings of length N from K=7 letters that do NOT contain
 * a permutation of all K letters as a substring.
 * Uses matrix exponentiation.
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef long long ll;

#define K 7
#define MOD 1000000000LL  /* 10^9 */

typedef struct {
    ll m[K][K];
} Matrix;

static Matrix mat_mul(Matrix a, Matrix b) {
    Matrix c;
    memset(&c, 0, sizeof(c));
    for (int i = 0; i < K; i++)
        for (int k = 0; k < K; k++) {
            if (a.m[i][k] == 0) continue;
            for (int j = 0; j < K; j++)
                c.m[i][j] = (c.m[i][j] + a.m[i][k] * b.m[k][j]) % MOD;
        }
    return c;
}

static Matrix mat_pow(Matrix base, ll exp) {
    Matrix result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < K; i++) result.m[i][i] = 1;

    while (exp > 0) {
        if (exp & 1) result = mat_mul(result, base);
        base = mat_mul(base, base);
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll N = 1000000000000LL;  /* 10^12 */

    /* Build transition matrix */
    Matrix A;
    memset(&A, 0, sizeof(A));
    for (int n = 1; n < K; n++) {
        A.m[n][n - 1] = K - n + 1;
        for (int i = n; i < K; i++)
            A.m[n][i] = 1;
    }

    Matrix An = mat_pow(A, N);

    ll ans = 0;
    for (int i = 0; i < K; i++)
        ans = (ans + An.m[i][0]) % MOD;

    printf("%lld\n", ans);
    return 0;
}
