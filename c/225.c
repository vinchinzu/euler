/*
 * Project Euler Problem 225: Tribonacci non-divisors
 *
 * Find the 124th odd number that does not divide any Tribonacci number.
 */
#include <stdio.h>

int main(void) {
    int N = 124;
    int d = 1;
    int count = 0;

    while (count < N) {
        d += 2;
        int a = 1, b = 1, c = 1;
        int found_zero = 0;

        while (1) {
            int next = (a + b + c) % d;
            if (next == 0) {
                found_zero = 1;
                break;
            }
            a = b;
            b = c;
            c = next;
            if (a == 1 && b == 1 && c == 1) {
                break;
            }
        }

        if (!found_zero)
            count++;
    }

    printf("%d\n", d);
    return 0;
}
