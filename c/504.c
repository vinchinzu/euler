/*
 * Project Euler Problem 504: Square on the Inside.
 * Count quadrilaterals with a square number of interior lattice points.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    const int N = 100;
    int f[101][101];
    for (int a = 1; a <= N; a++)
        for (int b = 1; b <= N; b++)
            f[a][b] = ((a+1)*(b+1) - gcd(a,b) - 1) / 2 - a;

    int max_val = 2*N*N;
    char *is_sq = (char*)calloc(max_val+1, 1);
    for (int i = 1; i*i <= max_val; i++)
        is_sq[i*i] = 1;

    long long ans = 0;
    for (int a = 1; a <= N; a++)
        for (int b = 1; b <= N; b++) {
            int fab = f[a][b];
            for (int c = 1; c <= N; c++) {
                int fab_fbc = fab + f[b][c];
                for (int d = 1; d <= N; d++) {
                    int total = fab_fbc + f[c][d] + f[d][a] + 1;
                    if (total <= max_val && is_sq[total])
                        ans++;
                }
            }
        }
    printf("%lld\n", ans);
    free(is_sq);
    return 0;
}
