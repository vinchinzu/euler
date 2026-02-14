/*
 * Project Euler 778 - Per-Digit Mod Product
 *
 * Let a (x) b = per-digit mod 10 product. Find sum of x1 (x) x2 (x) ... (x) xK
 * for all 0 <= xi <= N.
 *
 * For each place value, count digits, build 10x10 transition matrix,
 * raise to K-th power, sum contributions.
 */
#include <stdio.h>
#include <stdint.h>

typedef __int128 i128;

#define B 10
#define MOD 1000000009LL

static int64_t mat[B][B];
static int64_t tmp[B][B];
static int64_t result_mat[B][B];

static void mat_mul(int64_t a[B][B], int64_t b[B][B], int64_t c[B][B]) {
    int64_t t[B][B] = {{0}};
    for (int i = 0; i < B; i++)
        for (int j = 0; j < B; j++)
            for (int k = 0; k < B; k++)
                t[i][j] = (t[i][j] + (i128)a[i][k] * b[k][j]) % MOD;
    for (int i = 0; i < B; i++)
        for (int j = 0; j < B; j++)
            c[i][j] = t[i][j];
}

static void mat_pow(int64_t base[B][B], int64_t exp, int64_t res[B][B]) {
    /* res = identity */
    for (int i = 0; i < B; i++)
        for (int j = 0; j < B; j++)
            res[i][j] = (i == j) ? 1 : 0;

    int64_t b[B][B];
    for (int i = 0; i < B; i++)
        for (int j = 0; j < B; j++)
            b[i][j] = base[i][j];

    while (exp > 0) {
        if (exp & 1)
            mat_mul(res, b, res);
        mat_mul(b, b, b);
        exp >>= 1;
    }
}

int main(void) {
    int64_t N = 765432;
    int64_t K = 234567;

    int64_t ans = 0;
    int64_t pow_B = 1;

    while (pow_B <= N) {
        /* Count numbers with each digit d at this place value */
        int64_t counts[B];
        for (int d = 0; d < B; d++) {
            int64_t digit_at_pos = (N / pow_B) % B;
            int64_t base_count = (N / B) / pow_B;
            int64_t diff = d - digit_at_pos;
            if (diff > 0)
                counts[d] = base_count * pow_B;
            else if (diff == 0)
                counts[d] = base_count * pow_B + (N % pow_B) + 1;
            else
                counts[d] = (base_count + 1) * pow_B;
        }

        /* Build transition matrix */
        int64_t A[B][B] = {{0}};
        for (int d = 0; d < B; d++)
            for (int d2 = 0; d2 < B; d2++)
                A[(d * d2) % B][d] = (A[(d * d2) % B][d] + counts[d2]) % MOD;

        /* Compute A^K */
        int64_t A_pow[B][B];
        mat_pow(A, K, A_pow);

        /* Sum contributions */
        for (int d = 0; d < B; d++)
            ans = (ans + (i128)A_pow[d][1] % MOD * (pow_B % MOD) % MOD * d) % MOD;

        pow_B *= B;
    }

    printf("%lld\n", ans);
    return 0;
}
