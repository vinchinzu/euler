#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>

int main(void) {
    int limit = 2000000;
    bool *is_prime = calloc(limit + 1, sizeof(bool));
    if (!is_prime) return 1;
    for (int i = 2; i <= limit; i++) is_prime[i] = true;
    for (int i = 2; i <= (int)sqrt((double)limit); i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= limit; j += i) {
                is_prime[j] = false;
            }
        }
    }
    unsigned long long sum = 0;
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) sum += i;
    }
    free(is_prime);
    printf("%llu\n", sum);
    return 0;
}
