/* Project Euler Problem 110: Diophantine Reciprocals II
   Find smallest n with > 4,000,000 solutions to 1/x + 1/y = 1/n */
#include <stdio.h>

static int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47};
static int num_primes = 15;

/* Use __int128 for large n values */
typedef unsigned __int128 u128;

static u128 best;

static void search(int idx, int limit_exp, u128 current_n, long long divisor_count, long long target) {
    if (divisor_count > target) {
        if (current_n < best)
            best = current_n;
        return;
    }
    if (idx >= num_primes) return;

    u128 p = primes[idx];
    u128 value = current_n * p;
    int exp = 1;

    while (exp <= limit_exp) {
        if (value >= best) break;
        long long new_div = divisor_count * (2 * exp + 1);
        search(idx + 1, exp, value, new_div, target);
        exp++;
        value *= p;
    }
}

static void print_u128(u128 x) {
    if (x == 0) { printf("0"); return; }
    char buf[50];
    int pos = 0;
    while (x > 0) {
        buf[pos++] = '0' + (int)(x % 10);
        x /= 10;
    }
    for (int i = pos - 1; i >= 0; i--)
        putchar(buf[i]);
}

int main(void) {
    long long threshold = 4000000LL;
    long long target_divisors = 2 * threshold - 1;
    best = (u128)1 << 120;
    search(0, 20, 1, 1, target_divisors);
    print_u128(best);
    printf("\n");
    return 0;
}
