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
    unsigned long long n = 600851475143ULL;
    unsigned long long factors[64];
    int count = 0;
    unsigned long long ps = 1;
    unsigned long long x = 2;
    while (ps < n) {
        if (n % x == 0 && is_prime(x)) {
            factors[count++] = x;
            ps *= x;
        }
        x++;
    }
    printf("%llu\n", factors[count - 1]);
    return 0;
}
