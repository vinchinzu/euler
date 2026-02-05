"""Project Euler Problem 512: Sums of totients of powers.

Let f(n) = Sum_{i=1}^n phi(n^i) (mod n+1). Find Sum_{i=1}^n f(i).

f(n) = phi(n) * (1 - (-1)^n) / 2 = phi(n) if n is odd, 0 if n is even.
So we need h(1) = sum of phi(n) for odd n <= N.

Using: sum_{i=1}^{N/k} phi(i) = h(k) + sum_{e=1,2,4,...} e * h(2ek)
=> h(k) = sum_{i=1}^{N/k} phi(i) - sum_{e=1,2,4,...} e * h(2ek)
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

#define N 500000000LL

int sqrtN;
ll *small_sum;  // sum_phi[i] = sum_{j=1}^i phi(j) for i <= sqrtN
ll *large_sum;  // large_sum[k] = sum_{j=1}^{N/k} phi(j) for k <= sqrtN

// Sieve phi values up to sqrtN
int *phi;

void compute_phi_sieve() {
    phi = (int*)malloc((sqrtN + 1) * sizeof(int));
    for (int i = 0; i <= sqrtN; i++) phi[i] = i;

    for (int i = 2; i <= sqrtN; i++) {
        if (phi[i] == i) {  // i is prime
            for (int j = i; j <= sqrtN; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }
}

// Lucy algorithm to compute sum of phi up to N
void lucy_phi() {
    small_sum = (ll*)calloc(sqrtN + 1, sizeof(ll));
    large_sum = (ll*)calloc(sqrtN + 1, sizeof(ll));

    // Initialize with sum_{i=1}^m i = m(m+1)/2 (pretending all numbers are coprime)
    for (int i = 1; i <= sqrtN; i++) {
        small_sum[i] = (ll)i * (i + 1) / 2 - 1;  // subtract phi(1) = 1? No, keep it
    }
    for (int k = 1; k <= sqrtN; k++) {
        ll m = N / k;
        large_sum[k] = m * (m + 1) / 2 - 1;
    }

    // Actually, sum of phi is computed differently. Let me use the standard formula.
    // sum_{i=1}^n phi(i) = (1 + sum_{d=2}^n mu(d) * (n/d)*(n/d+1)/2)
    // But Lucy's algorithm for phi is complex.

    // Simpler approach: precompute phi sieve up to sqrtN, then use recursion
    // sum_{i=1}^n phi(i) = n(n+1)/2 - sum_{d=2}^n sum_{i=1}^{n/d} phi(i)

    // Let's use the relation: sum_{d|n} phi(d) = n
    // => sum_{i=1}^n phi(i) = sum_{i=1}^n (sum_{d|i} phi(d) - sum_{d|i,d<i} phi(d))
    //                       = sum_{i=1}^n i - sum_{d=2}^n sum_{i: d|i, i<=n} phi(i/d)
    //                       = n(n+1)/2 - sum_{d=2}^n S(n/d)
    // where S(m) = sum_{i=1}^m phi(i)

    // This gives a recursive formula that can be computed efficiently.

    // For now, let's just use precomputed phi values and a map for larger sums.
}

// Global map for memoization of sum_phi
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

// Compute sum of phi(i) for i = 1 to m
ll sum_phi(ll m) {
    if (m <= 0) return 0;
    if (m <= sqrtN) {
        // Use precomputed small_sum
        return small_sum[m];
    }

    int found;
    ll cached = hash_get(m, &found);
    if (found) return cached;

    // Use formula: S(m) = m(m+1)/2 - sum_{d=2}^sqrt(m) (S(m/d) + S(d-1) - S(d-1)) ... no

    // Correct formula:
    // S(n) = n(n+1)/2 - sum_{d=2}^n S(n/d)
    // Using hyperbola trick: sum_{d=2}^n S(n/d) = sum_{d=2}^{sqrt(n)} S(n/d) + sum_{q=1}^{n/d-1} (...)

    // Actually simpler: just iterate over d from 2 to sqrt(m), and use that n/d takes O(sqrt(n)) distinct values

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

// Memoization for h function
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

    // Compute cumulative sums of phi for small values
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
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)


if __name__ == "__main__":
    print(solve())
