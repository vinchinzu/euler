/*
 * Project Euler Problem 296 — Angular Bisector and Tangent.
 *
 * Stern-Brocot tree traversal with N=100000.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdint.h>

int main(void) {
    const int N = 100000;
    const int L = N / 6;

    int64_t ans = 0;
    int old_p = 0, old_q = 1;
    int p = 1, q = L;

    while (p <= q) {
        int64_t K = N / (p + q);
        int64_t limit_k = ((int64_t)p * K) / (p + 2 * q);

        for (int64_t k = 1; k <= limit_k; k++) {
            /* max_y = floor((K - k) / 2) */
            int64_t max_y = (K - k) / 2;
            /* min_y = ceil(k * q / p) = (k * q + p - 1) / p */
            int64_t min_y = (k * q + p - 1) / p;
            int64_t term = max_y - min_y + 1;
            if (term > 0)
                ans += term;
        }

        int med = (L + old_q) / q;
        int new_p = med * p - old_p;
        int new_q = med * q - old_q;
        old_p = p;
        old_q = q;
        p = new_p;
        q = new_q;
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
