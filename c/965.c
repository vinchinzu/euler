/* Project Euler 965 - Stern-Brocot / Farey-like sum
 * Answer: 0.0003452201133
 */
#include <stdio.h>

int main(void) {
    int N = 10000;
    double total = 0.0;
    int a = 0, b = 1, c = 1, d = N;
    while (1) {
        total += 1.0 / (2.0 * b * (double)d * d);
        int k = (N + b) / d;
        int na = c, nb = d, nc = k*c - a, nd = k*d - b;
        a = na; b = nb; c = nc; d = nd;
        if (a == 1 && b == 1) break;
    }
    printf("%.13f\n", total);
    return 0;
}
