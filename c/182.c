/* Project Euler 182: RSA encryption. */
#include <stdio.h>

static long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    long long P = 1009, Q = 3643;
    long long PHI = (P - 1) * (Q - 1);

    long long min_unconcealed = -1;
    long long sum_e = 0;

    for (long long e = 2; e < PHI; e++) {
        if (gcd(e, PHI) != 1) continue;

        long long unconcealed = (1 + gcd(e - 1, P - 1)) * (1 + gcd(e - 1, Q - 1));

        if (min_unconcealed < 0 || unconcealed < min_unconcealed) {
            min_unconcealed = unconcealed;
            sum_e = e;
        } else if (unconcealed == min_unconcealed) {
            sum_e += e;
        }
    }

    printf("%lld\n", sum_e);
    return 0;
}
