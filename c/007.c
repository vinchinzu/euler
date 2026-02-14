#include <stdio.h>
#include <math.h>
#include <stdbool.h>

bool is_prime(unsigned long long n) {
    if (n <= 1) return false;
    for (unsigned long long i = 2; i <= (unsigned long long)sqrt((double)n); i++) {
        if (n % i == 0) return false;
    }
    return true;
}

int main(void) {
    int count = 0;
    unsigned long long num = 2;
    while (1) {
        if (is_prime(num)) {
            count++;
            if (count == 10001) {
                printf("%llu\n", num);
                break;
            }
        }
        num++;
    }
    return 0;
}
