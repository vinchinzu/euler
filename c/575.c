/*
 * Project Euler Problem 575: Wandering Robot
 *
 * Robot in N x N room. Steady state probability of being in square-numbered room.
 * Case 1: self-loop adds 1 to degree. Corner=3, Side=4, Center=5. Total=N(5N-4).
 * Case 2: double self-loop. Corner=2, Side=3, Center=4. Total=4N(N-1).
 * Average of two cases.
 */
#include <stdio.h>
#include <math.h>

static int is_square(int n) {
    int r = (int)sqrt((double)n);
    if (r * r == n) return 1;
    if ((r + 1) * (r + 1) == n) return 1;
    return 0;
}

int main(void) {
    int N = 1000;
    long long case1 = 0, case2 = 0;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            int room = i * N + j + 1;
            if (is_square(room)) {
                int is_corner = (i == 0 || i == N - 1) && (j == 0 || j == N - 1);
                int is_side = (i == 0 || i == N - 1 || j == 0 || j == N - 1);
                if (is_corner) {
                    case1 += 3;
                    case2 += 2;
                } else if (is_side) {
                    case1 += 4;
                    case2 += 3;
                } else {
                    case1 += 5;
                    case2 += 4;
                }
            }
        }
    }

    double total1 = (double)N * (5.0 * N - 4.0);
    double total2 = 4.0 * N * (N - 1.0);
    double ans = ((double)case1 / total1 + (double)case2 / total2) / 2.0;

    printf("%.12f\n", ans);
    return 0;
}
