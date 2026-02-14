/*
 * Project Euler Problem 704: Factors of Two in Binomial Coefficients.
 *
 * F(n) = floor(log2(n)) if n is even, F(n) = F(floor(n/2)) if n is odd.
 * Sum F(n) for n = 1..N where N = 10^16.
 *
 * Iteratively: for each scale, add 1 for even numbers starting from 2, 4, 8, ...
 * then recurse on odd numbers via F(n) = F(floor(n/2)).
 */
#include <stdio.h>

typedef long long ll;

int main() {
    ll n = 10000000000000000LL; /* 10^16 */
    ll ans = 0;

    ll current_n = n;
    while (current_n > 1) {
        ll start = 2;
        while (start <= current_n) {
            ll count = (current_n - start) / 2 + 1;
            ans += count;
            start *= 2;
        }
        current_n = (current_n - 1) / 2;
    }

    printf("%lld\n", ans);
    return 0;
}
