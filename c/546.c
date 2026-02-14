/*
 * Project Euler Problem 546: Floor Function Recurrence.
 * Let f_k(n) = sum_{i=0}^{n} f_k(floor(i/k)). Find sum_{k=2}^{K} f_k(N).
 * N = 10^14, K = 10, mod 10^9+7.
 *
 * Uses recursive approach with coefficient tables that decompose
 * F[s](n) as a linear combination of F[t](floor(n/k) - t).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

#define MOD 1000000007LL

/* Hash table for memoization keyed by (n, k, s) */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)
#define HT_EMPTY (-1LL)

typedef struct {
    ll n;
    int k;
    int s;
    ll val;
    int used;
} HEntry;

static HEntry ht[1 << 20];

static void ht_init(void) {
    for (int i = 0; i < HT_SIZE; i++) ht[i].used = 0;
}

static unsigned int ht_hash(ll n, int k, int s) {
    unsigned long long h = (unsigned long long)n * 2654435761ULL;
    h ^= (unsigned long long)k * 40503ULL;
    h ^= (unsigned long long)s * 2246822519ULL;
    return (unsigned int)(h & HT_MASK);
}

static int ht_get(ll n, int k, int s, ll *val) {
    unsigned int idx = ht_hash(n, k, s);
    for (int i = 0; i < 1024; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!ht[pos].used) return 0;
        if (ht[pos].n == n && ht[pos].k == k && ht[pos].s == s) {
            *val = ht[pos].val;
            return 1;
        }
    }
    return 0;
}

static void ht_set(ll n, int k, int s, ll val) {
    unsigned int idx = ht_hash(n, k, s);
    for (int i = 0; i < 1024; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!ht[pos].used || (ht[pos].n == n && ht[pos].k == k && ht[pos].s == s)) {
            ht[pos].n = n;
            ht[pos].k = k;
            ht[pos].s = s;
            ht[pos].val = val;
            ht[pos].used = 1;
            return;
        }
    }
}

static ll imod(ll a, ll m) {
    return ((a % m) + m) % m;
}

/*
 * Compute f(n, k, s):
 * c[t][r] coefficients computed on the fly for each (k, s).
 * Since s grows with recursion depth (logarithmic), and k <= 10,
 * the coefficient tables are small.
 */
static ll f(ll n, int k, int s) {
    if (n == 0) return 1;
    if (n < 0) return 0;

    ll cached;
    if (ht_get(n, k, s, &cached)) return cached;

    /* Compute coefficients c[t][r] for 0 <= t <= s+1, 0 <= r < k */
    int max_t = s + 2;
    /* c[t][r]: use flat array, max_t * k entries */
    ll *c = (ll*)calloc(max_t * k, sizeof(ll));
    #define C(t, r) c[(t)*k + (r)]

    for (int r = 0; r < k; r++)
        C(0, r) = 1;

    for (int ss = 0; ss <= s; ss++) {
        for (int t = ss; t >= 0; t--) {
            for (int r = 1; r < k; r++) {
                C(t, r) = (C(t, r) + C(t, r - 1)) % MOD;
            }
            for (int r = 0; r < k; r++) {
                C(t + 1, r) = (C(t + 1, r) + C(t, k - 1)) % MOD;
            }
        }
    }

    ll result = 0;
    int r = (int)imod(n, k);
    for (int t = 0; t < max_t; t++) {
        ll sub_val = f(n / k - t, k, t);
        result = (result + (i128)C(t, r) * sub_val) % MOD;
    }

    #undef C
    free(c);

    ht_set(n, k, s, result);
    return result;
}

int main(void) {
    ll N = 100000000000000LL; /* 10^14 */
    int K = 10;

    ll ans = 0;
    for (int k = 2; k <= K; k++) {
        ht_init();
        ans = (ans + f(N, k, 0)) % MOD;
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
