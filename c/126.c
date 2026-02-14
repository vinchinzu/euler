/* Project Euler Problem 126: Cuboid layers.
 *
 * N_m(x,y,z) = 2(xy+yz+zx) + 4(x+y+z)(m-1) + 4(m-1)(m-2)
 * Find smallest n with exactly 1000 cuboid layer representations.
 */
#include <stdio.h>
#include <string.h>

#define N_TARGET 1000
#define N_LIMIT 20000

int counts[N_LIMIT + 1];

int main(void) {
    memset(counts, 0, sizeof(counts));

    for (int m = 1; ; m++) {
        int m1 = m - 1;
        int term_fixed = 4 * m1 * (m - 2);
        int term_factor = 4 * m1;

        /* Check if smallest cuboid (1x1x1) exceeds limit */
        int min_cubes = 6 + 12 * m1 + term_fixed;
        if (min_cubes > N_LIMIT) break;

        for (int z = 1; ; z++) {
            int cubes_zzz = 6 * z * z + 12 * z * m1 + term_fixed;
            if (cubes_zzz > N_LIMIT) break;

            for (int y = z; ; y++) {
                int cubes_yyz = 2 * (y * y + 2 * y * z) + term_factor * (2 * y + z) + term_fixed;
                if (cubes_yyz > N_LIMIT) break;

                for (int x = y; ; x++) {
                    int n_m = 2 * (x * y + y * z + z * x) + term_factor * (x + y + z) + term_fixed;
                    if (n_m > N_LIMIT) break;
                    counts[n_m]++;
                }
            }
        }
    }

    for (int n = 1; n <= N_LIMIT; n++) {
        if (counts[n] == N_TARGET) {
            printf("%d\n", n);
            return 0;
        }
    }

    return 1;
}
