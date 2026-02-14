/* Project Euler Problem 131: Prime cube partnership.
 *
 * Primes of the form p = 3a^2 + 3a + 1 (consecutive cube differences).
 * Count how many such primes below 1,000,000.
 */
#include <stdio.h>
#include <stdbool.h>

static bool is_prime(int n) {
    if (n < 2) return false;
    if (n < 4) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    for (int i = 5; (long long)i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

int main(void) {
    int count = 0;
    for (int a = 1; ; a++) {
        int p = 3 * a * a + 3 * a + 1;
        if (p >= 1000000) break;
        if (is_prime(p)) count++;
    }
    printf("%d\n", count);
    return 0;
}
