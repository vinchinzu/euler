/* Project Euler 349 - Langton's Ant
 *
 * After 10^18 moves, how many black squares are there?
 * Langton's ant enters a "highway" phase after about 10000 steps,
 * with a period of 104 steps adding 12 black squares per cycle.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define L 20000
#define GRID 512   /* grid size (centered at 256,256) */

static char grid[GRID][GRID];
static long long num_blacks_arr[L];

int main(void) {
    long long N = 1000000000000000000LL; /* 10^18 */
    int P = 104;

    memset(grid, 0, sizeof(grid));
    int black_count = 0;
    int ax = GRID / 2, ay = GRID / 2;
    int dx[] = {0, 1, 0, -1};
    int dy[] = {1, 0, -1, 0};
    int d = 0;

    for (int step = 0; step < L; step++) {
        num_blacks_arr[step] = black_count;
        if (grid[ax][ay]) {
            d = (d + 1) % 4;
            grid[ax][ay] = 0;
            black_count--;
        } else {
            d = (d + 3) % 4; /* (d-1+4)%4 */
            grid[ax][ay] = 1;
            black_count++;
        }
        ax += dx[d];
        ay += dy[d];
    }

    /* Find base index */
    long long base = ((L - P) / P) * (long long)P + N % P;
    long long ans = num_blacks_arr[base] + (N - base) / P * (num_blacks_arr[base] - num_blacks_arr[base - P]);
    printf("%lld\n", ans);
    return 0;
}
