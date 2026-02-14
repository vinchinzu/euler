/*
 * Project Euler Problem 588: Quintinomial coefficients.
 *
 * Let Q(k) be the number of odd coefficients in (x^4+x^3+x^2+x+1)^k.
 * Find sum_{k=1}^N Q(10^k).
 *
 * Recursive approach using fractal structure of the Quintinomial Sierpinski triangle.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;

#define KK 5  /* quintinomial */
#define MAX_COUNTS (1 << (KK - 1))  /* 16 */
#define MAX_SMALL (2 * KK)          /* 10 */

/*
 * helper(n, size) returns an array of MAX_COUNTS values.
 * size is a power of 2 >= n.
 */
void helper(ll n, ll size, ll *counts) {
    int small_sections[MAX_SMALL];
    memset(small_sections, 0, sizeof(small_sections));
    ll sub_counts[MAX_COUNTS];

    if (n == 0) {
        memset(counts, 0, MAX_COUNTS * sizeof(ll));
        counts[0] = 0;
        counts[1] = 1;
        small_sections[0] = 1;
    } else if (n < size / 2) {
        helper(n, size / 2, sub_counts);
        memcpy(counts, sub_counts, MAX_COUNTS * sizeof(ll));
        for (int i = 0; i < KK - 1; i++) {
            small_sections[i] = 1 << i;
        }
    } else {
        helper(n - size / 2, size / 2, sub_counts);
        memcpy(counts, sub_counts, MAX_COUNTS * sizeof(ll));
        for (int i = 0; i < KK - 1; i++) {
            small_sections[i] = (1 << (i + 1)) - 1;
            small_sections[i + KK - 1] = (1 << (KK - 1)) - (1 << i);
        }
    }

    ll new_counts[MAX_COUNTS];
    memset(new_counts, 0, sizeof(new_counts));

    for (int subset = 0; subset < MAX_COUNTS; subset++) {
        int left_half = 0;
        int right_half = 0;
        for (int i = 0; i < KK - 1; i++) {
            if (subset & (1 << i)) {
                left_half ^= small_sections[2 * i];
                right_half ^= small_sections[2 * i + 1];
            }
        }
        new_counts[subset] = counts[left_half] + counts[right_half];
    }

    memcpy(counts, new_counts, MAX_COUNTS * sizeof(ll));
}

int main() {
    int N_max = 18;
    ll ans = 0;

    for (int k = 1; k <= N_max; k++) {
        /* Compute 10^k */
        ll n = 1;
        for (int i = 0; i < k; i++) n *= 10;

        /* size = 1 << 62 (large enough) */
        ll size = 1LL << 62;

        ll result[MAX_COUNTS];
        helper(n, size, result);

        ans += result[1];
    }

    printf("%lld\n", ans);
    return 0;
}
