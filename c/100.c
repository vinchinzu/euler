#include <stdio.h>

int main(void) {
    const long long TARGET = 1000000000000LL;
    long long b = 15, n = 21;

    while (n <= TARGET) {
        long long b_next = 3 * b + 2 * n - 2;
        long long n_next = 4 * b + 3 * n - 3;
        b = b_next;
        n = n_next;
    }

    printf("%lld\n", b);
    return 0;
}
