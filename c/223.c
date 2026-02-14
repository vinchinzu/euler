/*
 * Project Euler Problem 223: Almost right-angled triangles I
 *
 * Count barely acute triangles (a <= b <= c, a^2 + b^2 = c^2 + 1)
 * with perimeter <= 25,000,000 using ternary tree generation.
 */
#include <stdio.h>

int main(void) {
    long N = 25000000L;
    long stack[30000]; /* 3 values per entry, max depth ~3000 */
    int top = 0;

    /* Seed 1: (1, 1, 1) */
    stack[top++] = 1; stack[top++] = 1; stack[top++] = 1;
    /* Seed 2: (1, 2, 2) */
    stack[top++] = 1; stack[top++] = 2; stack[top++] = 2;

    long ans = 0;

    while (top > 0) {
        long c = stack[--top];
        long b = stack[--top];
        long a = stack[--top];
        if (a + b + c <= N) {
            ans++;
            /* Child 1 */
            stack[top++] = a - 2*b + 2*c;
            stack[top++] = 2*a - b + 2*c;
            stack[top++] = 2*a - 2*b + 3*c;
            /* Child 2 (only if a != b) */
            if (a != b) {
                stack[top++] = -a + 2*b + 2*c;
                stack[top++] = -2*a + b + 2*c;
                stack[top++] = -2*a + 2*b + 3*c;
            }
            /* Child 3 */
            stack[top++] = a + 2*b + 2*c;
            stack[top++] = 2*a + b + 2*c;
            stack[top++] = 2*a + 2*b + 3*c;
        }
    }

    printf("%ld\n", ans);
    return 0;
}
