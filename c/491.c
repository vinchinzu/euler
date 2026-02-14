/* Project Euler 491 - Double pandigital numbers divisible by 11
 * Translated from python/491.py
 *
 * Count double pandigital numbers (use each digit 0-9 exactly twice)
 * divisible by 11. Uses multinomial coefficients.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;

ll nCr(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k > n - k) k = n - k;
    ll result = 1;
    for (int i = 0; i < k; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

ll gnCr(int *counts, int len) {
    int total = 0;
    for (int i = 0; i < len; i++) total += counts[i];
    ll result = 1;
    for (int i = 0; i < len; i++) {
        result *= nCr(total, counts[i]);
        total -= counts[i];
    }
    return result;
}

int main() {
    int B = 10;
    ll ans = 0;

    /* Iterate over all possible distributions of digits in even positions */
    /* Each count is 0, 1, or 2 (since each digit appears exactly twice total) */
    /* 3^10 = 59049 combinations */
    int counts[10];

    for (int mask = 0; mask < 59049; mask++) {
        int tmp = mask;
        int num = 0;
        int sum_val = 0;
        for (int i = 0; i < B; i++) {
            counts[i] = tmp % 3;
            tmp /= 3;
            num += counts[i];
            sum_val += i * counts[i];
        }

        /* nCr(B, 2) = 45 */
        if (num == B && (45 - sum_val) % (B + 1) == 0) {
            ll res1 = gnCr(counts, B);
            int bs[10];
            for (int i = 0; i < B; i++) bs[i] = 2 - counts[i];
            ll res2 = gnCr(bs, B);
            if (bs[0] > 0) {
                bs[0]--;
                res2 -= gnCr(bs, B);
                bs[0]++;
            }
            ans += res1 * res2;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
