/*
 * Project Euler 539 - Odd Elimination
 *
 * P(n) = last number remaining after alternately removing every other
 * from left then right. S(n) = sum_{k=1}^n P(k) mod M.
 *
 * Recursive formulas with memoization:
 * P(1) = 1
 * P(odd n) = P(n-1)
 * P(even n) = n + 2 - 2*P(n/2)
 *
 * S(1) = 1
 * S(even n) = P(n) + S(n-1)
 * S(odd n) = 1 + 2*(2*tr(n/2) + 2*(n/2) - 2*S(n/2))
 * where tr(n) = n*(n+1)/2
 */
#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define M 987654321LL

/* Hash map for memoization */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)

typedef struct { ll key; ll val; int occ; } HEntry;

static HEntry ht_P[HT_SIZE];
static HEntry ht_S[HT_SIZE];

static int ht_get(HEntry *ht, ll key, ll *val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HT_MASK;
        if (!ht[idx].occ) return 0;
        if (ht[idx].key == key) { *val = ht[idx].val; return 1; }
    }
    return 0;
}

static void ht_put(HEntry *ht, ll key, ll val) {
    ll h = (key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < 64; i++) {
        int idx = (h + i) & HT_MASK;
        if (!ht[idx].occ || ht[idx].key == key) {
            ht[idx].key = key;
            ht[idx].val = val;
            ht[idx].occ = 1;
            return;
        }
    }
    int idx = h & HT_MASK;
    ht[idx].key = key;
    ht[idx].val = val;
    ht[idx].occ = 1;
}

/* Modular inverse of 2 mod M */
static ll inv2;

static ll tr(ll n) {
    return (n % M) * ((n + 1) % M) % M * inv2 % M;
}

static ll P(ll n);
static ll S(ll n);

static ll P(ll n) {
    if (n <= 1) return n;
    ll val;
    if (ht_get(ht_P, n, &val)) return val;
    ll result;
    if (n % 2 == 1)
        result = P(n - 1);
    else
        result = n + 2 - 2 * P(n / 2);
    ht_put(ht_P, n, result);
    return result;
}

static ll S(ll n) {
    if (n <= 1) return n % M;
    ll val;
    if (ht_get(ht_S, n, &val)) return val;
    ll result;
    if (n % 2 == 0) {
        result = (P(n) % M + S(n - 1)) % M;
    } else {
        ll half = n / 2;
        result = (1 + 2 * (2 * tr(half) + 2 * (half % M) - 2 * S(half))) % M;
    }
    result = ((result % M) + M) % M;
    ht_put(ht_S, n, result);
    return result;
}

static ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    memset(ht_P, 0, sizeof(ht_P));
    memset(ht_S, 0, sizeof(ht_S));

    inv2 = mod_pow(2, M - 2, M);

    ll N = 1000000000000000000LL; /* 10^18 */
    ll result = S(N) % M;
    printf("%lld\n", result);
    return 0;
}
