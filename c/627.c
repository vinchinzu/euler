/*
 * Project Euler 627: Counting Products
 *
 * Find the number of integers expressible as a product of N=10001 integers
 * from 1..K=30, modulo 10^9+7.
 *
 * Uses Ehrhart polynomial approach: brute-force for small dilations,
 * then Lagrange interpolation to extrapolate.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base = ((base % mod) + mod) % mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Primes up to 30: 2,3,5,7,11,13,17,19,23,29 => L=10 */
#define L_PRIMES 10
#define K_VAL 30
#define N_VAL 10001

/* We need to enumerate all products of integers 1..30 iteratively.
 * The key observation: products of integers from 1..30 are determined by
 * their prime factorization using primes {2,3,5,7,11,13,17,19,23,29}.
 * An integer from 1..30 maps to a vector of exponents.
 *
 * For each "step n" we convolve the set of reachable exponent vectors
 * with the set of vectors from {1..30}. We track how many distinct
 * vectors are reachable.
 *
 * D = L - floor(sqrt(L)) = 10 - 3 = 7
 * We need F[0]..F[L-D] = F[0]..F[3] (4 values), by brute force with n=0..3.
 * Then interpolate polynomial of degree L=10 at x = N+D = 10008.
 */

/* Represent a product by its prime factorization as exponent vector.
 * Since we multiply at most a few small numbers, exponents stay small.
 * Use a hash set of exponent vectors. */

/* Actually, let's use a sorted array / set approach.
 * Each product is uniquely determined by (e2,e3,e5,e7,e11,e13,e17,e19,e23,e29).
 * For the brute force steps, max exponents after n multiplications are bounded.
 */

/* Hash-set approach: represent each vector as a single 64-bit hash.
 * Max exponent of 2 after 3 steps of multiplying by up to 30: 3*4=12 (since 16<=30).
 * Actually max e2 from single multiply: floor(log2(30))=4 (from 16).
 * After 3 steps: 3*4=12. Each exponent fits in 4 bits. 10 primes * 4 bits = 40 bits. */

typedef unsigned long long ull;

/* Encode exponent vector into 64-bit key */
/* primes = {2,3,5,7,11,13,17,19,23,29} */
static int plist[10] = {2,3,5,7,11,13,17,19,23,29};

/* Factor n (1..30) into exponent vector */
void factor30(int n, int exp[10]) {
    for (int i = 0; i < 10; i++) {
        exp[i] = 0;
        while (n % plist[i] == 0) {
            exp[i]++;
            n /= plist[i];
        }
    }
}

ull encode(int exp[10]) {
    ull key = 0;
    for (int i = 9; i >= 0; i--) {
        key = (key << 5) | (exp[i] & 0x1f);
    }
    return key;
}

/* Simple hash set */
#define HASH_SIZE (1 << 22) /* 4M buckets */
#define HASH_MASK (HASH_SIZE - 1)

typedef struct {
    ull *keys;
    int *used;
    int count;
} HashSet;

void hs_init(HashSet *hs) {
    hs->keys = (ull *)calloc(HASH_SIZE, sizeof(ull));
    hs->used = (int *)calloc(HASH_SIZE, sizeof(int));
    hs->count = 0;
}

void hs_free(HashSet *hs) {
    free(hs->keys);
    free(hs->used);
}

int hs_insert(HashSet *hs, ull key) {
    ull h = (key * 0x9E3779B97F4A7C15ULL) >> 42;
    h &= HASH_MASK;
    while (hs->used[h]) {
        if (hs->keys[h] == key) return 0; /* already present */
        h = (h + 1) & HASH_MASK;
    }
    hs->keys[h] = key;
    hs->used[h] = 1;
    hs->count++;
    return 1;
}

/* Get all keys from hash set */
void hs_get_all(HashSet *hs, ull **out, int *n) {
    *out = (ull *)malloc(hs->count * sizeof(ull));
    *n = 0;
    for (int i = 0; i < HASH_SIZE; i++) {
        if (hs->used[i]) {
            (*out)[(*n)++] = hs->keys[i];
        }
    }
}

int main() {
    int D = L_PRIMES - (int)(3); /* floor(sqrt(10)) = 3, so D = 7 */
    int num_bf = L_PRIMES + 1 - D; /* = 4, indices 0..3 */

    /* Precompute factor vectors for 1..30 */
    int fvecs[31][10];
    ull fkeys[31];
    for (int i = 1; i <= K_VAL; i++) {
        factor30(i, fvecs[i]);
        fkeys[i] = encode(fvecs[i]);
    }

    ll F[5]; /* F[0]..F[3] */
    F[0] = 1; /* only the empty product = 1 */

    HashSet cur, next;
    hs_init(&cur);
    int zero_vec[10] = {0};
    hs_insert(&cur, encode(zero_vec));

    for (int n = 1; n < num_bf; n++) {
        hs_init(&next);
        ull *all_keys;
        int nkeys;
        hs_get_all(&cur, &all_keys, &nkeys);

        for (int ki = 0; ki < nkeys; ki++) {
            ull base_key = all_keys[ki];
            /* Decode */
            int base_exp[10];
            ull tmp = base_key;
            for (int i = 0; i < 10; i++) {
                base_exp[i] = tmp & 0x1f;
                tmp >>= 5;
            }
            for (int f = 1; f <= K_VAL; f++) {
                int new_exp[10];
                for (int i = 0; i < 10; i++)
                    new_exp[i] = base_exp[i] + fvecs[f][i];
                hs_insert(&next, encode(new_exp));
            }
        }
        free(all_keys);
        hs_free(&cur);
        cur = next;
        F[n] = cur.count;
    }
    hs_free(&cur);

    /* Now build vals[0..L] for Lagrange interpolation */
    /* vals[i] = 0 for i < D, vals[i] = F[i-D] for i >= D */
    int n_pts = L_PRIMES + 1; /* 11 points, indices 0..10 */
    ll vals[11];
    for (int i = 0; i < n_pts; i++) {
        if (i < D) vals[i] = 0;
        else vals[i] = F[i - D] % MOD;
    }

    /* Lagrange interpolation at x = N_VAL + D = 10008 */
    ll x = N_VAL + D;

    ll prefix[12], suffix[12];
    prefix[0] = 1;
    for (int j = 0; j < n_pts; j++)
        prefix[j+1] = (lll)prefix[j] * ((x - j) % MOD + MOD) % MOD;

    suffix[n_pts] = 1;
    for (int j = n_pts - 1; j >= 0; j--)
        suffix[j] = (lll)suffix[j+1] * ((x - j) % MOD + MOD) % MOD;

    ll fact[12], inv_fact[12];
    fact[0] = 1;
    for (int i = 1; i < n_pts; i++)
        fact[i] = (lll)fact[i-1] * i % MOD;
    inv_fact[n_pts-1] = powmod(fact[n_pts-1], MOD-2, MOD);
    for (int i = n_pts-2; i >= 0; i--)
        inv_fact[i] = (lll)inv_fact[i+1] * (i+1) % MOD;

    ll ans = 0;
    for (int i = 0; i < n_pts; i++) {
        if (vals[i] == 0) continue;
        ll numer = (lll)prefix[i] * suffix[i+1] % MOD;
        ll denom_inv = (lll)inv_fact[i] * inv_fact[n_pts - 1 - i] % MOD;
        if ((n_pts - 1 - i) % 2 == 1)
            denom_inv = (MOD - denom_inv) % MOD;
        ans = (ans + (lll)vals[i] * numer % MOD * denom_inv) % MOD;
    }

    printf("%lld\n", ans % MOD);
    return 0;
}
