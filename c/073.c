/* Project Euler 073 - Counting fractions in a range */
#include <stdio.h>

static int gcd(int a, int b) {
    while (b) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

int main(void) {
    int limit_d = 12000;
    int count = 0;

    for (int d = 1; d <= limit_d; d++) {
        int n_min = d / 3 + 1;
        int n_max = (d - 1) / 2;

        for (int n = n_min; n <= n_max; n++) {
            if (gcd(n, d) == 1) {
                count++;
            }
        }
    }

    printf("%d\n", count);
    return 0;
}
