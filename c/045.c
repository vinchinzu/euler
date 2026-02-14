#include <stdio.h>
#include <stdbool.h>
#include <math.h>

bool is_pentagonal(long long num) {
    if (num <= 0) return false;
    long long disc = 1 + 24 * num;
    long long candidate = (long long)round(sqrt((double)disc));
    if (candidate * candidate != disc) return false;
    return (1 + candidate) % 6 == 0;
}

bool is_hexagonal(long long num) {
    if (num <= 0) return false;
    long long disc = 1 + 8 * num;
    long long candidate = (long long)round(sqrt((double)disc));
    if (candidate * candidate != disc) return false;
    return (1 + candidate) % 4 == 0;
}

int main(void) {
    long long known = 40755;
    int n = 286;

    while (1) {
        long long triangle = (long long)n * (n + 1) / 2;
        if (triangle > known && is_pentagonal(triangle) && is_hexagonal(triangle)) {
            printf("%lld\n", triangle);
            break;
        }
        n++;
    }

    return 0;
}
