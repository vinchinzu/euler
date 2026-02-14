#include <stdio.h>
#include <stdbool.h>

/* Returns true if C(n,k) > limit, with early exit to avoid overflow */
bool binomial_exceeds(int n, int k, long long limit) {
    if (k > n - k) k = n - k;
    long long result = 1;
    for (int i = 1; i <= k; i++) {
        result = result * (n - k + i) / i;
        if (result > limit) return true;
    }
    return result > limit;
}

int main(void) {
    int count = 0;
    for (int n = 1; n <= 100; n++) {
        for (int r = 0; r <= n; r++) {
            if (binomial_exceeds(n, r, 1000000)) {
                count++;
            }
        }
    }
    printf("%d\n", count);
    return 0;
}
