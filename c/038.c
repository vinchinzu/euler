/*
 * Project Euler 038 - Pandigital Multiples
 * Find the largest 1 to 9 pandigital 9-digit number that can be formed
 * as the concatenated product of an integer with (1,2,...,n) where n > 1.
 */
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

bool is_pandigital(const char *s) {
    if (strlen(s) != 9) return false;
    int digits[10] = {0};
    for (int i = 0; i < 9; i++) {
        int d = s[i] - '0';
        if (d == 0 || digits[d]) return false;
        digits[d] = 1;
    }
    return true;
}

int main(void) {
    const int MAX_K = 9999;
    long long max_pandigital = 0;

    for (int k = 2; k <= MAX_K; k++) {
        char result[20] = "";
        int total_len = 0;

        for (int n = 1; n <= 20; n++) {
            char buf[16];
            int len = sprintf(buf, "%d", k * n);

            if (total_len + len > 9) break;

            strcat(result, buf);
            total_len += len;

            if (total_len == 9) {
                if (is_pandigital(result)) {
                    long long val = 0;
                    for (int i = 0; i < 9; i++) {
                        val = val * 10 + (result[i] - '0');
                    }
                    if (val > max_pandigital) {
                        max_pandigital = val;
                    }
                }
                break;
            }
        }
    }

    printf("%lld\n", max_pandigital);
    return 0;
}
