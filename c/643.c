/*
 * Project Euler 643 - 2-Friendly
 * Count pairs 1 <= p < q <= N with gcd(p,q) a power of 2.
 * Uses sub-linear sum of Euler's totient with sieve up to n^{2/3}.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;

#define MOD 1000000007LL

static ll power_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

/* Hash table for large[i] = S(N/i) for i <= limit */
#define HT_SIZE (1 << 18)
#define HT_MASK (HT_SIZE - 1)

typedef struct {
    ll key;
    ll val;
    int used;
} HTEntry;

static HTEntry ht[HT_SIZE];

static void ht_set(ll key, ll val) {
    int h = (int)((ull)key * 2654435761ULL >> 14) & HT_MASK;
    while (ht[h].used && ht[h].key != key)
        h = (h + 1) & HT_MASK;
    ht[h].key = key;
    ht[h].val = val;
    ht[h].used = 1;
}

static ll ht_get(ll key) {
    int h = (int)((ull)key * 2654435761ULL >> 14) & HT_MASK;
    while (ht[h].used) {
        if (ht[h].key == key) return ht[h].val;
        h = (h + 1) & HT_MASK;
    }
    return -1; /* not found */
}

int main(void) {
    ll N_val = 100000000000LL; /* 10^11 */

    /* Sieve phi up to V ~ N^{2/3} */
    int V = (int)(pow((double)N_val, 2.0/3.0)) + 1;
    if (V < 100) V = 100;

    int *phi = (int *)malloc((V + 1) * sizeof(int));
    for (int i = 0; i <= V; i++) phi[i] = i;
    for (int i = 2; i <= V; i++) {
        if (phi[i] == i) { /* prime */
            for (int j = i; j <= V; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }

    /* Prefix sums mod MOD */
    ll *small = (ll *)calloc(V + 1, sizeof(ll));
    for (int i = 1; i <= V; i++) {
        small[i] = (small[i - 1] + phi[i]) % MOD;
    }
    free(phi);

    ll inv2 = power_mod(2, MOD - 2, MOD);

    /* Compute S(m) for m > V using recursive formula */
    /* m = N/i for i from limit down to 1 */
    int limit = (int)(N_val / ((ll)V + 1)) + 1;

    memset(ht, 0, sizeof(ht));

    for (int i = limit; i >= 1; i--) {
        ll m = N_val / i;
        if (m <= V) continue;

        ll result = (m % MOD) * ((m + 1) % MOD) % MOD * inv2 % MOD;

        ll d = 2;
        while (d <= m) {
            ll q = m / d;
            ll d_max = m / q;
            ll s_q;
            if (q <= V) {
                s_q = small[q];
            } else {
                ll idx = N_val / q;
                s_q = ht_get(idx);
            }
            result = (result - ((d_max - d + 1) % MOD + MOD) % MOD * s_q % MOD + MOD) % MOD;
            d = d_max + 1;
        }

        ht_set(i, result % MOD);
    }

    /* S(m) function */
    /* Now compute the answer */
    ll ans = 0;
    int t = 1;
    while ((1LL << t) <= N_val) {
        ll lim = N_val >> t;
        ll s_lim;
        if (lim <= V) {
            s_lim = small[lim];
        } else {
            ll idx = N_val / lim;
            s_lim = ht_get(idx);
        }
        ans = (ans + s_lim - 1 + MOD) % MOD;
        t++;
    }

    printf("%lld\n", ans % MOD);

    free(small);
    return 0;
}
