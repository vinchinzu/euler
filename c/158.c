/*
 * Project Euler 158 - Exploring strings with exactly one character
 * coming lexicographically after its left neighbour
 *
 * p(n) = C(26, n) * (2^n - n - 1)
 * Find max p(n) for n = 1..26.
 */
#include <stdio.h>

typedef long long ll;
typedef unsigned long long ull;

static ll binom(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k > n - k) k = n - k;
    ll num = 1, den = 1;
    for (int i = 1; i <= k; i++) {
        num *= (n - k + i);
        den *= i;
    }
    return num / den;
}

int main(void) {
    ll max_val = 0;
    for (int n = 2; n <= 26; n++) {
        ll euler = (1LL << n) - n - 1;
        ll val = binom(26, n) * euler;
        if (val > max_val) max_val = val;
    }
    printf("%lld\n", max_val);
    return 0;
}
