#include <stdio.h>
#include <stdbool.h>

bool next_permutation(int *arr, int len) {
    int i = len - 2;
    while (i >= 0 && arr[i] >= arr[i + 1]) i--;
    if (i < 0) return false;

    int j = len - 1;
    while (arr[j] <= arr[i]) j--;

    int tmp = arr[i]; arr[i] = arr[j]; arr[j] = tmp;

    int left = i + 1, right = len - 1;
    while (left < right) {
        tmp = arr[left]; arr[left] = arr[right]; arr[right] = tmp;
        left++; right--;
    }
    return true;
}

int main(void) {
    int digits[10] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9};
    int divisors[7] = {2, 3, 5, 7, 11, 13, 17};
    long long sum = 0;

    do {
        if (digits[0] == 0) continue;

        bool valid = true;
        for (int i = 0; i < 7; i++) {
            int num = digits[i + 1] * 100 + digits[i + 2] * 10 + digits[i + 3];
            if (num % divisors[i] != 0) {
                valid = false;
                break;
            }
        }

        if (valid) {
            long long num = 0;
            for (int i = 0; i < 10; i++) {
                num = num * 10 + digits[i];
            }
            sum += num;
        }
    } while (next_permutation(digits, 10));

    printf("%lld\n", sum);
    return 0;
}
