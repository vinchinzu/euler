/*
 * Project Euler 637: Flexible Digit Sum
 *
 * Find sum of n in 1..10^7 where f(n, 10) == f(n, 3).
 * f(n, B) = minimum steps to reduce n to single digit in base B by
 * inserting addition signs.
 *
 * For N=10^7, f(n,B) <= 3 for all n and both B=10,3.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_VAL 10000000

int sum_digits_b(int n, int base) {
    int s = 0;
    while (n > 0) {
        s += n % base;
        n /= base;
    }
    return s;
}

/* Check if we can split n (in base b) into parts whose digit-sums (recursive)
 * result in a single digit in one more step.
 * "good(sum_val, remaining, base, sd)" checks if sum_val + (digits of remaining)
 * after splitting can be made single-digit.
 */
int good(int sum_val, int remaining, int base, int *sd) {
    if (remaining == 0) {
        return sd[sum_val] < base;
    }
    int pow_base = base;
    while (pow_base <= base * remaining) {
        if (good(sum_val + remaining % pow_base, remaining / pow_base, base, sd))
            return 1;
        if (pow_base > remaining) break;
        pow_base *= base;
    }
    return 0;
}

int *compute_f(int n, int b) {
    int *sd = (int *)malloc((n + 1) * sizeof(int));
    int *f_arr = (int *)malloc((n + 1) * sizeof(int));

    for (int i = 0; i <= n; i++) {
        sd[i] = sum_digits_b(i, b);
        if (i < b) {
            f_arr[i] = 0;
        } else if (sd[i] < b) {
            f_arr[i] = 1;
        } else if (good(0, i, b, sd)) {
            f_arr[i] = 2;
        } else {
            f_arr[i] = 3;
        }
    }
    free(sd);
    return f_arr;
}

int main() {
    int *f1 = compute_f(N_VAL, 10);
    int *f2 = compute_f(N_VAL, 3);

    long long ans = 0;
    for (int i = 1; i <= N_VAL; i++) {
        if (f1[i] == f2[i]) ans += i;
    }

    printf("%lld\n", ans);

    free(f1);
    free(f2);
    return 0;
}
