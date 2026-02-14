#include <stdio.h>

int main(void) {
    long long limit = 1000000000LL;
    long long sum_of_perimeters = 0;

    /* Pell equation approach: X_{n+1} = 4*X_n - X_{n-1} */
    long long xk_minus_1 = 1;  /* X_0 */
    long long xk = 2;           /* X_1 */
    int current_k = 1;

    while (1) {
        long long xk_plus_1 = 4 * xk - xk_minus_1;
        int index = current_k + 1;

        long long perimeter;
        if (index % 2 == 0)
            perimeter = 2 * xk_plus_1 + 2;
        else
            perimeter = 2 * xk_plus_1 - 2;

        if (perimeter > limit)
            break;

        sum_of_perimeters += perimeter;

        xk_minus_1 = xk;
        xk = xk_plus_1;
        current_k = index;
    }

    printf("%lld\n", sum_of_perimeters);
    return 0;
}
