#include <stdio.h>
#include <stdlib.h>

#define K 1000000
#define N 1000000000L

void find_divisors(long prod, long *divisors, int *count) {
    *count = 0;
    for (long d = 1; d * d <= prod; d++) {
        if (prod % d == 0) {
            divisors[(*count)++] = d;
            if (d != prod / d) {
                divisors[(*count)++] = prod / d;
            }
        }
    }
}

int main() {
    long ans = 0;

    for (int k = 1; k <= K; k++) {
        long prod = 2L * k * k;

        long *divisors = malloc(100000 * sizeof(long));
        int div_count = 0;
        find_divisors(prod, divisors, &div_count);

        for (int i = 0; i < div_count; i++) {
            long d = divisors[i];

            // Case 1
            long a_plus_b = -(k + d);
            long b_plus_c = prod / d + k;

            long min_b = (a_plus_b + 1) / 2;
            if (b_plus_c - N > min_b) min_b = b_plus_c - N;

            long max_b = (b_plus_c - 1) / 2;
            if (a_plus_b + N < max_b) max_b = a_plus_b + N;

            if (min_b <= max_b) {
                ans += max_b - min_b + 1;
            }

            if (prod % (d + 2L * k) == 0) {
                long a_plus_c = prod / (d + 2L * k) - k;
                long sum = a_plus_b + b_plus_c + a_plus_c;

                if (sum % 2 == 0) {
                    long a = sum / 2 - b_plus_c;
                    long c = sum / 2 - a_plus_b;
                    if (a >= -N && c <= N) ans -= 2;
                }
            }

            // Case 2
            a_plus_b = k - d;
            b_plus_c = prod / d - k;

            min_b = b_plus_c / 2 + 1;
            if (b_plus_c - N > min_b) min_b = b_plus_c - N;

            max_b = N;
            if (a_plus_b + N < max_b) max_b = a_plus_b + N;

            if (min_b <= max_b) {
                ans += 2 * (max_b - min_b + 1);
            }

            if (d != 2L * k && prod % (2L * k - d) == 0) {
                long a_plus_c = k - prod / (2L * k - d);
                long sum = a_plus_b + b_plus_c + a_plus_c;

                if (sum % 2 == 0) {
                    long a = sum / 2 - b_plus_c;
                    long b = sum / 2 - a_plus_c;
                    long c = sum / 2 - a_plus_b;
                    if (a >= -N && c < b && b <= N) ans--;
                }
            }
        }
        free(divisors);
    }

    printf("%ld\n", ans);
    return 0;
}
