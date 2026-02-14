/*
 * Project Euler 030 - Digit Fifth Powers
 * Find the sum of all numbers that can be written as the sum of
 * fifth powers of their digits.
 */
#include <stdio.h>

int main(void) {
    /* Upper limit: for 6 digits, max sum is 6 * 9^5 = 354294 */
    const int MAX_N = 354294;

    /* Precompute fifth powers */
    int fifth[10];
    fifth[0] = 0;
    for (int i = 1; i < 10; i++) {
        fifth[i] = i * i * i * i * i;
    }

    long long total = 0;
    for (int i = 2; i <= MAX_N; i++) {
        int sum = 0;
        int temp = i;
        while (temp > 0) {
            sum += fifth[temp % 10];
            temp /= 10;
        }
        if (sum == i) {
            total += i;
        }
    }

    printf("%lld\n", total);
    return 0;
}
