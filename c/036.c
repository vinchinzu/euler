/*
 * Project Euler 036 - Double-base Palindromes
 * Find the sum of all numbers less than one million which are
 * palindromic in both base 10 and base 2.
 */
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

bool is_palindrome_str(const char *s, int len) {
    for (int i = 0; i < len / 2; i++) {
        if (s[i] != s[len - 1 - i]) return false;
    }
    return true;
}

bool is_decimal_palindrome(int n) {
    char buf[16];
    int len = sprintf(buf, "%d", n);
    return is_palindrome_str(buf, len);
}

bool is_binary_palindrome(int n) {
    if (n == 0) return true;
    char buf[32];
    int len = 0;
    int tmp = n;
    while (tmp > 0) {
        buf[len++] = '0' + (tmp & 1);
        tmp >>= 1;
    }
    /* buf is reversed, but palindrome check is symmetric */
    return is_palindrome_str(buf, len);
}

int main(void) {
    const int LIMIT = 1000000;
    long long sum = 0;

    for (int num = 1; num < LIMIT; num++) {
        if (is_decimal_palindrome(num) && is_binary_palindrome(num)) {
            sum += num;
        }
    }

    printf("%lld\n", sum);
    return 0;
}
