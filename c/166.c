/*
 * Project Euler Problem 166: Criss Cross
 *
 * Count 4x4 grids with digits 0-9 where all rows, columns, and diagonals
 * have the same sum.
 */
#include <stdio.h>

int main(void) {
    long long total = 0;

    for (int s = 0; s <= 36; s++) {
        /* s=0 is trivial: all zeros = 1 grid */
        if (s == 0) { total += 1; continue; }

        /* Enumerate row0 and row1 */
        for (int a = 0; a <= 9; a++) {
            if (a > s) break;
            for (int b = 0; b <= 9; b++) {
                int ab = a + b;
                if (ab > s) break;
                for (int c = 0; c <= 9; c++) {
                    int abc = ab + c;
                    if (abc > s) break;
                    int d = s - abc;
                    if (d < 0 || d > 9) continue;

                    for (int e = 0; e <= 9; e++) {
                        if (a + e > s) break;
                        for (int f = 0; f <= 9; f++) {
                            if (b + f > s) break;
                            int ef = e + f;
                            for (int g = 0; g <= 9; g++) {
                                if (c + g > s) break;
                                int h = s - e - f - g;
                                if (h < 0 || h > 9) continue;
                                if (d + h > s) continue;

                                /* Determine remaining digits */
                                int i_min = 0;
                                int tmp = s - 9 - (a + e);
                                if (tmp > i_min) i_min = tmp;
                                int i_max = s - (a + e);
                                if (i_max > 9) i_max = 9;
                                if (i_min > i_max) continue;

                                for (int i = i_min; i <= i_max; i++) {
                                    /* anti-diagonal: d + g + j + m = s
                                     * col0: a + e + i + m = s => m = s - a - e - i
                                     * so j = s - d - g - m = s - d - g - (s - a - e - i) = a + e + i - d - g */
                                    int j = a + e + i - d - g;
                                    if (j < 0 || j > 9) continue;

                                    /* main diagonal: a + f + k + p = s
                                     * row2: i + j + k + l = s => k = s - i - j - l
                                     * col3: d + h + l + p = s => p = s - d - h - l
                                     * a + f + (s - i - j - l) + (s - d - h - l) = s
                                     * a + f + s - i - j - l + s - d - h - l = s
                                     * a + f + s - i - j - d - h - 2l = 0
                                     * 2l = a + f + s - i - j - d - h
                                     * Actually let me redo this:
                                     * row2: i + j + k + l = s
                                     * col1: b + f + j + n = s => n = s - b - f - j
                                     * col2: c + g + k + o = s => o = s - c - g - k
                                     * col3: d + h + l + p = s => p = s - d - h - l
                                     * main diag: a + f + k + p = s
                                     * a + f + k + s - d - h - l = s
                                     * k = d + h + l - a - f
                                     * From row2: l = s - i - j - k = s - i - j - (d + h + l - a - f)
                                     * l = s - i - j - d - h - l + a + f
                                     * 2l = s - i - j - d - h + a + f
                                     */
                                    int two_l = s - i - j - d - h + a + f;
                                    if (two_l < 0 || two_l > 18 || two_l % 2 != 0) continue;
                                    int l = two_l / 2;
                                    if (l < 0 || l > 9) continue;

                                    int k = s - i - j - l;
                                    if (k < 0 || k > 9) continue;

                                    int m = s - a - e - i;
                                    if (m < 0 || m > 9) continue;

                                    int n = s - b - f - j;
                                    if (n < 0 || n > 9) continue;

                                    int o = s - c - g - k;
                                    if (o < 0 || o > 9) continue;

                                    int p = s - d - h - l;
                                    if (p < 0 || p > 9) continue;

                                    /* Check row3 */
                                    if (m + n + o + p != s) continue;

                                    /* Check main diagonal */
                                    if (a + f + k + p != s) continue;

                                    /* Check anti-diagonal */
                                    if (d + g + j + m != s) continue;

                                    total++;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    printf("%lld\n", total);
    return 0;
}
