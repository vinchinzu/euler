#include <stdio.h>
#include <stdbool.h>

bool is_prime(int n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int main(void) {
    /* For each odd composite, check if it can be written as prime + 2*square */
    for (int i = 3; i < 10000; i += 2) {
        if (is_prime(i)) continue;

        bool found = false;
        for (int s = 1; 2 * s * s < i; s++) {
            int remainder = i - 2 * s * s;
            if (remainder > 0 && is_prime(remainder)) {
                found = true;
                break;
            }
        }

        if (!found) {
            printf("%d\n", i);
            return 0;
        }
    }

    return 0;
}
