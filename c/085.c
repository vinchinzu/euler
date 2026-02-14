#include <stdio.h>
#include <stdlib.h>

int main(void) {
    long long target = 2000000;
    long long closest_diff = target;
    int best_area = 0;

    for (int m = 1; m <= 2000; m++) {
        for (int n = 1; n <= m; n++) {
            long long count = (long long)m * (m + 1) / 2 * ((long long)n * (n + 1) / 2);
            long long diff = count > target ? count - target : target - count;
            if (diff < closest_diff) {
                closest_diff = diff;
                best_area = m * n;
            }
            if (count > target + closest_diff) break;
        }
    }

    printf("%d\n", best_area);
    return 0;
}
