#include <stdio.h>

/*
 * Project Euler 872 - Rooted Trees
 *
 * f(n, k) = sum of node numbers along path from root to k in T_n.
 * The path follows the binary representation of (n - k).
 *
 * Find f(10^17, 9^17).
 */

typedef unsigned long long ull;
typedef __int128 u128;

int main(void) {
    /* Compute 10^17 and 9^17 */
    ull n = 1;
    for (int i = 0; i < 17; i++) n *= 10;

    ull k = 1;
    for (int i = 0; i < 17; i++) k *= 9;

    ull diff = n - k;

    /* total and current use __int128 since values can exceed 64 bits */
    u128 total = k;
    u128 current = k;

    /* Find highest power of 2 <= diff */
    ull power = 1;
    while (power <= diff) power <<= 1;
    power >>= 1;

    /* Add each power of 2 that appears in binary representation of diff */
    while (power > 0) {
        if (diff & power) {
            current += power;
            total += current;
        }
        power >>= 1;
    }

    /* Print __int128 result */
    /* The result fits in a ull since the sum of ~60 terms each <= 10^17 */
    /* Actually it could be larger. Let's print carefully. */
    if (total == 0) {
        printf("0\n");
    } else {
        /* For __int128, manually convert to string */
        char buf[50];
        int pos = 0;
        u128 tmp = total;
        while (tmp > 0) {
            buf[pos++] = '0' + (int)(tmp % 10);
            tmp /= 10;
        }
        for (int i = pos - 1; i >= 0; i--) {
            putchar(buf[i]);
        }
        putchar('\n');
    }

    return 0;
}
