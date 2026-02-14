/*
 * Project Euler 142 - Perfect Square Collection
 *
 * Find the smallest x+y+z where x>y>z>0 and x+y, x-y, x+z, x-z, y+z, y-z
 * are all perfect squares.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>

static bool is_square(long long n) {
    if (n < 0) return false;
    long long r = (long long)sqrt((double)n);
    while (r * r > n) r--;
    while ((r+1)*(r+1) <= n) r++;
    return r * r == n;
}

int main(void) {
    long long min_sum = -1;
    int limit_k5 = 1000;

    for (int k5 = 2; k5 <= limit_k5; k5 += 2) {
        for (int k6 = 2; k6 < k5; k6 += 2) {
            long long k5_sq = (long long)k5 * k5;
            long long k6_sq = (long long)k6 * k6;

            long long y_val = (k5_sq + k6_sq) / 2;
            long long z_val = (k5_sq - k6_sq) / 2;

            long long z2_val = 2 * z_val;

            int p_limit = (int)sqrt((double)z2_val);
            for (int p_factor = 2; p_factor <= p_limit; p_factor += 2) {
                if (z2_val % p_factor != 0) continue;
                long long q_factor = z2_val / p_factor;
                if (q_factor % 2 == 1) continue;
                if (p_factor >= q_factor) continue;

                long long k3 = (p_factor + q_factor) / 2;
                long long k4 = (q_factor - p_factor) / 2;

                long long x_val = (k3 * k3 + k4 * k4) / 2;

                long long k1_sq_val = x_val + y_val;
                long long k2_sq_val = x_val - y_val;

                if (k2_sq_val <= 0) continue;

                if (!is_square(k1_sq_val)) continue;
                if (!is_square(k2_sq_val)) continue;

                long long k1 = (long long)sqrt((double)k1_sq_val);
                while (k1*k1 < k1_sq_val) k1++;
                long long k2 = (long long)sqrt((double)k2_sq_val);
                while (k2*k2 < k2_sq_val) k2++;

                if ((k1 % 2) != (k2 % 2)) continue;
                if ((k1 % 2) != (x_val % 2)) continue;

                long long sum_val = x_val + y_val + z_val;
                if (min_sum < 0 || sum_val < min_sum) {
                    min_sum = sum_val;
                }
            }
        }
    }
    printf("%lld\n", min_sum);
    return 0;
}
