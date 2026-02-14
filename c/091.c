#include <stdio.h>

int gcd(int a, int b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    int N = 50;
    int count = 0;

    /* Case 1: Right angle at origin */
    count += N * N;

    /* Case 2: Right angle at P */
    for (int x1 = 0; x1 <= N; x1++) {
        for (int y1 = 0; y1 <= N; y1++) {
            if (x1 == 0 && y1 == 0) continue;

            int g = gcd(x1, y1);
            int dx = -y1 / g;
            int dy = x1 / g;

            for (int k = 1; ; k++) {
                int x2 = x1 + k * dx;
                int y2 = y1 + k * dy;
                if (x2 < 0 || x2 > N || y2 < 0 || y2 > N) break;
                count++;
            }
        }
    }

    /* Case 3: Right angle at Q (by symmetry, same count as Case 2) */
    count += count - N * N;

    printf("%d\n", count);
    return 0;
}
