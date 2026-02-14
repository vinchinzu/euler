/*
 * Project Euler 645 - Every Day is a Holiday
 * Expected number of emperors until every day is a holiday.
 */
#include <stdio.h>

static double nCr(int n, int k) {
    if (k < 0 || k > n) return 0.0;
    double result = 1.0;
    for (int i = 0; i < k; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

int main(void) {
    int N = 10000;

    double P = 1.0;
    double ans = 1.0;
    for (int k = N - 1; k > 0; k--) {
        ans += (1 - P) * (double)N / (N - k);
        if (k >= 2 && 2 * k - N >= 2) {
            P *= nCr(2 * k - N, 2) / nCr(k, 2);
        } else {
            P = 0.0;
        }
    }

    printf("%.4f\n", ans);
    return 0;
}
