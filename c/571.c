/*
 * Project Euler Problem 571: Super Pandigital Numbers
 *
 * Find the sum of the smallest 10 numbers that are pandigital in all bases
 * from 2 to 12. Generate permutations of base-12 digits and check.
 */
#include <stdio.h>

#define BASE 12
#define K 10

static int count;
static long long ans;

static int is_pandigital_in_base11(long long n) {
    int used[11] = {0};
    while (n > 0) {
        used[n % 11] = 1;
        n /= 11;
    }
    for (int i = 0; i < 11; i++)
        if (!used[i]) return 0;
    return 1;
}

static int is_pandigital(long long n, int base) {
    int used[16] = {0};
    while (n > 0) {
        used[n % base] = 1;
        n /= base;
    }
    for (int i = 0; i < base; i++)
        if (!used[i]) return 0;
    return 1;
}

static void helper(int index, long long n, int *visited) {
    if (index == BASE) {
        if (!is_pandigital_in_base11(n)) return;
        for (int base = 2; base < BASE; base++) {
            if (base == 11) continue;
            if (!is_pandigital(n, base)) return;
        }
        count++;
        ans += n;
        return;
    }
    if (count == K) return;
    for (int i = 0; i < BASE; i++) {
        if (!visited[i]) {
            visited[i] = 1;
            helper(index + 1, n * BASE + i, visited);
            visited[i] = 0;
            if (count == K) return;
        }
    }
}

int main(void) {
    count = 0;
    ans = 0;
    int visited[BASE] = {0};
    helper(0, 0, visited);
    printf("%lld\n", ans);
    return 0;
}
