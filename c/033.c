/*
 * Project Euler 033 - Digit Cancelling Fractions
 * Find the denominator of the product of the four "curious" fractions
 * in lowest terms.
 */
#include <stdio.h>
#include <math.h>

int gcd(int a, int b) {
    while (b != 0) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a;
}

int main(void) {
    int num_product = 1;
    int den_product = 1;

    for (int n = 10; n <= 98; n++) {
        for (int d = n + 1; d <= 99; d++) {
            if (n % 10 == 0 && d % 10 == 0) continue;

            int n1 = n / 10, n2 = n % 10;
            int d1 = d / 10, d2 = d % 10;

            float simplified = 0.0f;

            /* Case 1: n's first digit equals d's second digit */
            if (n1 == d2 && d1 != 0 && n2 < d1) {
                simplified = (float)n2 / (float)d1;
            }
            /* Case 2: n's second digit equals d's first digit */
            else if (n2 == d1 && d2 != 0 && n1 < d2) {
                simplified = (float)n1 / (float)d2;
            }
            else {
                continue;
            }

            /* Check if fractions are equal */
            if (fabsf((float)n / (float)d - simplified) < 1e-6f) {
                num_product *= n;
                den_product *= d;
            }
        }
    }

    int common_divisor = gcd(num_product, den_product);
    printf("%d\n", den_product / common_divisor);
    return 0;
}
