#include <stdio.h>

unsigned long long binomial(int n, int k) {
    unsigned long long res = 1;
    for (int i = 1; i <= k; i++) {
        res *= (n - k + i);
        res /= i;
    }
    return res;
}

int main(void) {
    int r = 20;
    printf("%llu\n", binomial(2 * r, r));
    return 0;
}
