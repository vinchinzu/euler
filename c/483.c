/* Project Euler 483 - Repeated permutation
 * Extracted from embedded C in python/483.py
 * Average f^2(P) over all permutations of {1..350}
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAXN 351
#define MAX_PRIMES 100

int primes[MAX_PRIMES];
int nprimes = 0;
int largest_pf[MAXN];
double ffact[MAXN];

/* Simple hash map */
#define HSIZE (1 << 23)
#define HMASK (HSIZE - 1)

typedef struct HNode {
    int maxIndex, min_k, n;
    long long lcm;
    double value;
    struct HNode *next;
} HNode;

HNode *htable[HSIZE];
HNode *pool;
int pool_idx = 0;
int pool_cap = 0;

HNode* alloc_node() {
    if (pool_idx >= pool_cap) {
        pool_cap += 1000000;
        pool = (HNode*)realloc(pool, pool_cap * sizeof(HNode));
    }
    return &pool[pool_idx++];
}

unsigned hash4(int a, int b, int c, long long d) {
    unsigned h = (unsigned)a * 2654435761u;
    h ^= (unsigned)b * 2246822519u;
    h ^= (unsigned)c * 3266489917u;
    h ^= (unsigned)(d ^ (d >> 32)) * 668265263u;
    return h & HMASK;
}

int cache_lookup(int mi, int mk, int n, long long lcm, double *val) {
    unsigned h = hash4(mi, mk, n, lcm);
    for (HNode *p = htable[h]; p; p = p->next) {
        if (p->maxIndex == mi && p->min_k == mk && p->n == n && p->lcm == lcm) {
            *val = p->value;
            return 1;
        }
    }
    return 0;
}

void cache_store(int mi, int mk, int n, long long lcm, double val) {
    unsigned h = hash4(mi, mk, n, lcm);
    HNode *nd = alloc_node();
    nd->maxIndex = mi; nd->min_k = mk; nd->n = n; nd->lcm = lcm;
    nd->value = val; nd->next = htable[h];
    htable[h] = nd;
}

long long gcd_ll(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

long long lcm_ll(long long a, long long b) {
    return a / gcd_ll(a, b) * b;
}

double sumF2(int maxIndex, int min_k, int n, long long lcm) {
    double cached;
    if (cache_lookup(maxIndex, min_k, n, lcm, &cached))
        return cached;

    long long relevantLcm = lcm;
    long long scale = 1;
    double result = (double)lcm * (double)lcm / ffact[n];

    for (int index = maxIndex; index >= 0; index--) {
        int p = primes[index];
        int start_k = (index == maxIndex) ? min_k : 1;
        for (int k = start_k; k * p <= n; k++) {
            if (largest_pf[k] > p) continue;
            int cycle_len = k * p;
            long long new_lcm = lcm_ll(relevantLcm, (long long)cycle_len);
            double pow_cl = (double)cycle_len;
            for (int mult = 1; mult * cycle_len <= n; mult++) {
                double sub = sumF2(index, k + 1, n - mult * cycle_len, new_lcm);
                result += sub * (double)scale * (double)scale
                         / pow_cl / ffact[mult];
                pow_cl *= (double)cycle_len;
            }
        }
        while (relevantLcm % p == 0) {
            relevantLcm /= p;
            scale *= p;
        }
    }

    cache_store(maxIndex, min_k, n, lcm, result);
    return result;
}

void sieve_primes(int limit) {
    char is_prime[MAXN];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i * i <= limit; i++)
        if (is_prime[i])
            for (int j = i*i; j <= limit; j += i)
                is_prime[j] = 0;
    for (int i = 2; i <= limit; i++)
        if (is_prime[i])
            primes[nprimes++] = i;
}

int main() {
    int N = 350;
    sieve_primes(N);

    memset(largest_pf, 0, sizeof(largest_pf));
    for (int pi = 0; pi < nprimes; pi++)
        for (int i = primes[pi]; i <= N; i += primes[pi])
            largest_pf[i] = primes[pi];

    ffact[0] = 1.0;
    for (int i = 1; i <= N; i++)
        ffact[i] = ffact[i-1] * i;

    memset(htable, 0, sizeof(htable));
    pool = (HNode*)malloc(1000000 * sizeof(HNode));
    pool_cap = 1000000;

    double ans = sumF2(nprimes - 1, 1, N, 1);
    /* Format to match expected: 4.993401567e22 */
    /* Format to match expected: 4.993401567e22 (no '+' in exponent) */
    char buf[64];
    snprintf(buf, sizeof(buf), "%.9e", ans);
    /* Remove '+' from exponent */
    char *p = buf;
    char *out = buf;
    while (*p) {
        if (*p != '+') *out++ = *p;
        p++;
    }
    *out = '\0';
    printf("%s\n", buf);

    free(pool);
    return 0;
}
