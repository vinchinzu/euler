/* Project Euler 075 - Singular integer right triangles */
#include <stdio.h>
#include <math.h>

#define LIMIT 1500000

static int perimeter_counts[LIMIT + 1];

static int gcd(int a, int b) {
    while (b) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

int main(void) {
    int m_limit = (int)sqrt((double)LIMIT / 2.0);

    for (int m = 2; m <= m_limit; m++) {
        for (int n = 1; n < m; n++) {
            if ((m - n) % 2 == 0) continue;
            if (gcd(m, n) != 1) continue;

            int primitive_l = 2 * m * (m + n);
            if (primitive_l > LIMIT) break;

            for (int k = 1; k * primitive_l <= LIMIT; k++) {
                perimeter_counts[k * primitive_l]++;
            }
        }
    }

    int count = 0;
    for (int i = 0; i <= LIMIT; i++) {
        if (perimeter_counts[i] == 1) {
            count++;
        }
    }

    printf("%d\n", count);
    return 0;
}
