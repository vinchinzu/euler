/* Project Euler Problem 103: Special Sum Sets */
#include <stdio.h>
#include <stdbool.h>

static int set[7] = {20, 31, 38, 39, 40, 42, 45};

static bool is_special_sum_set(void) {
    int n = 7;
    /* Iterate through all 3^n assignments: 0=neither, 1=B, 2=C */
    int assignments[7] = {0};

    /* Total 3^7 = 2187 assignments. Use iterative approach. */
    int total = 1;
    for (int i = 0; i < n; i++) total *= 3;

    for (int t = 0; t < total; t++) {
        int tmp = t;
        for (int i = 0; i < n; i++) {
            assignments[i] = tmp % 3;
            tmp /= 3;
        }

        int sum_b = 0, sum_c = 0, size_b = 0, size_c = 0;
        for (int i = 0; i < n; i++) {
            if (assignments[i] == 1) { sum_b += set[i]; size_b++; }
            else if (assignments[i] == 2) { sum_c += set[i]; size_c++; }
        }

        if (size_b == 0 || size_c == 0) continue;

        if (sum_b == sum_c) return false;
        if (size_b > size_c && sum_b <= sum_c) return false;
        if (size_c > size_b && sum_c <= sum_b) return false;
    }
    return true;
}

int main(void) {
    if (is_special_sum_set()) {
        for (int i = 0; i < 7; i++)
            printf("%d", set[i]);
        printf("\n");
    }
    return 0;
}
