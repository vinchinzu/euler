/*
 * Project Euler 277: A Modified Collatz sequence
 *
 * Find the smallest integer > 10^15 whose modified Collatz sequence
 * starts with the given string "UDDDUdddDDUDDddDdDddDDUDDdUUDd".
 *
 * Process string in reverse to build congruence.
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef __int128 i128;

int main(void) {
    const char *S = "UDDDUdddDDUDDddDdDddDDUDDdUUDd";
    long long N = 1000000000000000LL; /* 10^15 */
    int len = (int)strlen(S);

    i128 mod = 1;
    i128 ans = 0;

    for (int i = len - 1; i >= 0; i--) {
        mod *= 3;
        char c = S[i];
        if (c == 'D') {
            ans *= 3;
        } else if (c == 'U') {
            ans = 3 * ans - 2;
            while (ans % 4 != 0)
                ans += mod;
            ans /= 4;
        } else if (c == 'd') {
            ans = 3 * ans + 1;
            while (ans % 2 != 0)
                ans += mod;
            ans /= 2;
        }
    }

    while (ans <= (i128)N)
        ans += mod;

    /* Print __int128 */
    char buf[50];
    int pos = 0;
    i128 v = ans;
    while (v > 0) {
        buf[pos++] = '0' + (int)(v % 10);
        v /= 10;
    }
    for (int i = pos - 1; i >= 0; i--)
        putchar(buf[i]);
    putchar('\n');
    return 0;
}
