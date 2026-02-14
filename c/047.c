#include <stdio.h>

int count_distinct_prime_factors(int n) {
    int count = 0;

    if (n % 2 == 0) {
        count++;
        while (n % 2 == 0) n /= 2;
    }

    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) {
            count++;
            while (n % i == 0) n /= i;
        }
    }

    if (n > 1) count++;
    return count;
}

int main(void) {
    #define LIMIT 200000
    static int factor_counts[LIMIT + 1];

    for (int i = 2; i <= LIMIT; i++) {
        factor_counts[i] = count_distinct_prime_factors(i);
    }

    for (int i = 2; i <= LIMIT - 3; i++) {
        if (factor_counts[i] == 4 &&
            factor_counts[i + 1] == 4 &&
            factor_counts[i + 2] == 4 &&
            factor_counts[i + 3] == 4) {
            printf("%d\n", i);
            break;
        }
    }

    return 0;
}
