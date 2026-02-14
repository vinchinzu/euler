#include <stdio.h>

unsigned long long gcd(unsigned long long a, unsigned long long b) {
    while (b != 0) {
        unsigned long long t = b;
        b = a % b;
        a = t;
    }
    return a;
}

unsigned long long lcm(unsigned long long a, unsigned long long b) {
    return a / gcd(a, b) * b;
}

int main(void) {
    unsigned long long result = 1;
    for (int n = 1; n <= 20; n++) {
        result = lcm(result, (unsigned long long)n);
    }
    printf("%llu\n", result);
    return 0;
}
