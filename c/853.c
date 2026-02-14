#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * Project Euler 853 - Pisano Period
 *
 * Find sum of all n < 10^9 for which pi(n) = 120.
 * Compute F_120 exactly (it's a ~25-digit number), find all divisors <= 10^9,
 * check which have Pisano period exactly 120.
 */

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

/* Fibonacci mod m using fast doubling */
void fib_mod(ll k, ll m, ll *fk, ll *fk1) {
    if (m == 1) { *fk = 0; *fk1 = 0; return; }
    if (k == 0) { *fk = 0; *fk1 = 1 % m; return; }

    ll a, b;
    fib_mod(k >> 1, m, &a, &b);
    ll c = (i128)a * (2 * b - a + m + m) % m;
    ll d = ((i128)a * a + (i128)b * b) % m;
    if (k & 1) {
        *fk = d;
        *fk1 = (c + d) % m;
    } else {
        *fk = c;
        *fk1 = d;
    }
}

/* Exact Fibonacci using __int128 (F_120 fits in 128 bits) */
void fib_exact(ll k, i128 *fk, i128 *fk1) {
    if (k == 0) { *fk = 0; *fk1 = 1; return; }

    i128 a, b;
    fib_exact(k >> 1, &a, &b);
    i128 c = a * (2 * b - a);
    i128 d = a * a + b * b;
    if (k & 1) {
        *fk = d;
        *fk1 = c + d;
    } else {
        *fk = c;
        *fk1 = d;
    }
}

/* Factorize a 128-bit number - trial division */
#define MAX_FACTORS 30
typedef struct {
    i128 prime;
    int exp;
} Factor;

int factorize128(i128 n, Factor *factors) {
    int cnt = 0;
    for (i128 d = 2; d * d <= n; d++) {
        if (n % d == 0) {
            factors[cnt].prime = d;
            factors[cnt].exp = 0;
            while (n % d == 0) {
                factors[cnt].exp++;
                n /= d;
            }
            cnt++;
        }
    }
    if (n > 1) {
        factors[cnt].prime = n;
        factors[cnt].exp = 1;
        cnt++;
    }
    return cnt;
}

/* Generate divisors of F_120 that are <= 10^9 */
ll divisors[5000000];
int n_divisors = 0;

void gen_divisors(int idx, int n_factors, Factor *factors, i128 current, ll limit) {
    if (current > limit) return;
    if (idx == n_factors) {
        divisors[n_divisors++] = (ll)current;
        return;
    }
    i128 pe = 1;
    for (int i = 0; i <= factors[idx].exp; i++) {
        gen_divisors(idx + 1, n_factors, factors, current * pe, limit);
        if (i < factors[idx].exp) {
            pe *= factors[idx].prime;
            if (current * pe > limit) break;
        }
    }
}

/* Divisors of 120 */
int divisors_120[] = {1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20, 24, 30, 40, 60};
int n_div120 = 15;

int has_pisano_exactly(ll n, int K) {
    if (n <= 0) return 0;
    if (n == 1) return (K == 1);

    ll fk, fk1;
    fib_mod(K, n, &fk, &fk1);
    if (fk != 0 || fk1 != 1) return 0;

    for (int i = 0; i < n_div120; i++) {
        int d = divisors_120[i];
        fib_mod(d, n, &fk, &fk1);
        if (fk == 0 && fk1 == 1) return 0;
    }
    return 1;
}

int main(void) {
    int K = 120;
    ll N = 1000000000LL;

    /* Compute F_120 exactly */
    i128 fk, fk1;
    fib_exact(K, &fk, &fk1);

    /* Factorize F_120 */
    Factor factors[MAX_FACTORS];
    int n_factors = factorize128(fk, factors);

    /* Generate all divisors <= N */
    n_divisors = 0;
    gen_divisors(0, n_factors, factors, 1, N);

    /* Check each candidate */
    ll total = 0;
    for (int i = 0; i < n_divisors; i++) {
        if (has_pisano_exactly(divisors[i], K)) {
            total += divisors[i];
        }
    }

    printf("%lld\n", total);
    return 0;
}
