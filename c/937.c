/* Project Euler Problem 937 - Factorial Partitions
 * G(n) = sum of k! for k=1..n where k%3 != 2, mod 10^9+7
 */
#include <stdio.h>

typedef long long ll;
#define MOD 1000000007LL

int main(void) {
    ll n = 100000000LL; /* 10^8 */
    ll total_sum = 0;
    ll current_factorial = 1;

    for (ll k = 1; k <= n; k++) {
        current_factorial = current_factorial * (k % MOD) % MOD;
        if (k % 3 != 2) {
            total_sum = (total_sum + current_factorial) % MOD;
        }
    }

    printf("%lld\n", total_sum);
    return 0;
}
