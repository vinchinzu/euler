/*
 * Project Euler 835: Supernatural Triangles
 *
 * Two cases:
 * 1) Leg and hypotenuse consecutive: sum 4t^2+6t+2 for t=1..limit1
 * 2) Two legs consecutive (Pell equation): matrix exponentiation for prefix sums
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

#define M 1234567891LL

static ll mod(ll x) {
    return ((x % M) + M) % M;
}

/* 3x3 matrix multiply mod M */
static void mat_mul(ll A[3][3], ll B[3][3], ll C[3][3]) {
    ll tmp[3][3] = {{0}};
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++)
            for (int k = 0; k < 3; k++)
                tmp[i][j] = (tmp[i][j] + (i128)A[i][k] * B[k][j]) % M;
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++)
            C[i][j] = (tmp[i][j] + M) % M;
}

static void mat_pow(ll base[3][3], ll exp, ll result[3][3]) {
    /* Identity */
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++)
            result[i][j] = (i == j) ? 1 : 0;

    ll tmp[3][3];
    while (exp > 0) {
        if (exp & 1) {
            mat_mul(result, base, tmp);
            for (int i = 0; i < 3; i++)
                for (int j = 0; j < 3; j++)
                    result[i][j] = tmp[i][j];
        }
        mat_mul(base, base, tmp);
        for (int i = 0; i < 3; i++)
            for (int j = 0; j < 3; j++)
                base[i][j] = tmp[i][j];
        exp >>= 1;
    }
}

static ll pow_mod(ll base, ll exp, ll mod_val) {
    i128 result = 1;
    i128 b = ((base % mod_val) + mod_val) % mod_val;
    while (exp > 0) {
        if (exp & 1) result = result * b % mod_val;
        b = b * b % mod_val;
        exp >>= 1;
    }
    return (ll)result;
}

int main(void) {
    ll N = 10000000000LL; /* 10^10 */
    ll B = 10;

    ll inv2 = pow_mod(2, M - 2, M);
    ll inv6 = pow_mod(6, M - 2, M);

    /* First case: limit1 = 10^(N/2) / 2 - 1, mod M */
    ll limit1 = (pow_mod(B, N / 2, M) * inv2 % M - 1 + M) % M;

    ll n = limit1;
    ll sum_t = (i128)n * ((n + 1) % M) % M * inv2 % M;
    ll sum_t2 = (i128)n * ((n + 1) % M) % M * ((2 * n + 1) % M) % M * inv6 % M;

    ll ans = (4 * sum_t2 % M + 6 * sum_t % M + 2 * n % M) % M;

    /* Second case: Pell equation */
    double sqrt2 = sqrt(2.0);
    double log_base = log(3.0 + 2.0 * sqrt2);
    ll limit2 = (ll)((N * log(B) + log(2.0 * sqrt2)) / log_base);

    ll S_limit2;
    if (limit2 == 0) {
        S_limit2 = 0;
    } else if (limit2 == 1) {
        S_limit2 = 2;
    } else if (limit2 == 2) {
        S_limit2 = 14;
    } else {
        ll rec_mat[3][3] = {
            {mod(7), mod(-7), mod(1)},
            {1, 0, 0},
            {0, 1, 0}
        };
        ll result[3][3];
        mat_pow(rec_mat, limit2 - 2, result);
        S_limit2 = ((i128)result[0][0] * 14 % M + (i128)result[0][1] * 2 % M) % M;
    }

    ans = (ans + S_limit2 - 14 + M) % M;
    printf("%lld\n", ans);
    return 0;
}
