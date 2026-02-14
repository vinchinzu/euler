/*
 * Project Euler Problem 512: Sums of totients of powers.
 * Sum of phi(n) for odd n <= N, using Lucy DP + recursive h function.
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

#define N 500000000LL

int sqrtN;
ll *small_sum;
int *phi;

void compute_phi_sieve() {
    phi = (int*)malloc((sqrtN + 1) * sizeof(int));
    for (int i = 0; i <= sqrtN; i++) phi[i] = i;

    for (int i = 2; i <= sqrtN; i++) {
        if (phi[i] == i) {
            for (int j = i; j <= sqrtN; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }
}

#define HASH_SIZE 1000003
typedef struct {
    ll key;
    ll value;
    int used;
} HashEntry;

HashEntry *hash_table;

void hash_init() {
    hash_table = (HashEntry*)calloc(HASH_SIZE, sizeof(HashEntry));
}

ll hash_get(ll key, int *found) {
    ll idx = (ull)key % HASH_SIZE;
    for (int i = 0; i < HASH_SIZE; i++) {
        ll pos = (idx + i) % HASH_SIZE;
        if (!hash_table[pos].used) {
            *found = 0;
            return 0;
        }
        if (hash_table[pos].key == key) {
            *found = 1;
            return hash_table[pos].value;
        }
    }
    *found = 0;
    return 0;
}

void hash_put(ll key, ll value) {
    ll idx = (ull)key % HASH_SIZE;
    for (int i = 0; i < HASH_SIZE; i++) {
        ll pos = (idx + i) % HASH_SIZE;
        if (!hash_table[pos].used || hash_table[pos].key == key) {
            hash_table[pos].key = key;
            hash_table[pos].value = value;
            hash_table[pos].used = 1;
            return;
        }
    }
}

ll sum_phi(ll m) {
    if (m <= 0) return 0;
    if (m <= sqrtN) {
        return small_sum[m];
    }

    int found;
    ll cached = hash_get(m, &found);
    if (found) return cached;

    ll result = m * (m + 1) / 2;
    ll d = 2;
    while (d <= m) {
        ll q = m / d;
        ll d_next = m / q + 1;
        result -= (d_next - d) * sum_phi(q);
        d = d_next;
    }

    hash_put(m, result);
    return result;
}

HashEntry *h_cache;

void h_cache_init() {
    h_cache = (HashEntry*)calloc(HASH_SIZE, sizeof(HashEntry));
}

ll h_get(ll key, int *found) {
    ll idx = (ull)key % HASH_SIZE;
    for (int i = 0; i < HASH_SIZE; i++) {
        ll pos = (idx + i) % HASH_SIZE;
        if (!h_cache[pos].used) {
            *found = 0;
            return 0;
        }
        if (h_cache[pos].key == key) {
            *found = 1;
            return h_cache[pos].value;
        }
    }
    *found = 0;
    return 0;
}

void h_put(ll key, ll value) {
    ll idx = (ull)key % HASH_SIZE;
    for (int i = 0; i < HASH_SIZE; i++) {
        ll pos = (idx + i) % HASH_SIZE;
        if (!h_cache[pos].used || h_cache[pos].key == key) {
            h_cache[pos].key = key;
            h_cache[pos].value = value;
            h_cache[pos].used = 1;
            return;
        }
    }
}

ll h(ll k) {
    if (k > N) return 0;

    int found;
    ll cached = h_get(k, &found);
    if (found) return cached;

    ll m = N / k;
    ll result = sum_phi(m);

    for (ll e = 1; 2 * k * e <= N; e *= 2) {
        result -= e * h(2 * e * k);
    }

    h_put(k, result);
    return result;
}

int main() {
    sqrtN = (int)sqrt((double)N) + 1;

    compute_phi_sieve();

    small_sum = (ll*)malloc((sqrtN + 1) * sizeof(ll));
    small_sum[0] = 0;
    for (int i = 1; i <= sqrtN; i++) {
        small_sum[i] = small_sum[i - 1] + phi[i];
    }

    hash_init();
    h_cache_init();

    ll answer = h(1);
    printf("%lld\n", answer);

    free(phi);
    free(small_sum);
    free(hash_table);
    free(h_cache);

    return 0;
}
