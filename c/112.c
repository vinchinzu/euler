/* Project Euler Problem 112: Bouncy Numbers */
#include <stdio.h>
#include <stdbool.h>

static bool is_bouncy(int n) {
    if (n < 100) return false;
    int digits[10], nd = 0;
    int tmp = n;
    while (tmp > 0) { digits[nd++] = tmp % 10; tmp /= 10; }
    /* digits are reversed (least significant first), reverse them */
    for (int i = 0; i < nd / 2; i++) {
        int t = digits[i]; digits[i] = digits[nd - 1 - i]; digits[nd - 1 - i] = t;
    }

    bool inc = true, dec = true;
    for (int i = 0; i < nd - 1; i++) {
        if (digits[i] > digits[i + 1]) inc = false;
        if (digits[i] < digits[i + 1]) dec = false;
    }
    return !inc && !dec;
}

int main(void) {
    int bouncy_count = 0;
    int n = 0;
    while (1) {
        n++;
        if (is_bouncy(n)) bouncy_count++;
        if (n % 100 == 0) {
            /* Check: bouncy_count * 100 == 99 * n */
            if ((long long)bouncy_count * 100 == 99LL * n) {
                printf("%d\n", n);
                return 0;
            }
        }
    }
}
