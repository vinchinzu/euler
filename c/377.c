/*
 * Project Euler Problem 377
 *
 * f(n) = sum of all positive integers (digits 1-9 only) with digit sum n.
 * count(n) = number of such integers.
 *
 * Recurrence:
 *   f(n) = 10*sum(f(n-k) for k=1..9) + sum(k*count(n-k) for k=1..9)
 *   count(n) = sum(count(n-k) for k=1..9)
 *
 * Use 18x18 companion matrix exponentiation to compute f(13^i) for i=1..17.
 * Answer = sum(f(13^i) for i=1..17) mod 10^9.
 */

#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000000LL
#define ORDER 18
#define MAX_INIT 40

static ll f_vals[MAX_INIT + 1];
static ll count_vals[MAX_INIT + 1];

/* Matrix operations */
typedef struct { ll m[ORDER][ORDER]; } Matrix;

static Matrix mat_mult(Matrix *A, Matrix *B) {
    Matrix C;
    memset(&C, 0, sizeof(C));
    for (int i = 0; i < ORDER; i++) {
        for (int p = 0; p < ORDER; p++) {
            if (A->m[i][p] == 0) continue;
            for (int j = 0; j < ORDER; j++) {
                C.m[i][j] = (C.m[i][j] + (lll)A->m[i][p] * B->m[p][j]) % MOD;
            }
        }
    }
    return C;
}

static Matrix mat_pow(Matrix *M, ll exp) {
    Matrix result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < ORDER; i++) result.m[i][i] = 1;

    Matrix base = *M;
    while (exp > 0) {
        if (exp & 1) result = mat_mult(&result, &base);
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    return result;
}

static void compute_initial_values(void) {
    /* dp[length][sum] = {count, sum_of_values} */
    /* Flatten: dp[length * (MAX_INIT+1) + sum][0..1] */
    int dp_size = (MAX_INIT + 1) * (MAX_INIT + 1);
    ll dp_cnt[dp_size], dp_sum[dp_size];
    memset(dp_cnt, 0, sizeof(dp_cnt));
    memset(dp_sum, 0, sizeof(dp_sum));
    dp_cnt[0] = 1;  /* dp[0][0].count = 1 */

    for (int length = 1; length <= MAX_INIT; length++) {
        for (int digit = 1; digit <= 9; digit++) {
            for (int s = 0; s + digit <= MAX_INIT; s++) {
                int prev = (length - 1) * (MAX_INIT + 1) + s;
                int cur = length * (MAX_INIT + 1) + s + digit;
                ll cnt = dp_cnt[prev];
                if (!cnt) continue;
                ll sum_val = (dp_sum[prev] * 10 + cnt * digit) % MOD;
                dp_cnt[cur] = (dp_cnt[cur] + cnt) % MOD;
                dp_sum[cur] = (dp_sum[cur] + sum_val) % MOD;
            }
        }
    }

    memset(f_vals, 0, sizeof(f_vals));
    memset(count_vals, 0, sizeof(count_vals));

    for (int length = 1; length <= MAX_INIT; length++) {
        for (int s = 0; s <= MAX_INIT; s++) {
            int idx = length * (MAX_INIT + 1) + s;
            f_vals[s] = (f_vals[s] + dp_sum[idx]) % MOD;
            count_vals[s] = (count_vals[s] + dp_cnt[idx]) % MOD;
        }
    }
}

static Matrix build_companion(void) {
    Matrix M;
    memset(&M, 0, sizeof(M));

    /* Row 0: f(n) = 10*f(n-1) + ... + 10*f(n-9) + 1*c(n-1) + ... + 9*c(n-9) */
    for (int k = 0; k < 9; k++)
        M.m[0][k] = 10;
    for (int k = 0; k < 9; k++)
        M.m[0][9 + k] = k + 1;

    /* Rows 1-8: shift f values */
    for (int i = 1; i < 9; i++)
        M.m[i][i - 1] = 1;

    /* Row 9: count(n) = count(n-1) + ... + count(n-9) */
    for (int k = 0; k < 9; k++)
        M.m[9][9 + k] = 1;

    /* Rows 10-17: shift count values */
    for (int i = 10; i < 18; i++)
        M.m[i][i - 1] = 1;

    return M;
}

static ll compute_f_at(ll n) {
    if (n <= MAX_INIT) return f_vals[n];

    /* State vector at position 9: [f(9)..f(1), count(9)..count(1)] */
    ll state[ORDER];
    for (int i = 0; i < 9; i++)
        state[i] = f_vals[9 - i];
    for (int i = 0; i < 9; i++)
        state[i + 9] = count_vals[9 - i];

    Matrix companion = build_companion();
    ll power = n - 9;
    Matrix M = mat_pow(&companion, power);

    ll result = 0;
    for (int j = 0; j < ORDER; j++)
        result = (result + (lll)M.m[0][j] * state[j]) % MOD;
    return result;
}

int main(void) {
    compute_initial_values();

    ll total = 0;
    ll power = 13;
    for (int i = 1; i <= 17; i++) {
        ll fn = compute_f_at(power);
        total = (total + fn) % MOD;
        power *= 13;
    }

    /* Print with leading zeros to 9 digits */
    printf("%09lld\n", total);
    return 0;
}
