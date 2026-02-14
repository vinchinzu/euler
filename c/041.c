#include <stdio.h>
#include <stdbool.h>

bool is_prime(int n) {
    if (n < 2) return false;
    if (n == 2 || n == 3) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    for (int i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return false;
    }
    return true;
}

bool is_pandigital(int num, int n) {
    /* Check if num is 1-to-n pandigital */
    int digits[10] = {0};
    int count = 0;
    int tmp = num;
    while (tmp > 0) {
        int d = tmp % 10;
        if (d == 0 || d > n) return false;
        if (digits[d]) return false;
        digits[d] = 1;
        count++;
        tmp /= 10;
    }
    if (count != n) return false;
    for (int i = 1; i <= n; i++) {
        if (!digits[i]) return false;
    }
    return true;
}

/* Generate next permutation in descending order (prev_permutation) */
bool prev_permutation(int *arr, int len) {
    /* Find largest i such that arr[i] > arr[i+1] */
    int i = len - 2;
    while (i >= 0 && arr[i] <= arr[i + 1]) i--;
    if (i < 0) return false;

    /* Find largest j such that arr[j] < arr[i] */
    int j = len - 1;
    while (arr[j] >= arr[i]) j--;

    /* Swap */
    int tmp = arr[i]; arr[i] = arr[j]; arr[j] = tmp;

    /* Reverse from i+1 to end */
    int left = i + 1, right = len - 1;
    while (left < right) {
        tmp = arr[left]; arr[left] = arr[right]; arr[right] = tmp;
        left++; right--;
    }
    return true;
}

int main(void) {
    int largest_prime = 0;

    for (int n = 7; n >= 1; n--) {
        int digits[10];
        /* Fill digits n, n-1, ..., 1 (descending) */
        for (int i = 0; i < n; i++) {
            digits[i] = n - i;
        }

        do {
            int num = 0;
            for (int i = 0; i < n; i++) {
                num = num * 10 + digits[i];
            }

            if (is_pandigital(num, n) && is_prime(num)) {
                largest_prime = num;
                break;
            }
        } while (prev_permutation(digits, n));

        if (largest_prime > 0) break;
    }

    printf("%d\n", largest_prime);
    return 0;
}
