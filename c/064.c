/* Project Euler 064 - Odd period square roots */
#include <stdio.h>
#include <math.h>

static int period_of_sqrt_cf(int n) {
    int a0 = (int)sqrt((double)n);
    if (a0 * a0 == n) return 0;

    int period = 0;
    int m = 0;
    int d = 1;
    int a = a0;

    while (1) {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        period++;
        if (a == 2 * a0) break;
    }

    return period;
}

int main(void) {
    int count = 0;

    for (int n = 2; n <= 10000; n++) {
        if (period_of_sqrt_cf(n) % 2 == 1) {
            count++;
        }
    }

    printf("%d\n", count);
    return 0;
}
