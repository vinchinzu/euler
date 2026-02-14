#include <stdio.h>

/*
 * Project Euler 862 - Sum of T(n) for k-digit numbers
 *
 * S(k) = sum of T(n) for all k-digit numbers.
 * For each digit frequency tuple, count valid permutations
 * and add C(num_valid, 2).
 *
 * Find S(12).
 */

typedef long long ll;

static ll fact[13];
static int counts[10];
static ll total;
static int K;

void dfs(int pos, int rem) {
    if (pos == 9) {
        counts[9] = rem;

        ll denom = 1;
        for (int i = 0; i < 10; i++) {
            denom *= fact[counts[i]];
        }

        ll total_perms = fact[K] / denom;

        ll num_valid;
        int c0 = counts[0];
        if (c0 == 0) {
            num_valid = total_perms;
        } else {
            /* denom2 = (denom / fact[c0]) * fact[c0-1] = denom / c0 */
            ll denom2 = denom / c0;
            ll zero_first = fact[K - 1] / denom2;
            num_valid = total_perms - zero_first;
        }

        if (num_valid > 1) {
            total += num_valid * (num_valid - 1) / 2;
        }
        return;
    }

    for (int v = 0; v <= rem; v++) {
        counts[pos] = v;
        dfs(pos + 1, rem - v);
    }
}

int main(void) {
    K = 12;

    /* Precompute factorials */
    fact[0] = 1;
    for (int i = 1; i <= K; i++) {
        fact[i] = fact[i - 1] * i;
    }

    total = 0;
    dfs(0, K);

    printf("%lld\n", total);
    return 0;
}
