#include <stdio.h>

/*
 * Project Euler 866 - Number Caterpillar
 *
 * E[k] = (2k-1) * sum(E[i] * E[k-1-i] for i=0..k-1)
 * E[0] = 1
 * Find E[100] mod 987654319.
 */

#define MOD 987654319LL

int main(void) {
    int n = 100;
    long long E[101];
    E[0] = 1;

    for (int k = 1; k <= n; k++) {
        long long sum_val = 0;
        for (int i = 0; i < k; i++) {
            sum_val = (sum_val + E[i] * E[k - 1 - i]) % MOD;
        }
        long long factor = (2LL * k - 1) % MOD;
        E[k] = (factor * sum_val) % MOD;
    }

    printf("%lld\n", E[n]);
    return 0;
}
