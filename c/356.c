/* Project Euler 356 - Largest Roots of Cubic Polynomials
 *
 * For each i, g(x) = x^3 - 2^i * x^2 + i = 0.
 * a_i is the largest root. Compute sum floor(a_i^K) mod 10^8 for K=987654321, i=1..30.
 *
 * S_k = a^k + b^k + c^k satisfies S_k = 2^i * S_{k-1} - i * S_{k-3}.
 * Since |b|,|c| < 1 for i >= 1 and K is odd, floor(a^K) = S_K - 1.
 * Use matrix exponentiation mod 10^8.
 */

#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

#define MOD 100000000LL
#define K 987654321LL

typedef struct { ll m[3][3]; } Mat;

Mat mat_mul(Mat A, Mat B) {
    Mat C;
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++) {
            i128 s = 0;
            for (int k = 0; k < 3; k++)
                s += (i128)A.m[i][k] * B.m[k][j];
            C.m[i][j] = (ll)(s % MOD);
            if (C.m[i][j] < 0) C.m[i][j] += MOD;
        }
    return C;
}

Mat mat_pow(Mat M, ll p) {
    Mat result;
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++)
            result.m[i][j] = (i == j) ? 1 : 0;

    while (p > 0) {
        if (p & 1) result = mat_mul(result, M);
        M = mat_mul(M, M);
        p >>= 1;
    }
    return result;
}

ll mod_pow_ll(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll compute_SK(int i) {
    ll p = mod_pow_ll(2, i, MOD);
    ll neg_i = (-i % MOD + MOD) % MOD;

    ll S0 = 3 % MOD;
    ll S1 = p;
    ll S2 = (i128)p * p % MOD;

    if (K == 0) return S0;
    if (K == 1) return S1;
    if (K == 2) return S2;

    Mat M;
    M.m[0][0] = p;     M.m[0][1] = 0; M.m[0][2] = neg_i;
    M.m[1][0] = 1;     M.m[1][1] = 0; M.m[1][2] = 0;
    M.m[2][0] = 0;     M.m[2][1] = 1; M.m[2][2] = 0;

    Mat Mpow = mat_pow(M, K - 2);

    ll SK = ((i128)Mpow.m[0][0] * S2 + (i128)Mpow.m[0][1] * S1 + (i128)Mpow.m[0][2] * S0) % MOD;
    if (SK < 0) SK += MOD;
    return SK;
}

int main(void) {
    ll total = 0;
    for (int i = 1; i <= 30; i++) {
        ll SK = compute_SK(i);
        /* b^K + c^K > 0 for all i=1..30 with K odd, so floor(a^K) = S_K - 1 */
        ll floor_aK = (SK - 1 + MOD) % MOD;
        total = (total + floor_aK) % MOD;
    }
    printf("%lld\n", total);
    return 0;
}
