/*
 * Project Euler 624: Two heads are better than one
 *
 * Probability a/b that first occurrence of two consecutive heads ends
 * after a multiple of N coin flips.
 *
 * D_n = -2*D_{n-1} + 4*D_{n-2}
 * a = (-1)^n * (4*D_{n-2} - 1)
 * b = 4^n + (-1)^n * (2*D_{n-1} - 8*D_{n-2} + 1)
 * Answer = a * b^{-1} mod M
 *
 * Uses 2x2 matrix exponentiation for the recurrence.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000009LL

ll mod(ll a) {
    return ((a % MOD) + MOD) % MOD;
}

ll powmod(ll base, ll exp, ll m) {
    ll result = 1;
    base = ((base % m) + m) % m;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % m;
        base = (lll)base * base % m;
        exp >>= 1;
    }
    return result;
}

ll modinv(ll a, ll m) {
    return powmod(a, m - 2, m);
}

typedef struct { ll m[2][2]; } Mat;

Mat matmul(Mat A, Mat B) {
    Mat C;
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 2; j++) {
            C.m[i][j] = 0;
            for (int k = 0; k < 2; k++)
                C.m[i][j] = (C.m[i][j] + (lll)A.m[i][k] * B.m[k][j]) % MOD;
            C.m[i][j] = mod(C.m[i][j]);
        }
    return C;
}

Mat matpow(Mat A, ll exp) {
    Mat result = {{{1, 0}, {0, 1}}};
    while (exp > 0) {
        if (exp & 1) result = matmul(result, A);
        A = matmul(A, A);
        exp >>= 1;
    }
    return result;
}

int main() {
    ll N = 1000000000000000000LL; /* 10^18 */

    /* A = [[-2, 4], [1, 0]] */
    /* D_0 = 1 (identity start), compute A^(N-1) and A^(N-2) */
    /* A^k applied to [D_1, D_0] = [D_{k+1}, D_k] */
    /* D_0 = 1, D_1 = -2 (from D_1 = -2*D_0 + 4*D_{-1}; but actually D_0 is the starting det) */
    /* Actually the recurrence starts with D_0 = 1, D_1 = -2*1 = -2 (assuming D_{-1} = 0) */
    /* Let's use matrix to compute: [D_n, D_{n-1}] = A^{n} * [D_0, ...] */
    /* With D_0 = 1 and vec = [1, 0] (just the initial state) */
    /* A^n * [1; 0] gives [D_n; D_{n-1}] where the recurrence is D_n = -2*D_{n-1} + 4*D_{n-2} */
    /* A = [[-2, 4], [1, 0]], initial [D_0; 0] = [1; 0] */

    Mat A;
    A.m[0][0] = mod(-2); A.m[0][1] = 4;
    A.m[1][0] = 1; A.m[1][1] = 0;

    /* A^(N-1) * [1; 0] gives [D_{N-1}; D_{N-2}] */
    Mat An1 = matpow(A, N - 1);
    ll d1 = An1.m[0][0]; /* D_{N-1} */
    ll d2 = An1.m[1][0]; /* D_{N-2} */

    ll parity_n = (N % 2 == 0) ? 1 : (MOD - 1); /* (-1)^N */

    ll a = mod(parity_n * mod(4 * d2 - 1 + MOD) % MOD);
    ll b_val = mod(powmod(4, N, MOD) + parity_n * mod(2 * d1 % MOD - 8 * d2 % MOD + 1 + 3 * MOD) % MOD);

    ll ans = (lll)a % MOD * modinv(b_val, MOD) % MOD;
    printf("%lld\n", mod(ans));
    return 0;
}
