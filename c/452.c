/*
 * Project Euler 452 - Long products
 *
 * Extracted from python/452_helper.c.
 * Number of N-tuples of positive integers whose product <= N.
 */
#include <stdio.h>
#include <stdint.h>

#define N 1000000000LL
#define M 1234567891LL

static int L;
static int64_t prods[64];
static int64_t inv_fact[64];
static int64_t ans;

static int64_t mod_pow(int64_t base, int64_t exp) {
    int64_t res = 1 % M;
    base %= M;
    while (exp > 0) {
        if (exp & 1) res = (res * base) % M;
        base = (base * base) % M;
        exp >>= 1;
    }
    return res;
}

static void helper(int min_val, int64_t n, int prev, int num_elements, int64_t num_perm) {
    if (prev != 1) {
        ans = (ans + num_perm * prods[num_elements]) % M;
    }
    if ((int64_t)min_val <= N / n) {
        int64_t term = num_perm * prods[num_elements + 1] % M;
        term = term * ((N / n) - min_val + 1) % M;
        ans = (ans + term) % M;
    }

    int i = min_val;
    while ((int64_t)i * i <= N / n) {
        int count = 1;
        int64_t new_n = n * i;
        while (new_n <= N) {
            int64_t new_perm = num_perm * inv_fact[count] % M;
            helper(i + 1, new_n, count, num_elements + count, new_perm);
            count++;
            if (new_n > N / i) break;
            new_n *= i;
        }
        i++;
    }
}

int main(void) {
    L = 0;
    int64_t tmp = N;
    while (tmp > 0) {
        L++;
        tmp >>= 1;
    }
    L += 1;

    prods[0] = 1;
    for (int i = 1; i <= L; i++) {
        prods[i] = (prods[i - 1] * ((N + 1LL) - i)) % M;
    }

    int64_t fact = 1;
    for (int i = 1; i <= L; i++) {
        fact = (fact * i) % M;
    }
    inv_fact[L] = mod_pow(fact, M - 2);
    for (int i = L; i >= 1; i--) {
        inv_fact[i - 1] = (inv_fact[i] * i) % M;
    }

    ans = 0;
    helper(2, 1, 0, 0, 1);

    printf("%lld\n", ans % M);
    return 0;
}
