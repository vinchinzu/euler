#include <stdio.h>

int divisor_count(int n) {
    int count = 0;
    for (int i = 1; i * i <= n; i++) {
        if (n % i == 0) count += 2;
    }
    /* Adjust for perfect square */
    int s = 1;
    while (s * s < n) s++;
    if (s * s == n) count--;
    return count;
}

int main(void) {
    int i = 1, tri = 1;
    while (divisor_count(tri) <= 500) { i++; tri += i; }
    printf("%d\n", tri);
    return 0;
}
