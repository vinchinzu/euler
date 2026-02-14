/*
 * Project Euler 024 - Lexicographic Permutations
 * Find the millionth lexicographic permutation of digits 0-9.
 */
#include <stdio.h>

int main(void) {
    int available[10] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int avail_count = 10;
    int result[10];
    int n = 1000000 - 1; /* zero-based */

    for (int i = 9; i >= 0; i--) {
        /* Compute i! */
        long long fact = 1;
        for (int j = 1; j <= i; j++) {
            fact *= j;
        }

        int idx = n / fact;
        n %= fact;

        result[9 - i] = available[idx];

        /* Remove element at idx */
        for (int j = idx; j < avail_count - 1; j++) {
            available[j] = available[j + 1];
        }
        avail_count--;
    }

    for (int i = 0; i < 10; i++) {
        printf("%d", result[i]);
    }
    printf("\n");
    return 0;
}
