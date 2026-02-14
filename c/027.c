/*
 * Project Euler 027 - Quadratic Primes
 * Find the product of coefficients a and b for n^2 + an + b that produces
 * the maximum number of primes for consecutive values of n.
 */
#include <stdio.h>
#include <stdbool.h>

bool is_prime(int n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int count_consecutive_primes(int a, int b) {
    int n = 0;
    while (true) {
        int val = n * n + a * n + b;
        if (val < 0) val = -val; /* is_prime needs positive */
        if (!is_prime(n * n + a * n + b > 0 ? n * n + a * n + b : 0)) break;
        n++;
    }
    return n;
}

int main(void) {
    /* Collect primes up to 1000 for b values */
    int primes_b[200];
    int primes_b_count = 0;
    for (int i = 2; i <= 1000; i++) {
        if (is_prime(i)) {
            primes_b[primes_b_count++] = i;
        }
    }

    int max_count = 0;
    int best_product = 0;

    for (int a = -999; a <= 999; a++) {
        for (int bi = 0; bi < primes_b_count; bi++) {
            int b = primes_b[bi];
            int n = 0;
            while (true) {
                int val = n * n + a * n + b;
                if (!is_prime(val)) break;
                n++;
            }
            if (n > max_count) {
                max_count = n;
                best_product = a * b;
            }
        }
    }

    printf("%d\n", best_product);
    return 0;
}
