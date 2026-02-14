/* Project Euler 348 - Sum of a Square and a Cube
 *
 * Find palindromic numbers that can be expressed as a^2 + b^3 (a,b > 1)
 * in exactly 4 different ways. Sum the five smallest such palindromes.
 */

#include <stdio.h>
#include <math.h>
#include <string.h>

#define MAX_VAL 1000000000LL

int is_palindrome(long long n) {
    char s[20];
    int len = sprintf(s, "%lld", n);
    for (int i = 0; i < len / 2; i++)
        if (s[i] != s[len - 1 - i]) return 0;
    return 1;
}

int count_representations(long long n) {
    int count = 0;
    for (long long b = 2; b * b * b < n; b++) {
        long long remainder = n - b * b * b;
        if (remainder <= 1) continue;
        long long a = (long long)sqrt((double)remainder);
        /* Adjust for floating point */
        while (a * a < remainder) a++;
        while (a * a > remainder) a--;
        if (a > 1 && a * a == remainder)
            count++;
    }
    return count;
}

int main(void) {
    /* Generate palindromes in sorted order and check each */
    long long results[5];
    int found = 0;

    /* Generate palindromes by constructing from half-digits */
    /* Single digits */
    for (int d = 1; d <= 9 && found < 5; d++) {
        if (d >= 28) { /* min is 2^2 + 2^3 = 12 */
            int ways = count_representations(d);
            if (ways == 4) results[found++] = d;
        }
    }

    for (int length = 2; found < 5; length++) {
        if (length % 2 == 0) {
            /* Even length palindromes */
            int half_len = length / 2;
            long long start = 1;
            for (int i = 1; i < half_len; i++) start *= 10;
            long long end = start * 10;

            for (long long half = start; half < end && found < 5; half++) {
                char hs[12], ps[24];
                int hlen = sprintf(hs, "%lld", half);
                memcpy(ps, hs, hlen);
                for (int i = 0; i < hlen; i++)
                    ps[hlen + i] = hs[hlen - 1 - i];
                ps[2 * hlen] = '\0';
                long long palindrome = 0;
                for (int i = 0; ps[i]; i++)
                    palindrome = palindrome * 10 + (ps[i] - '0');
                if (palindrome > MAX_VAL) break;
                if (palindrome < 28) continue;
                int ways = count_representations(palindrome);
                if (ways == 4)
                    results[found++] = palindrome;
            }
        } else {
            /* Odd length palindromes */
            int half_len = length / 2 + 1;
            long long start = 1;
            for (int i = 1; i < half_len; i++) start *= 10;
            long long end = start * 10;

            for (long long half = start; half < end && found < 5; half++) {
                char hs[12], ps[24];
                int hlen = sprintf(hs, "%lld", half);
                memcpy(ps, hs, hlen);
                for (int i = 0; i < hlen - 1; i++)
                    ps[hlen + i] = hs[hlen - 2 - i];
                ps[2 * hlen - 1] = '\0';
                long long palindrome = 0;
                for (int i = 0; ps[i]; i++)
                    palindrome = palindrome * 10 + (ps[i] - '0');
                if (palindrome > MAX_VAL) break;
                if (palindrome < 28) continue;
                int ways = count_representations(palindrome);
                if (ways == 4)
                    results[found++] = palindrome;
            }
        }
    }

    long long total = 0;
    for (int i = 0; i < 5; i++) total += results[i];
    printf("%lld\n", total);
    return 0;
}
