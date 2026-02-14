#include <stdio.h>
#include <math.h>

static int is_perfect_square(int n) {
    int root = (int)sqrt((double)n);
    return root * root == n;
}

int main(void) {
    int target = 1000000;
    int count = 0;

    for (int M = 1; count <= target; M++) {
        for (int sum = 2; sum <= 2 * M; sum++) {
            int dist_sq = M * M + sum * sum;
            if (is_perfect_square(dist_sq)) {
                int min_b = 1 > sum - M ? 1 : sum - M;
                int max_b = sum / 2;
                if (max_b >= min_b)
                    count += max_b - min_b + 1;
            }
        }
        if (count > target) {
            printf("%d\n", M);
            break;
        }
    }

    return 0;
}
