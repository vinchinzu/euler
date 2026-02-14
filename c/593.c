/*
 * Project Euler Problem 593: Fleeting Medians.
 *
 * Extracted from embedded C in Python solution.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef int64_t i64;

#define N 10000000
#define K 100000
#define M 10007
#define D 10000
#define MAX_VAL (2 * M + 1)

// Sieve for primes
int *ff;
int prime_count = 0;
int *primes;

void sieve(int limit) {
    ff = (int*)calloc(limit + 1, sizeof(int));
    primes = (int*)malloc((limit + 1) * sizeof(int));
    for (int i = 2; i <= limit; i++) {
        if (ff[i] == 0) {
            ff[i] = i;
            primes[prime_count++] = i;
            for (i64 j = (i64)i * i; j <= limit; j += i)
                if (ff[j] == 0) ff[j] = i;
        }
    }
}

int generator(int m) {
    int phi = m - 1;
    int factors[100], nf = 0;
    int temp = phi;
    for (int p = 2; p * p <= temp; p++) {
        if (temp % p == 0) {
            factors[nf++] = p;
            while (temp % p == 0) temp /= p;
        }
    }
    if (temp > 1) factors[nf++] = temp;

    for (int g = 2; g < m; g++) {
        int is_gen = 1;
        for (int i = 0; i < nf; i++) {
            i64 x = 1;
            for (int e = phi / factors[i]; e > 0; e--)
                x = x * g % m;
            if (x == 1) { is_gen = 0; break; }
        }
        if (is_gen) return g;
    }
    return 1;
}

// Fenwick tree for counting elements
int *bit;

void bit_update(int i, int delta) {
    for (; i < MAX_VAL; i += i & (-i))
        bit[i] += delta;
}

int bit_query(int i) {
    int sum = 0;
    for (; i > 0; i -= i & (-i))
        sum += bit[i];
    return sum;
}

// Find k-th smallest (1-indexed)
int bit_find_kth(int k) {
    int pos = 0;
    int sum = 0;
    for (int i = 14; i >= 0; i--) {  // log2(MAX_VAL) ~ 14
        int next = pos + (1 << i);
        if (next < MAX_VAL && sum + bit[next] < k) {
            sum += bit[next];
            pos = next;
        }
    }
    return pos + 1;
}

int main() {
    // p_{10^7} ~ 179424673, need limit around 200 million
    int limit = 200000000;
    sieve(limit);

    int g = generator(M);

    i64 *pows = (i64*)malloc(M * sizeof(i64));
    pows[0] = 1;
    for (int i = 1; i < M; i++)
        pows[i] = pows[i-1] * g % M;

    int *logs = (int*)malloc(M * sizeof(int));
    for (int i = 0, gp = 1; i < M; i++) {
        logs[gp] = i;
        gp = gp * g % M;
    }

    i64 *S = (i64*)calloc(N + 2, sizeof(i64));
    for (int k = 1; k <= N + 1 && k - 1 < prime_count; k++) {
        int p = primes[k-1];
        if (p == M) S[k] = 0;
        else S[k] = pows[(i64)k % (M-1) * logs[p % M] % (M-1)];
    }

    int *S2 = (int*)calloc(N + 2, sizeof(int));
    for (int k = 1; k <= N + 1; k++)
        S2[k] = (int)(S[k] + S[k / D + 1]);

    // Initialize BIT
    bit = (int*)calloc(MAX_VAL, sizeof(int));

    // Add first K elements
    for (int i = 1; i <= K; i++)
        bit_update(S2[i] + 1, 1);  // +1 because BIT is 1-indexed and values start at 0

    double F = 0;
    for (int next = K + 1; next <= N + 1; next++) {
        // Find median (average of K/2 and K/2+1 smallest)
        int v1 = bit_find_kth(K / 2) - 1;
        int v2 = bit_find_kth(K / 2 + 1) - 1;
        F += (v1 + v2) / 2.0;

        // Remove old element
        int old_val = S2[next - K];
        bit_update(old_val + 1, -1);

        // Add new element
        int new_val = S2[next];
        bit_update(new_val + 1, 1);
    }

    printf("%.1f\n", F);

    free(ff); free(primes); free(pows); free(logs);
    free(S); free(S2); free(bit);

    return 0;
}
