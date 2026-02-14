/*
 * Project Euler 034 - Digit Factorials
 * Find the sum of all numbers which are equal to the sum of the
 * factorial of their digits.
 */
#include <stdio.h>

int main(void) {
    /* Precompute factorials 0! to 9! */
    long long factorials[10];
    factorials[0] = 1;
    for (int i = 1; i < 10; i++) {
        factorials[i] = factorials[i - 1] * i;
    }

    /* Find upper limit: max_digits * 9! must be >= 10^(max_digits-1) */
    /* 7 * 362880 = 2540160, which is 7 digits. 8 * 362880 = 2903040, also 7 digits.
       So upper limit is about 2540160. */
    long long upper_limit = 7 * factorials[9];

    long long result = 0;
    for (long long n = 10; n <= upper_limit; n++) {
        long long sum = 0;
        long long temp = n;
        while (temp > 0) {
            sum += factorials[temp % 10];
            temp /= 10;
        }
        if (sum == n) {
            result += n;
        }
    }

    printf("%lld\n", result);
    return 0;
}
