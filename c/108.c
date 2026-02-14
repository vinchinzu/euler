/* Project Euler Problem 108: Diophantine Reciprocals I
   Find smallest n with > 1000 solutions to 1/x + 1/y = 1/n */
#include <stdio.h>

static int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41};
static int num_primes = 13;
static long long best;

static void search(int idx, int limit_exp, long long current_n, long long divisor_count, long long target) {
    if (divisor_count > target) {
        if (current_n < best)
            best = current_n;
        return;
    }
    if (idx >= num_primes) return;

    long long p = primes[idx];
    long long value = current_n * p;
    int exp = 1;

    while (exp <= limit_exp) {
        if (value >= best) break;
        long long new_div = divisor_count * (2 * exp + 1);
        search(idx + 1, exp, value, new_div, target);
        exp++;
        value *= p;
    }
}

int main(void) {
    long long threshold = 1000;
    long long target_divisors = 2 * threshold - 1;
    best = 1LL << 60;
    search(0, 20, 1, 1, target_divisors);
    printf("%lld\n", best);
    return 0;
}
