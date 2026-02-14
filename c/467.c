/*
 * Project Euler Problem 467: Superstring of prime and composite digital roots
 *
 * Build shortest common supersequence of first N=10000 digital roots
 * of primes and composites. Return its digits interpreted as base-10
 * number mod 10^9+7.
 *
 * DP with backtracking: dp[i][j] = min length of SCS of P[i..] and C[j..].
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define NN 10000
#define MOD 1000000007LL

typedef long long ll;

static char is_prime_arr[120001];

void sieve(int limit) {
    memset(is_prime_arr, 0, sizeof(is_prime_arr));
    for (int i = 2; i <= limit; i++) is_prime_arr[i] = 1;
    for (int i = 2; i * i <= limit; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= limit; j += i)
                is_prime_arr[j] = 0;
        }
    }
}

int digital_root(int n) {
    if (n == 0) return 0;
    return 1 + (n - 1) % 9;
}

int main(void) {
    sieve(120000);

    int P[NN], C[NN];
    int np = 0, nc = 0;

    for (int n = 2; np < NN || nc < NN; n++) {
        if (is_prime_arr[n]) {
            if (np < NN) P[np++] = digital_root(n);
        } else {
            if (nc < NN) C[nc++] = digital_root(n);
        }
    }

    /* DP arrays: dp[i][j] = min SCS length for P[i..] and C[j..] */
    /* move_i[i][j] = 1 if we should advance i (take from P) when P[i]!=C[j] */
    int stride = NN + 1;
    int *dp = (int *)malloc((long long)stride * stride * sizeof(int));
    unsigned char *move_i = (unsigned char *)calloc((long long)stride * stride, 1);

    /* Base cases */
    for (int j = 0; j <= NN; j++)
        dp[NN * stride + j] = NN - j;
    for (int i = 0; i <= NN; i++)
        dp[i * stride + NN] = NN - i;

    /* When j==NN, only P elements remain */
    for (int i = 0; i < NN; i++)
        move_i[i * stride + NN] = 1;

    for (int i = NN - 1; i >= 0; i--) {
        int pi = P[i];
        int *row = dp + i * stride;
        int *next_row = dp + (i + 1) * stride;
        unsigned char *mi_row = move_i + i * stride;
        for (int j = NN - 1; j >= 0; j--) {
            if (pi == C[j]) {
                row[j] = 1 + next_row[j + 1];
            } else {
                int val_i = next_row[j];
                int val_j = row[j + 1];
                if (val_i <= val_j) {
                    row[j] = 1 + val_i;
                    if (val_i < val_j || pi < C[j])
                        mi_row[j] = 1;
                } else {
                    row[j] = 1 + val_j;
                }
            }
        }
    }

    /* Reconstruct the answer */
    ll ans = 0;
    int i = 0, j = 0;
    while (i < NN || j < NN) {
        int idx = i * stride + j;
        int digit;
        if (i < NN && j < NN && P[i] == C[j]) {
            digit = P[i];
            i++;
            j++;
        } else if (move_i[idx]) {
            digit = P[i];
            i++;
        } else {
            digit = C[j];
            j++;
        }
        ans = (10LL * ans + digit) % MOD;
    }

    printf("%lld\n", ans);

    free(dp);
    free(move_i);
    return 0;
}
