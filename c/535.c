/*
 * Project Euler 535 - Fractal Sequence
 *
 * Define sequence S with fractal property: circled numbers are consecutive,
 * non-circled numbers form S itself, each non-circled a_i has floor(sqrt(a_i))
 * circled numbers before it. Find T = sum_{i=1}^N S_i mod 10^9.
 *
 * Recursive approach with memoization using hash map.
 */
#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000000LL

/* Hash map for memoization */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)

typedef struct { ll key; ll val; int occupied; } HEntry;

static HEntry ht_f[HT_SIZE];
static HEntry ht_ss[HT_SIZE];
static HEntry ht_T[HT_SIZE];

static int ht_get(HEntry *ht, ll key, ll *val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HT_MASK;
        if (!ht[idx].occupied) return 0;
        if (ht[idx].key == key) { *val = ht[idx].val; return 1; }
    }
    return 0;
}

static void ht_put(HEntry *ht, ll key, ll val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HT_MASK;
        if (!ht[idx].occupied || ht[idx].key == key) {
            ht[idx].key = key;
            ht[idx].val = val;
            ht[idx].occupied = 1;
            return;
        }
    }
    /* Fallback */
    int idx = h & HT_MASK;
    ht[idx].key = key;
    ht[idx].val = val;
    ht[idx].occupied = 1;
}

static ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)sqrt((double)n);
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

static ll sq(ll n) { return n * n; }

static ll sum_powers_1(ll n) {
    if (n <= 0) return 0;
    return n * (n + 1) / 2;
}

static ll sum_powers_2(ll n) {
    if (n <= 0) return 0;
    return n * (n + 1) * (2 * n + 1) / 6;
}

static ll tr(ll n) {
    return (n % MOD) * ((n + 1) % MOD) % MOD * 500000000LL % MOD;
    /* 500000000 = modular inverse of 2 mod 10^9 */
}

/* Forward declarations for mutual recursion */
static ll f(ll n);
static ll sum_sqrts(ll n);

/*
 * f(n): number of non-circled numbers in first n terms.
 * The number of circled values up to position n equals n - f(n),
 * and the largest circled value is n - f(n).
 * Each non-circled value a has floor(sqrt(a)) circled values before it.
 * We need: the total number of values (circled + non-circled) that fit
 * in positions 1..n. Circled: consecutive 1..C. Non-circled: f(n) values
 * from S, which contribute sum_of_floor_sqrts of first f(n) terms' values.
 *
 * sum_sqrts(m) + m = n means m = f(n), where sum_sqrts(m) = sum of
 * floor(sqrt(S_i)) for i=1..m.
 */
static ll f(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get(ht_f, n, &val)) return val;

    /* Binary search: find largest m such that sum_sqrts(m) + m <= n */
    ll lo = 0, hi = n;
    while (lo + 1 < hi) {
        ll mid = lo + (hi - lo) / 2;
        if (sum_sqrts(mid) + mid <= n)
            lo = mid;
        else
            hi = mid;
    }

    ht_put(ht_f, n, lo);
    return lo;
}

/*
 * sum_sqrts(n): sum of floor(sqrt(S_i)) for i=1..n.
 * Using the fractal property, the sequence S consists of:
 *   - circled values: 1, 2, ..., C where C = n - f(n)
 *   - non-circled values: S_1, S_2, ..., S_{f(n)}
 * So sum_sqrts(n) = sum_{v=1}^{C} floor(sqrt(v)) + sum_sqrts(f(n))
 * For sum_{v=1}^{C} floor(sqrt(v)), use the formula involving
 * partitioning by floor(sqrt(v)) = k.
 */
static ll sum_sqrts(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get(ht_ss, n, &val)) return val;

    ll fn = f(n);
    ll C = n - fn;
    ll l = isqrt_ll(C);

    /* sum_{v=1}^{C} floor(sqrt(v))
     * = sum_{k=1}^{l-1} k * ((k+1)^2 - k^2) + l * (C - l^2 + 1)
     * = sum_{k=1}^{l-1} k * (2k+1) + l * (C - l^2 + 1)
     * = 2*sum(k^2, 1..l-1) + sum(k, 1..l-1) + l*(C - l^2 + 1) */
    ll res = sum_sqrts(fn)
           + (C - sq(l) + 1) * l
           + 2 * sum_powers_2(l - 1)
           + sum_powers_1(l - 1);

    ht_put(ht_ss, n, res);
    return res;
}

static ll T(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get(ht_T, n, &val)) return val;

    ll fn = f(n);
    ll res = (T(fn) + tr(n - fn)) % MOD;

    ht_put(ht_T, n, res);
    return res;
}

int main(void) {
    memset(ht_f, 0, sizeof(ht_f));
    memset(ht_ss, 0, sizeof(ht_ss));
    memset(ht_T, 0, sizeof(ht_T));

    ll N = 1000000000000000000LL; /* 10^18 */
    ll result = T(N);
    printf("%lld\n", result);
    return 0;
}
