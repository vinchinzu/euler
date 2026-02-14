#include <stdio.h>

int main(void) {
    long long total = 0;
    for (int i = 1; i < 1000; i++) {
        if (i % 3 == 0 || i % 5 == 0) {
            total += i;
        }
    }
    printf("%lld\n", total);
    return 0;
}
