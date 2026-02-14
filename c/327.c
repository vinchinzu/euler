/*
 * Project Euler Problem 327 - Rooms of Doom
 *
 * M(C, R) = minimum cards needed to traverse R rooms with capacity C.
 * Find sum_{C=3}^{40} M(C, 30).
 *
 * Recurrence: if R < C, M = R+1. Else M(C,R) = k + (k-2)/(C-2)*2 + 1
 * where k = M(C, R-1).
 */
#include <stdio.h>

long long M_func(int C, int R) {
    if (R < C)
        return R + 1;
    long long k = M_func(C, R - 1);
    return k + (k - 2) / (C - 2) * 2 + 1;
}

int main(void) {
    long long ans = 0;
    for (int C = 3; C <= 40; C++) {
        ans += M_func(C, 30);
    }
    printf("%lld\n", ans);
    return 0;
}
