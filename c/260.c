/*
 * Project Euler Problem 260: Stone Game
 *
 * Find sum(x+y+z) for all losing configurations (x<=y<=z) in a
 * 3-pile Nim variant where a player can remove k stones from 1, 2,
 * or all 3 piles.
 */
#include <stdio.h>
#include <string.h>

#define N 1000

/* 2D boolean arrays for tracking used configurations */
static char lines[N+1][N+1];
static char diags[N+1][N+1];
static char space_arr[N+1][N+1];

int main(void) {
    memset(lines, 0, sizeof(lines));
    memset(diags, 0, sizeof(diags));
    memset(space_arr, 0, sizeof(space_arr));

    long long ans = 0;

    for (int x = 0; x <= N; x++) {
        for (int y = x; y <= N; y++) {
            for (int z = y; z <= N; z++) {
                if (lines[x][y] || lines[x][z] || lines[y][z])
                    continue;
                if (diags[x][z-y] || diags[y][z-x] || diags[z][y-x])
                    continue;
                if (space_arr[y-x][z-y])
                    continue;

                /* Mark as losing configuration */
                lines[x][y] = 1;
                lines[x][z] = 1;
                lines[y][z] = 1;
                diags[x][z-y] = 1;
                diags[y][z-x] = 1;
                diags[z][y-x] = 1;
                space_arr[y-x][z-y] = 1;

                ans += x + y + z;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
