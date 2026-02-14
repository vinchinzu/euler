/*
 * Project Euler 268: Counting numbers with at least four distinct
 * prime factors less than 100.
 *
 * Inclusion-exclusion over subsets of primes < 100.
 * For each subset of size >= 4, weight = (-1)^(s-4) * C(s-1, 3).
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef long long ll;

static int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
                       53, 59, 61, 67, 71, 73, 79, 83, 89, 97};
static int num_primes = 25;

static ll nCr(int n, int r) {
    if (r < 0 || r > n) return 0;
    if (r > n / 2) r = n - r;
    ll result = 1;
    for (int i = 0; i < r; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

static ll ans = 0;

static void helper(int index, int s, ll prod, ll N) {
    if (index == num_primes) {
        if (s >= 4) {
            int parity = (s - 4) % 2 == 0 ? 1 : -1;
            ll weight = parity * nCr(s - 1, 3);
            ans += weight * (N / prod);
        }
        return;
    }

    /* Don't include this prime */
    helper(index + 1, s, prod, N);

    /* Include this prime (if product doesn't exceed N) */
    if (prod <= N / primes[index]) {
        helper(index + 1, s + 1, prod * primes[index], N);
    }
}

int main(void) {
    ll N = 10000000000000000LL; /* 10^16 */
    helper(0, 0, 1, N);
    printf("%lld\n", ans);
    return 0;
}
