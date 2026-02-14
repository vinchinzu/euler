/*
 * Project Euler 206: Concealed Square
 *
 * Find the unique positive integer whose square is of the form 1_2_3_4_5_6_7_8_9_0.
 * Since it ends in 0, n = 10k, and n/10 must end in 3 or 7.
 */
#include <stdio.h>
#include <math.h>

static int matches(long long sq) {
    /* Check pattern: 1_2_3_4_5_6_7_8_9_0 (19 digits) */
    /* Check from right: positions 18,16,14,...,0 should be 0,9,8,...,1 */
    int digits[] = {0, 9, 8, 7, 6, 5, 4, 3, 2, 1};
    for (int i = 0; i < 10; i++) {
        if (sq % 10 != digits[i]) return 0;
        sq /= 100;
    }
    return (sq == 0);
}

int main(void) {
    long long lo = 1020304050607080900LL;
    long long hi = 1929394959697989990LL;

    long long n = (long long)sqrt((double)hi);
    /* Adjust down if needed */
    while (n * n > hi) n--;

    /* Align to end in 30 or 70 */
    long long r = n % 100;
    if (r > 70) n -= (r - 70);
    else if (r > 30) n -= (r - 30);
    else n -= (r + 30);

    long long lo_n = (long long)sqrt((double)lo);
    while (lo_n * lo_n < lo) lo_n++;

    while (n >= lo_n) {
        long long sq = n * n;
        if (matches(sq)) {
            printf("%lld\n", n);
            return 0;
        }
        if (n % 100 == 70)
            n -= 40;
        else
            n -= 60;
    }

    printf("0\n");
    return 0;
}
