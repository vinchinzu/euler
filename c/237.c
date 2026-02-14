/*
 * Project Euler Problem 237: Tours on a 4 x n playing board
 *
 * Find T(10^12) mod 10^8 where T(n) counts Hamiltonian paths on 4xn grid
 * from top-left (0,0) to bottom-left (3,0).
 *
 * Recurrence (order 4, verified by brute force for n=1..10):
 *   T(n) = 2*T(n-1) + 2*T(n-2) - 2*T(n-3) + T(n-4)
 *
 * Base values (brute-force verified):
 *   T(1)=1, T(2)=1, T(3)=4, T(4)=8
 *
 * Uses 4x4 matrix exponentiation to compute T(10^12) mod 10^8.
 */
#include <stdio.h>
#include <string.h>

#define MOD 100000000LL
#define SZ 4

typedef long long Mat[SZ][SZ];

static void mat_mult(Mat A, Mat B, Mat C) {
    Mat temp;
    for (int i = 0; i < SZ; i++)
        for (int j = 0; j < SZ; j++) {
            long long s = 0;
            for (int k = 0; k < SZ; k++)
                s = (s + A[i][k] * B[k][j]) % MOD;
            temp[i][j] = (s % MOD + MOD) % MOD;
        }
    memcpy(C, temp, sizeof(Mat));
}

static void mat_pow(Mat M, long long exp, Mat result) {
    memset(result, 0, sizeof(Mat));
    for (int i = 0; i < SZ; i++) result[i][i] = 1;

    Mat base;
    memcpy(base, M, sizeof(Mat));

    while (exp > 0) {
        if (exp & 1)
            mat_mult(result, base, result);
        mat_mult(base, base, base);
        exp >>= 1;
    }
}

int main(void) {
    long long N = 1000000000000LL; /* 10^12 */

    /* Brute-force verified values */
    long long T[5] = {0, 1, 1, 4, 8};

    /* Recurrence: T(n) = 2*T(n-1) + 2*T(n-2) - 2*T(n-3) + T(n-4) */
    long long coeffs[SZ] = {2, 2, -2, 1};

    /* Build transition matrix */
    Mat M;
    memset(M, 0, sizeof(Mat));
    for (int j = 0; j < SZ; j++)
        M[0][j] = ((coeffs[j] % MOD) + MOD) % MOD;
    for (int i = 1; i < SZ; i++)
        M[i][i - 1] = 1;

    /* Initial vector: [T(4), T(3), T(2), T(1)] */
    long long init[SZ];
    for (int i = 0; i < SZ; i++)
        init[i] = T[SZ - i];

    /* Compute M^(N-4) */
    Mat Mpow;
    mat_pow(M, N - SZ, Mpow);

    /* Result = Mpow * init, take first element */
    long long result = 0;
    for (int j = 0; j < SZ; j++)
        result = (result + Mpow[0][j] * (init[j] % MOD)) % MOD;

    result = (result % MOD + MOD) % MOD;
    printf("%lld\n", result);
    return 0;
}
