/*
 * Project Euler 032 - Pandigital Products
 * Find the sum of all products whose multiplicand/multiplier/product
 * identity can be written as a 1 through 9 pandigital.
 */
#include <stdio.h>
#include <stdbool.h>

/* Generate next permutation in-place. Returns false if no more. */
bool next_permutation(int *arr, int n) {
    /* Find largest i such that arr[i] < arr[i+1] */
    int i = n - 2;
    while (i >= 0 && arr[i] >= arr[i + 1]) i--;
    if (i < 0) return false;

    /* Find largest j > i such that arr[j] > arr[i] */
    int j = n - 1;
    while (arr[j] <= arr[i]) j--;

    /* Swap */
    int tmp = arr[i]; arr[i] = arr[j]; arr[j] = tmp;

    /* Reverse from i+1 to end */
    int left = i + 1, right = n - 1;
    while (left < right) {
        tmp = arr[left]; arr[left] = arr[right]; arr[right] = tmp;
        left++; right--;
    }
    return true;
}

int main(void) {
    int digits[9] = {1, 2, 3, 4, 5, 6, 7, 8, 9};

    /* Collect unique products using a simple set (array + linear search) */
    int products[100];
    int product_count = 0;

    do {
        /* Split 1: a (1 digit), b (4 digits), product (4 digits) */
        int a1 = digits[0];
        int b1 = digits[1] * 1000 + digits[2] * 100 + digits[3] * 10 + digits[4];
        int c1 = digits[5] * 1000 + digits[6] * 100 + digits[7] * 10 + digits[8];

        if (a1 * b1 == c1) {
            /* Check if c1 already in products */
            bool found = false;
            for (int i = 0; i < product_count; i++) {
                if (products[i] == c1) { found = true; break; }
            }
            if (!found) products[product_count++] = c1;
        }

        /* Split 2: a (2 digits), b (3 digits), product (4 digits) */
        int a2 = digits[0] * 10 + digits[1];
        int b2 = digits[2] * 100 + digits[3] * 10 + digits[4];
        int c2 = digits[5] * 1000 + digits[6] * 100 + digits[7] * 10 + digits[8];

        if (a2 * b2 == c2) {
            bool found = false;
            for (int i = 0; i < product_count; i++) {
                if (products[i] == c2) { found = true; break; }
            }
            if (!found) products[product_count++] = c2;
        }
    } while (next_permutation(digits, 9));

    int sum = 0;
    for (int i = 0; i < product_count; i++) {
        sum += products[i];
    }

    printf("%d\n", sum);
    return 0;
}
