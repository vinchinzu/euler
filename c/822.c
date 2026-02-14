/*
 * Project Euler 822: Square the Smallest
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
#define K 10000
#define M 1234567891LL

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

typedef struct {
    double log_val;
    ll mod_val;
    int original;
} Number;

int cmp_numbers(const void *a, const void *b) {
    const Number *na = (const Number *)a;
    const Number *nb = (const Number *)b;
    if (na->log_val < nb->log_val) return -1;
    if (na->log_val > nb->log_val) return 1;
    if (na->original < nb->original) return -1;
    if (na->original > nb->original) return 1;
    return 0;
}

int main(void) {
    ll N = 10000000000000000LL; /* 10^16 */
    int sz = K - 1; /* numbers from 2 to K */

    Number *nums = (Number *)malloc((size_t)sz * sizeof(Number));
    if (!nums) return 1;

    for (int i = 0; i < sz; i++) {
        int v = i + 2;
        nums[i].log_val = log((double)v);
        nums[i].mod_val = (ll)v;
        nums[i].original = v;
    }

    qsort(nums, (size_t)sz, sizeof(Number), cmp_numbers);

    ll T = N;
    while (T % (ll)sz != 0 || nums[0].log_val * 2.0 < nums[sz - 1].log_val) {
        /* Pop first (smallest), square it, push back, re-sort */
        Number first = nums[0];
        double new_log = first.log_val * 2.0;
        ll new_mod = pow_mod(first.mod_val, 2, M);

        /* Find insertion point via binary search */
        int lo = 0, hi = sz - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (nums[mid + 1].log_val < new_log ||
                (nums[mid + 1].log_val == new_log && nums[mid + 1].original < first.original)) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        /* Shift [1..lo] left by one position */
        for (int i = 0; i < lo; i++) {
            nums[i] = nums[i + 1];
        }
        nums[lo].log_val = new_log;
        nums[lo].mod_val = new_mod;
        nums[lo].original = first.original;

        T--;
    }

    ll ans = 0;
    ll exp = pow_mod(2, T / (ll)sz, M - 1);
    for (int i = 0; i < sz; i++) {
        ans = (ans + pow_mod(nums[i].mod_val, exp, M)) % M;
    }

    printf("%lld\n", ans);
    free(nums);
    return 0;
}
