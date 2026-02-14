/*
 * Project Euler 829: Integral Fusion
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>
#include <limits.h>

typedef unsigned long long ull;
typedef __uint128_t u128;

static const int NN = 31;
static const int PRIMES[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31};
static const int NPRIMES = 11;
static ull POWS[11][64];

typedef struct {
    ull key;
    char *s;
    int len;
} Entry;

static Entry *CACHE = NULL;
static size_t CACHE_SIZE = 1u << 21;

static inline ull hash64(ull x) {
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccdULL;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53ULL;
    x ^= x >> 33;
    return x;
}

static Entry *cache_find_slot(ull key) {
    size_t mask = CACHE_SIZE - 1;
    size_t idx = (size_t)(hash64(key) & mask);
    while (CACHE[idx].key != 0 && CACHE[idx].key != key)
        idx = (idx + 1) & mask;
    return &CACHE[idx];
}

static inline int is_small_prime(ull n) {
    for (int i = 0; i < NPRIMES; i++)
        if ((ull)PRIMES[i] == n)
            return 1;
    return 0;
}

static inline ull isqrt_ull(ull n) {
    long double x = sqrtl((long double)n);
    ull r = (ull)x;
    while ((u128)(r + 1) * (r + 1) <= n) r++;
    while ((u128)r * r > n) r--;
    return r;
}

static int factorize(ull n, ull *ps, int *es) {
    int nf = 0;
    for (int i = 0; i < NPRIMES; i++) {
        ull p = (ull)PRIMES[i];
        if ((u128)p * p > n)
            break;
        if (n % p == 0) {
            int e = 0;
            do {
                n /= p;
                e++;
            } while (n % p == 0);
            ps[nf] = p;
            es[nf] = e;
            nf++;
        }
    }
    if (n > 1) {
        ps[nf] = n;
        es[nf] = 1;
        nf++;
    }
    return nf;
}

static ull best_d_sqrt_n;
static ull best_d_ans;
static ull best_ps[64];
static int best_es[64];

static void best_d_dfs(int idx, int nf, ull cur) {
    if (idx == nf) {
        if (cur <= best_d_sqrt_n && cur > best_d_ans)
            best_d_ans = cur;
        return;
    }

    ull p = best_ps[idx];
    int e = best_es[idx];
    ull mul = 1;
    for (int i = 0; i <= e; i++) {
        u128 next = (u128)cur * mul;
        if (next > best_d_sqrt_n)
            break;
        best_d_dfs(idx + 1, nf, (ull)next);
        if (i < e) {
            if ((u128)mul * p > best_d_sqrt_n)
                break;
            mul *= p;
        }
    }
}

static ull best_divisor(ull n) {
    ull ps[64];
    int es[64];
    int nf = factorize(n, ps, es);

    best_d_sqrt_n = isqrt_ull(n);
    best_d_ans = 1;
    for (int i = 0; i < nf; i++) {
        best_ps[i] = ps[i];
        best_es[i] = es[i];
    }
    best_d_dfs(0, nf, 1);
    return best_d_ans;
}

static Entry *shape_of(ull n) {
    Entry *slot = cache_find_slot(n);
    if (slot->key == n)
        return slot;

    slot->key = n;

    if (is_small_prime(n)) {
        slot->len = 1;
        slot->s = (char *)malloc(2);
        slot->s[0] = '.';
        slot->s[1] = '\0';
        return slot;
    }

    ull d = best_divisor(n);
    Entry *left = shape_of(d);
    Entry *right = shape_of(n / d);

    slot->len = left->len + right->len + 2;
    slot->s = (char *)malloc((size_t)slot->len + 1);
    slot->s[0] = '(';
    memcpy(slot->s + 1, left->s, (size_t)left->len);
    memcpy(slot->s + 1 + left->len, right->s, (size_t)right->len);
    slot->s[slot->len - 1] = ')';
    slot->s[slot->len] = '\0';
    return slot;
}

static int count_prime_factors_with_multiplicity(ull n) {
    int total = 0;
    for (int i = 0; i < NPRIMES; i++) {
        ull p = (ull)PRIMES[i];
        while (n % p == 0) {
            n /= p;
            total++;
        }
    }
    if (n > 1)
        total++;
    return total;
}

static ull best_res;
static const char *target_s;
static int target_len;

static void search(int k, int min_pi, ull cur) {
    if (k == 0) {
        Entry *s = shape_of(cur);
        if (s->len == target_len && strcmp(s->s, target_s) == 0 && cur < best_res)
            best_res = cur;
        return;
    }

    for (int pi = min_pi; pi < NPRIMES; pi++) {
        if ((u128)cur * POWS[pi][k] > best_res)
            break;
        search(k - 1, pi, cur * (ull)PRIMES[pi]);
    }
}

static ull M_of_n(int n) {
    ull ndf = 1;
    for (int i = n; i > 0; i -= 2)
        ndf *= (ull)i;

    int k = count_prime_factors_with_multiplicity(ndf);
    Entry *target = shape_of(ndf);

    best_res = ndf;
    target_s = target->s;
    target_len = target->len;
    search(k, 0, 1);
    return best_res;
}

int main(void) {
    CACHE = (Entry *)calloc(CACHE_SIZE, sizeof(Entry));
    if (!CACHE) {
        fprintf(stderr, "cache allocation failed\n");
        return 1;
    }

    for (int i = 0; i < NPRIMES; i++) {
        ull p = (ull)PRIMES[i];
        POWS[i][0] = 1;
        for (int e = 1; e < 64; e++) {
            u128 v = (u128)POWS[i][e - 1] * p;
            POWS[i][e] = (v > ULLONG_MAX) ? ULLONG_MAX : (ull)v;
        }
    }

    ull ans = 0;
    for (int n = 2; n <= NN; n++)
        ans += M_of_n(n);

    printf("%llu\n", ans);
    return 0;
}
