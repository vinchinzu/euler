/*
 * Project Euler 037 - Truncatable Primes
 * Find the sum of the only eleven primes that are both truncatable
 * from left to right and right to left.
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

/* Count digits */
int num_digits(int n) {
    int count = 0;
    while (n > 0) { count++; n /= 10; }
    return count;
}

/* Compute 10^n */
int power10(int n) {
    int r = 1;
    for (int i = 0; i < n; i++) r *= 10;
    return r;
}

bool is_left_truncatable(int n) {
    /* n is already known prime and > 10 */
    int len = num_digits(n);
    for (int i = 1; i < len; i++) {
        int truncated = n % power10(i);
        if (!is_prime(truncated)) return false;
    }
    return true;
}

bool is_right_truncatable(int n) {
    /* n is already known prime and > 10 */
    int temp = n / 10;
    while (temp > 0) {
        if (!is_prime(temp)) return false;
        temp /= 10;
    }
    return true;
}

int main(void) {
    int count = 0;
    int sum = 0;
    int num = 11;

    while (count < 11) {
        if (is_prime(num) && is_left_truncatable(num) && is_right_truncatable(num)) {
            sum += num;
            count++;
        }
        num += 2;
        if (num % 5 == 0) num += 2;
    }

    printf("%d\n", sum);
    return 0;
}
