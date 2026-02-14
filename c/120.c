/* Project Euler Problem 120: Square Remainders */
#include <stdio.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    long long sum = 0;
    for (int a = 3; a <= 1000; a++) {
        int d = gcd(4, a);
        int m = 2 + d * ((a - 3) / d);
        sum += (long long)a * m;
    }
    printf("%lld\n", sum);
    return 0;
}
