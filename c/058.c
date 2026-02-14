#include <stdio.h>
#include <stdbool.h>

bool is_prime(long long n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (long long i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int main(void) {
    int prime_count = 0;
    int total_diagonals = 1;
    int side_length = 1;

    while (1) {
        side_length += 2;

        long long br = (long long)side_length * side_length;
        long long bl = br - (side_length - 1);
        long long tl = bl - (side_length - 1);
        long long tr = tl - (side_length - 1);

        if (is_prime(bl)) prime_count++;
        if (is_prime(tl)) prime_count++;
        if (is_prime(tr)) prime_count++;

        total_diagonals += 4;

        double ratio = (double)prime_count / total_diagonals;
        if (ratio < 0.10) {
            printf("%d\n", side_length);
            break;
        }
    }

    return 0;
}
