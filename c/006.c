#include <stdio.h>

int main(void) {
    unsigned long long a = 0;
    for (int x = 1; x <= 100; x++) {
        a += (unsigned long long)x * x;
    }
    unsigned long long sum = 0;
    for (int x = 1; x <= 100; x++) {
        sum += x;
    }
    unsigned long long b = sum * sum;
    unsigned long long c = b - a;
    printf("%llu\n", c);
    return 0;
}
