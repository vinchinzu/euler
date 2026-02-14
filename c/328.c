/*
 * Project Euler Problem 328 - Lowest-cost Search
 *
 * C(n) = minimum worst-case cost to find a number in {1..n}.
 * Find sum_{n=1}^{200000} C(n).
 *
 * Uses optimized (k, s) parameter tracking.
 */
#include <stdio.h>
#include <string.h>

#define N 200000

long long C_arr[N + 1];

static int popcount_ll(long long x) {
    int c = 0;
    while (x) { c += x & 1; x >>= 1; }
    return c;
}

int main(void) {
    memset(C_arr, 0, sizeof(C_arr));

    int k = 0;
    long long s = 0;
    long long right_cost = 1;
    long long ans = 0;

    for (int n = 2; n <= N; n++) {
        C_arr[n] = (long long)n - 1 + C_arr[n - 2];

        long long guess = (long long)n - 2 * ((1LL << k) + s) - 1;
        if (guess > 0) {
            long long cost = guess + (C_arr[guess - 1] > right_cost ? C_arr[guess - 1] : right_cost);
            if (cost < C_arr[n]) C_arr[n] = cost;
        }

        /* Count trailing ones in s */
        int num_ending_ones = 0;
        long long tmp = s;
        while (tmp & 1) {
            num_ending_ones++;
            tmp >>= 1;
        }

        int next_k;
        long long next_s, next_right_cost;

        if (num_ending_ones == k) {
            next_k = k + 1;
            next_s = (next_k == 1) ? 1 : 3;
            next_right_cost = (long long)(next_k + 1) * n
                - ((long long)next_k << (next_k + 1))
                + next_k
                + (next_k == 1 ? -1 : 3);
        } else {
            next_k = k;
            int num_remaining_ones = popcount_ll(s & ~((1LL << num_ending_ones) - 1));
            if (num_ending_ones < num_remaining_ones + 3) {
                next_s = s + (1LL << num_ending_ones);
                next_right_cost = right_cost + (long long)(num_ending_ones - num_remaining_ones) * (1LL << (num_ending_ones + 1));
            } else {
                next_s = s + (1LL << (num_remaining_ones + 3));
                next_right_cost = right_cost + 3LL * (1LL << (num_ending_ones + 1));
            }
        }

        long long next_guess = (long long)n - 2 * ((1LL << next_k) + next_s) - 1;
        if (next_guess > 0) {
            long long next_total_cost = next_guess + (C_arr[next_guess - 1] > next_right_cost ? C_arr[next_guess - 1] : next_right_cost);
            if (next_total_cost <= C_arr[n]) {
                k = next_k;
                s = next_s;
                right_cost = next_right_cost;
                C_arr[n] = next_total_cost;
            }
        }

        ans += C_arr[n];
        right_cost += k + 1;
    }

    printf("%lld\n", ans);
    return 0;
}
