/*
 * Project Euler Problem 387: Strong right truncatable Harshad primes
 *
 * Find the sum of strong, right truncatable Harshad primes below 10^14.
 */
#include <stdio.h>
#include <string.h>

#define LIMIT 100000000000000LL  /* 10^14 */
#define MAX_M 10000000000000LL   /* 10^13 */

static int is_prime(long long n) {
    if (n < 2) return 0;
    if (n == 2 || n == 3) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (long long i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

/* Queue for BFS */
typedef struct {
    long long num;
    int digit_sum;
} Entry;

#define QUEUE_SIZE 20000000
static Entry queue[QUEUE_SIZE];
static int qhead, qtail;

int main(void) {
    long long total_sum = 0;
    qhead = 0;
    qtail = 0;

    /* Initialize with single-digit Harshad numbers (1-9) */
    for (int d = 1; d <= 9; d++) {
        queue[qtail].num = d;
        queue[qtail].digit_sum = d;
        qtail++;
    }

    while (qhead < qtail) {
        long long num = queue[qhead].num;
        int digit_sum = queue[qhead].digit_sum;
        qhead++;

        /* Check if num is a strong Harshad number */
        if (digit_sum > 0 && num % digit_sum == 0 && is_prime(num / digit_sum)) {
            /* Try appending digits 0-9 to form a prime */
            for (int d = 0; d <= 9; d++) {
                long long candidate = num * 10 + d;
                if (candidate < LIMIT && is_prime(candidate)) {
                    total_sum += candidate;
                }
            }
        }

        /* Extend right-truncatable Harshad numbers */
        if (num < MAX_M) {
            for (int d = 0; d <= 9; d++) {
                long long new_num = num * 10 + d;
                int new_ds = digit_sum + d;
                if (new_ds > 0 && new_num % new_ds == 0) {
                    if (qtail < QUEUE_SIZE) {
                        queue[qtail].num = new_num;
                        queue[qtail].digit_sum = new_ds;
                        qtail++;
                    }
                }
            }
        }
    }

    printf("%lld\n", total_sum);
    return 0;
}
