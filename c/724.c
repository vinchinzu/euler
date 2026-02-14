/* Project Euler 724: Drone Delivery -- harmonic sums. */
#include <stdio.h>
#include <stdint.h>

int main() {
    int n = 100000000;
    double h = 0.0, ans = 0.0;
    for (int i = 1; i <= n; i++) {
        h += 1.0 / i;
        ans += h / i;
    }
    printf("%lld\n", (long long)(ans * n));
    return 0;
}
