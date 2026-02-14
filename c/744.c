/* Project Euler Problem 744: What? Where? When?
 * Translated from python/744.py
 *
 * Simple closed-form: probability = 1 - 1/(2*(1-p))
 */
#include <stdio.h>

int main() {
    double p = 0.4999;
    double ans = 1.0 - 1.0 / 2.0 / (1.0 - p);
    printf("%.10f\n", ans);
    return 0;
}
