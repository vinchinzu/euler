#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000000LL

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
}

static ll f(ll n);
static ll sum_sqrts(ll n);

static ll f(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get(ht_f, n, &val)) return val;

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

static ll sum_sqrts(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get(ht_ss, n, &val)) return val;

    ll fn = f(n);
    ll C = n - fn;
    ll l = isqrt_ll(C);

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

    // Test small values first
    for (ll N = 1; N <= 20; N++) {
        printf("T(%lld) = %lld\n", N, T(N));
    }
    
    // Try progressively larger
    for (ll N = 100; N <= 1000000000000000000LL; N *= 10) {
        memset(ht_f, 0, sizeof(ht_f));
        memset(ht_ss, 0, sizeof(ht_ss));
        memset(ht_T, 0, sizeof(ht_T));
        printf("T(%lld) = %lld\n", N, T(N));
    }

    return 0;
}
