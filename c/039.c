/*
 * Project Euler 039 - Integer Right Triangles
 * For which value of p <= 1000 is the number of right triangle solutions maximised?
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    const int P = 1000;
    int counts[1001] = {0};

    for (int x = 1; x < P / 2; x++) {
        for (int y = x; y < P - x; y++) {
            double c_double = sqrt((double)x * x + (double)y * y);
            int c = (int)(c_double + 0.5);

            if (c * c == x * x + y * y) {
                int perimeter = x + y + c;
                if (perimeter <= P) {
                    counts[perimeter]++;
                }
            }
        }
    }

    int max_count = 0;
    int best_p = 0;

    for (int i = 1; i <= P; i++) {
        if (counts[i] > max_count) {
            max_count = counts[i];
            best_p = i;
        }
    }

    printf("%d\n", best_p);
    return 0;
}
