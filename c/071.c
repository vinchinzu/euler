/* Project Euler 071 - Ordered fractions */
#include <stdio.h>

int main(void) {
    int limit_d = 1000000;

    /* Find the largest d <= limit_d such that d % 7 == 5 */
    int target_d = 0;
    for (int d = limit_d; d >= 1; d--) {
        if (d % 7 == 5) {
            target_d = d;
            break;
        }
    }

    int numerator_n = (3 * target_d - 1) / 7;
    printf("%d\n", numerator_n);
    return 0;
}
