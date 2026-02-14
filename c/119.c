/* Project Euler Problem 119: Digit Power Sum */
#include <stdio.h>
#include <stdlib.h>

static int digit_sum(long long n) {
    int s = 0;
    while (n > 0) { s += (int)(n % 10); n /= 10; }
    return s;
}

static int cmp_ll(const void *a, const void *b) {
    long long va = *(const long long*)a, vb = *(const long long*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

int main(void) {
    long long results[10000];
    int count = 0;
    long long limit = 1000000000000000000LL; /* 10^18 */

    for (int s = 2; s <= 200; s++) {
        long long power = (long long)s * s;
        for (int k = 2; k <= 60 && power < limit; k++) {
            if (power >= 10 && digit_sum(power) == s) {
                results[count++] = power;
            }
            if (power > limit / s) break;
            power *= s;
        }
    }

    qsort(results, count, sizeof(long long), cmp_ll);

    /* Remove duplicates */
    int unique = 0;
    for (int i = 0; i < count; i++) {
        if (i == 0 || results[i] != results[i - 1])
            results[unique++] = results[i];
    }

    printf("%lld\n", results[29]); /* a_30 (0-indexed) */
    return 0;
}
