#include <stdio.h>
#include <stdint.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000000LL
#define HT_SIZE (1 << 22)
#define HT_MASK (HT_SIZE - 1)

typedef struct { ll key; lll val; int occupied; } HEntry;
typedef struct { ll key; ll val; int occupied; } HEntry_ll;

static HEntry ht_ss[HT_SIZE];
static HEntry_ll ht_f[HT_SIZE];
static HEntry_ll ht_T[HT_SIZE];

static int ht_get_lll(HEntry *ht, ll key, lll *val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 128; i++) {
        int idx = (h + i) & HT_MASK;
        if (!ht[idx].occupied) return 0;
        if (ht[idx].key == key) { *val = ht[idx].val; return 1; }
    }
    return 0;
}

static void ht_put_lll(HEntry *ht, ll key, lll val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 128; i++) {
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

static int ht_get_ll(HEntry_ll *ht, ll key, ll *val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 128; i++) {
        int idx = (h + i) & HT_MASK;
        if (!ht[idx].occupied) return 0;
        if (ht[idx].key == key) { *val = ht[idx].val; return 1; }
    }
    return 0;
}

static void ht_put_ll(HEntry_ll *ht, ll key, ll val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 128; i++) {
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
    while (x > 0 && x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

/* Triangular number n*(n+1)/2 mod MOD */
static ll tr(ll n) {
    ll a = n % MOD;
    ll b = (n + 1) % MOD;
    if (a < 0) a += MOD;
    if (b < 0) b += MOD;
    if (a % 2 == 0) {
        return (a / 2) % MOD * (b % MOD) % MOD;
    } else {
        return (a % MOD) * ((b / 2) % MOD) % MOD;
    }
}

static ll f(ll n);
static lll sum_sqrts(ll n);

static ll f(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get_ll(ht_f, n, &val)) return val;

    ll lo = 0, hi = n;
    while (lo + 1 < hi) {
        ll mid = lo + (hi - lo) / 2;
        lll ss = sum_sqrts(mid);
        if (ss + mid <= n)
            lo = mid;
        else
            hi = mid;
    }

    ht_put_ll(ht_f, n, lo);
    return lo;
}

static lll sum_sqrts(ll n) {
    if (n == 0) return 0;
    lll val;
    if (ht_get_lll(ht_ss, n, &val)) return val;

    ll fn = f(n);
    ll C = n - fn;
    ll l = isqrt_ll(C);

    lll res = sum_sqrts(fn)
           + (lll)(C - l*l + 1) * l
           + 2 * (lll)(l-1) * l * (2*(l-1)+1) / 6
           + (lll)(l-1) * l / 2;

    ht_put_lll(ht_ss, n, res);
    return res;
}

static ll T(ll n) {
    if (n == 0) return 0;
    ll val;
    if (ht_get_ll(ht_T, n, &val)) return val;

    ll fn = f(n);
    ll res = (T(fn) + tr(n - fn)) % MOD;

    ht_put_ll(ht_T, n, res);
    return res;
}

int main(void) {
    memset(ht_f, 0, sizeof(ht_f));
    memset(ht_ss, 0, sizeof(ht_ss));
    memset(ht_T, 0, sizeof(ht_T));

    for (ll N = 1; N <= 20; N++) {
        printf("T(%lld) = %lld\n", N, T(N));
    }

    memset(ht_f, 0, sizeof(ht_f));
    memset(ht_ss, 0, sizeof(ht_ss));
    memset(ht_T, 0, sizeof(ht_T));

    ll N = 1000000000000000000LL;
    ll result = T(N);
    printf("T(10^18) = %lld\n", result);
    return 0;
}
