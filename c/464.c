/*
 * Project Euler Problem 464: Mobius function and balanced pairs
 *
 * Count pairs 1 <= a <= b <= N where the counts of mu(n)=1 and mu(n)=-1
 * in [a,b] are approximately balanced: 99*N(a,b) <= 100*P(a,b) and vice versa.
 *
 * Uses Fenwick tree approach with prefix sum tracking.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define NN 20000000
#define K 100

static void pre_mobius(int limit, signed char *mu) {
    char *is_prime = (char *)calloc(limit + 1, 1);
    for (int i = 0; i <= limit; i++) {
        is_prime[i] = (i >= 2) ? 1 : 0;
        mu[i] = 1;
    }
    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j != i) is_prime[j] = 0;
                if ((long long)j % ((long long)i * i) == 0)
                    mu[j] = 0;
                else
                    mu[j] = -mu[j];
            }
        }
    }
    free(is_prime);
}

/* BIT (Fenwick tree) */
static long long *bit_tree;
static int bit_size;

static void bit_init(int size) {
    bit_size = size + 2;
    bit_tree = (long long *)calloc(bit_size + 1, sizeof(long long));
}

static void bit_free(void) {
    free(bit_tree);
}

static void bit_add(int idx, long long val) {
    idx++;
    while (idx <= bit_size) {
        bit_tree[idx] += val;
        idx += idx & (-idx);
    }
}

static long long bit_sum(int idx) {
    idx++;
    long long result = 0;
    while (idx > 0) {
        result += bit_tree[idx];
        idx -= idx & (-idx);
    }
    return result;
}

int main(void) {
    int L = K * (int)sqrt((double)NN);

    signed char *mu = (signed char *)calloc(NN + 1, 1);
    pre_mobius(NN, mu);

    long long ans = (long long)NN * (NN + 1) / 2;

    for (int sign = 1; sign >= -1; sign -= 2) {
        int f = 0;
        bit_init(NN + L);
        memset(bit_tree, 0, (bit_size + 1) * sizeof(long long));
        for (int b = 1; b <= NN; b++) {
            bit_add(f + L, 1);
            if (mu[b] == sign)
                f += K;
            else if (mu[b] == -sign)
                f -= K - 1;
            ans -= bit_sum(NN + L) - bit_sum(f + L);
        }
        bit_free();
    }

    free(mu);
    printf("%lld\n", ans);
    return 0;
}
