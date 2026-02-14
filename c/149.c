/*
 * Project Euler 149 - Maximum-sum subsequence in a 2000x2000 table
 *
 * Generate table with Lagged Fibonacci Generator, then find max sum
 * contiguous subsequence in any row, column, or diagonal (Kadane's algorithm).
 */
#include <stdio.h>
#include <stdlib.h>

#define N 2000
#define TOTAL (N * N)
#define MOD 1000000
#define OFFSET 500000

static int table[N][N];

static int kadane(int *arr, int len) {
    if (len == 0) return -2147483647;
    int max_here = arr[0], max_so_far = arr[0];
    for (int i = 1; i < len; i++) {
        if (max_here + arr[i] > arr[i])
            max_here = max_here + arr[i];
        else
            max_here = arr[i];
        if (max_here > max_so_far)
            max_so_far = max_here;
    }
    return max_so_far;
}

int main(void) {
    /* Generate sequence */
    int *s = malloc(TOTAL * sizeof(int));

    for (int i = 0; i < 55; i++) {
        long long k = i + 1;
        long long temp = (100003 - 200003*k + 300007*k*k*k) % MOD;
        if (temp < 0) temp += MOD;
        s[i] = (int)(temp - OFFSET);
    }

    for (int i = 55; i < TOTAL; i++) {
        int temp = (s[i-24] + s[i-55] + MOD) % MOD;
        s[i] = temp - OFFSET;
    }

    /* Fill table */
    for (int i = 0; i < N; i++)
        for (int j = 0; j < N; j++)
            table[i][j] = s[i * N + j];

    free(s);

    int max_sum = -2147483647;
    int *buf = malloc(N * sizeof(int));

    /* Horizontal */
    for (int i = 0; i < N; i++) {
        int v = kadane(table[i], N);
        if (v > max_sum) max_sum = v;
    }

    /* Vertical */
    for (int j = 0; j < N; j++) {
        for (int i = 0; i < N; i++) buf[i] = table[i][j];
        int v = kadane(buf, N);
        if (v > max_sum) max_sum = v;
    }

    /* Main diagonals (top-left to bottom-right) */
    /* Starting from row r, col 0 */
    for (int r = 0; r < N; r++) {
        int len = 0;
        int i = r, j = 0;
        while (i < N && j < N) {
            buf[len++] = table[i][j];
            i++; j++;
        }
        int v = kadane(buf, len);
        if (v > max_sum) max_sum = v;
    }
    /* Starting from row 0, col c */
    for (int c = 1; c < N; c++) {
        int len = 0;
        int i = 0, j = c;
        while (i < N && j < N) {
            buf[len++] = table[i][j];
            i++; j++;
        }
        int v = kadane(buf, len);
        if (v > max_sum) max_sum = v;
    }

    /* Anti-diagonals (top-right to bottom-left) */
    /* Starting from row 0, col c */
    for (int c = 0; c < N; c++) {
        int len = 0;
        int i = 0, j = c;
        while (i < N && j >= 0) {
            buf[len++] = table[i][j];
            i++; j--;
        }
        int v = kadane(buf, len);
        if (v > max_sum) max_sum = v;
    }
    /* Starting from row r, col N-1 */
    for (int r = 1; r < N; r++) {
        int len = 0;
        int i = r, j = N - 1;
        while (i < N && j >= 0) {
            buf[len++] = table[i][j];
            i++; j--;
        }
        int v = kadane(buf, len);
        if (v > max_sum) max_sum = v;
    }

    free(buf);
    printf("%d\n", max_sum);
    return 0;
}
