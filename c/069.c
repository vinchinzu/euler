/* Project Euler 069 - Totient maximum */
#include <stdio.h>
#include <stdbool.h>

static bool is_prime(int n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int main(void) {
    int limit = 1000000;
    int result = 1;
    int p = 2;

    while (result * p <= limit) {
        result *= p;
        p++;
        while (!is_prime(p)) p++;
    }

    printf("%d\n", result);
    return 0;
}
