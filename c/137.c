/* Project Euler Problem 137: Fibonacci golden nuggets.
 *
 * The 15th golden nugget = (L_61 - 1) / 5, where L_n is the nth Lucas number.
 * We need big integers since L_61 is large. Use __int128.
 * Actually L_61 is about 3.2 * 10^12, fits in long long.
 */
#include <stdio.h>

int main(void) {
    /* Compute Lucas number L_61 */
    /* L_0 = 2, L_1 = 1, L_n = L_{n-1} + L_{n-2} */
    long long a = 2, b = 1;
    for (int i = 2; i <= 61; i++) {
        long long t = a + b;
        a = b;
        b = t;
    }
    /* b = L_61 */
    long long result = (b - 1) / 5;
    printf("%lld\n", result);
    return 0;
}
